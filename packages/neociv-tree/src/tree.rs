use std::{cell::RefCell, collections::HashMap};

use petgraph::graph::DiGraph;

use crate::node::Node;

pub struct NeocivTree {
    id: String,
    graph: DiGraph<String, bool>,
    map: HashMap<String, Node>,
    unlinked_deps: Vec<(String, String)>,
    pool: u32,
    target: Option<String>,
}

impl NeocivTree {
    pub fn new(id: String) -> NeocivTree {
        Self {
            id,
            graph: DiGraph::new(),
            map: HashMap::new(),
            unlinked_deps: Vec::new(),
            pool: 0,
            target: None,
        }
    }

    pub fn contains(&self, id: &String) -> bool {
        self.map.contains_key(id)
    }

    pub fn contains_pair(&self, a: &String, b: &String) -> bool {
        self.contains(a) && self.contains(b)
    }

    pub fn get(&self, id: &String) -> Result<&Node, String> {
        if self.contains(id) {
            Ok(self.map.get(id).unwrap())
        } else {
            Err(format!(
                "Unable to find node '{}' in tree '{}'",
                id, self.id
            ))
        }
    }

    pub fn get_mut(&mut self, id: &String) -> Result<&mut Node, String> {
        if self.contains(id) {
            Ok(self.map.get_mut(id).unwrap())
        } else {
            Err(format!(
                "Unable to find node '{}' in tree '{}'",
                id, self.id
            ))
        }
    }

    /*
    pub fn modify_node<F>(&mut self, id: &String, func: F) where F {

    }
    */

    pub fn add(
        &mut self,
        id: String,
        cost: Option<u32>,
        total: Option<u32>,
        col: Option<u16>,
        row: Option<u8>,
        deps: Option<Vec<String>>,
    ) -> Result<&mut Self, String> {
        // Some values need to be pulled out and finalised as they potentially reflect on each other
        let final_deps = deps.unwrap_or(Vec::new());
        let final_col = col.unwrap_or_else(|| self.default_column(&final_deps));
        let final_row = row.unwrap_or_else(|| self.default_row(final_col));

        // Add the node with sane defaults
        self.add_node(Node {
            id,
            cost: cost.unwrap_or(0),
            total: total.unwrap_or(0),
            col: final_col,
            row: final_row,
            deps: final_deps,
            linked_deps: Vec::new(),
            linked_dependants: Vec::new(),
            index: None,
        })
    }

    fn add_node(&mut self, node: Node) -> Result<&mut Self, String> {
        if self.contains(&node.id) {
            Err(format!(
                "Node '{}' already exists in tree '{}'",
                node.id, self.id
            ))
        } else if node.deps.len() > 0 && self.furthest_dep_col(&node.deps) >= node.col {
            Err(format!("Node '{}' cannot be inserted into tree '{}', its column is less than or equal to a dependency", node.id, self.id))
        } else {
            // Insert into the graph and thus get the index of the node
            let node_index = self.graph.add_node(node.id.clone());
            // Insert a new node into the map and include the index
            self.map.insert(
                node.id.clone(),
                Node {
                    index: Some(node_index),
                    ..node.clone()
                },
            );
            // Link the dependencies and get the self reference back as mutable
            self.link_node_deps(&node.id)?;

            // If all is well return self
            Ok(self)
        }
    }

    pub fn remove_node(&mut self, id: &String) -> Result<&mut Self, String> {
        panic!("Unimplemented");
        // TODO: Remove deps *but* if one end of the dep exists put it back into the unlinked_deps cache
        Ok(self)
    }

    pub fn set_target(&mut self, id: Option<&String>) -> Result<&mut Self, String> {
        panic!("Unimplemented");
        Ok(self)
    }

    /// Applies the progression amount to the tree and the current target (if any).
    pub fn apply(&mut self, amount: u32) -> Result<&mut Self, String> {
        // Set the pool value
        self.pool += {
            if self.target.is_none() {
                amount
            } else if self.target.as_ref().is_some_and(|id| self.contains(&id)) {
                0
            } else {
                0
            }
        };
        // TODO: Draw from the pool to apply to the target nodes
        Ok(self)
    }

    fn link_node_deps(&mut self, id: &String) -> Result<&mut Self, String> {
        // Create the list of deps as tuples
        let deps: Vec<(String, String)> = {
            // Get a reference to the node
            let node = self.get(&id)?;
            // Generate the list of dependency tuples
            node.deps
                .clone()
                .into_iter()
                .map(|d| (node.id.clone(), d.clone()))
                .collect()
        };

        // Push the new deps to the *end* of the list of this tree's unlinked deps
        self.append_deps(deps)?;

        // Link any and all unlinked deps
        self.link_unlinked_deps()?;

        Ok(self)
    }

    fn append_deps(&mut self, deps: Vec<(String, String)>) -> Result<&mut Self, String> {
        self.unlinked_deps.extend(deps);
        Ok(self)
    }

    /// Iterate over and link any unlinked deps
    fn link_unlinked_deps(&mut self) -> Result<&mut Self, String> {
        // This is *slightly* more performant than using retain but more importantly, for my sanity,
        // doesn't have the borrow conflicts. Every removal does cause a shift - which is why we start from
        // the end as it is most likely that the most recently inserted node will use this function and thus
        // less shifts will occur.
        if self.unlinked_deps.len() >= 1 {
            // We are starting from the back index
            let mut idx = self.unlinked_deps.len() - 1;

            while idx >= 0 as usize {
                let dp = &self.unlinked_deps[idx].clone();

                // If both nodes are contained
                if self.contains_pair(&dp.0, &dp.1) {
                    self.link_dep(&dp.0, &dp.1)?;
                    self.unlinked_deps.remove(idx);
                    continue;
                } else if idx == 0 {
                    break;
                } else {
                    idx -= 1;
                }
            }
        }
        Ok(self)
    }

    fn link_dep(&mut self, target: &String, dep: &String) -> Result<&mut Self, String> {
        if self.contains_pair(target, dep) {
            let node_target = self.get_mut(&target)?.index.unwrap();
            let node_dep = self.get_mut(&dep)?.index.unwrap();

            // Create the edge from the dependency to the target
            let dep_index =
                self.graph
                    .add_edge(node_target, node_dep, self.get(dep)?.is_complete());

            Ok(self)
        } else {
            Err(format!(
                "Cannot create dependency between '{}' and '{}' in tree '{}', one of the nodes does not exist",
                target, dep, self.id
            ))
        }
    }

    fn unlink_deps(&mut self, id: &String) -> Result<&mut Self, String> {
        panic!("Unimplemented");
        Ok(self)
    }

    /// Find the dependency in the list that has the furthest column.
    fn furthest_dep_col(&self, deps: &Vec<String>) -> u16 {
        deps.iter()
            .fold(None, |p, c| match self.get(c) {
                Ok(n) => {
                    if p.is_some_and(|x| x < n.col) {
                        Some(n.col)
                    } else {
                        p
                    }
                }
                Err(_) => p,
            })
            .unwrap_or(0)
    }

    /// Find the default column to place a node in based on its furthest dependency plus one.
    fn default_column(&self, deps: &Vec<String>) -> u16 {
        match self.furthest_dep_col(deps) {
            0 => 0,
            v => v + 1,
        }
    }

    /// Find the column siblings and then determine the deepest non-conflicting row value.
    fn default_row(&self, col: u16) -> u8 {
        self.map
            .values()
            .fold(None, |p, c| {
                if c.col == col && p.is_some_and(|x| x < c.row) {
                    Some(c.row)
                } else {
                    p
                }
            })
            .unwrap_or(0)
    }
}
