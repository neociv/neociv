use std::collections::HashMap;

use petgraph::graph::DiGraph;

use crate::node::Node;

pub struct NeocivTree {
    id: String,
    graph: DiGraph<String, bool>,
    map: HashMap<String, Node>,
    unlinked_deps: Vec<(String, String)>,
}

impl NeocivTree {
    pub fn new(id: String) -> NeocivTree {
        Self {
            id,
            graph: DiGraph::new(),
            map: HashMap::new(),
            unlinked_deps: Vec::new(),
        }
    }

    pub fn contains(&self, id: &String) -> bool {
        self.map.contains_key(id)
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
        let final_col = col.unwrap_or_else(|| self.furthest_dep_col(&final_deps));
        let final_row = row.unwrap_or_else(|| self.default_sibling_row(final_col));

        // Add the node with sane defaults
        self.add_node(Node {
            id,
            cost: cost.unwrap_or(1),
            total: total.unwrap_or(0),
            col: final_col,
            row: final_row,
            deps: final_deps,
            linked_deps: Vec::new(),
            index: None,
        })
    }

    fn add_node(&mut self, node: Node) -> Result<&mut Self, String> {
        if self.contains(&node.id) {
            Err(format!(
                "Node '{}' already exists in tree '{}'",
                node.id.as_str(),
                self.id.as_str()
            ))
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
        self.append_deps(deps);

        // Link any and all unlinked deps
        self.link_unlinked_deps();

        Ok(self)
    }

    fn append_deps(&mut self, deps: Vec<(String, String)>) {
        self.unlinked_deps.extend(deps)
    }

    /// Iterate over and link any unlinked deps
    fn link_unlinked_deps(&mut self) {
        let mut idx = 0 as usize;
        while idx < self.unlinked_deps.len() {
            if self.contains(&self.unlinked_deps[idx].0)
                && self.contains(&self.unlinked_deps[idx].1)
            {
                // TODO: Link edges in graph
                // TODO: Assign the directional edge indicies to the Node entries
                self.unlinked_deps.remove(idx);
                continue;
            }
            idx += 1;
        }
    }

    fn unlink_deps(&mut self, id: &String) -> Result<&mut Self, String> {
        panic!("Unimplemented");
        Ok(self)
    }

    fn furthest_dep_col(&self, deps: &Vec<String>) -> u16 {
        panic!("Unimplemented");
        0
    }

    fn default_sibling_row(&self, col: u16) -> u8 {
        panic!("Unimplemented");
        0
    }
}
