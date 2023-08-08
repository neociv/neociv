use std::{borrow::Cow, collections::HashMap};

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
            Err(format!("Unable to find node '{}' in tree '{}'", id, self.id))
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
            // Link the dependencies
            self.link_deps(&node.id)
        }
    }

    pub fn remove_node(&mut self, id: &String) -> Result<&mut Self, String> {
        panic!("Unimplemented");
        // TODO: Remove deps *but* if one end of the dep exists put it back into the unlinked_deps cache
        Ok(self)
    }

    fn link_deps(&mut self, id: &String) -> Result<&mut Self, String> {
        panic!("Unimplemented");
        // TODO: Link deps that exist
        // TODO: Search the unlinked_deps cache for any matching deps, link and remove them
        // TODO: Cache unlinkable deps into unlinked_deps
        Ok(self)
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
