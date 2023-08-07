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

    pub fn contains(&self, id: String) -> bool {
        self.map.contains_key(&id)
    }

    pub fn add(
        &mut self,
        id: String,
        cost: u32,
        total: u32,
        ord: u16,
        deps: Option<Vec<String>>,
    ) -> Result<&mut Self, String> {
        self.add_node(Node {
            id,
            cost,
            total,
            ord,
            deps: deps.unwrap_or(Vec::new()),
            index: None,
        })
    }

    pub fn add_node(&mut self, node: Node) -> Result<&mut Self, String> {
        if self.contains(node.id.clone()) {
            Err(format!(
                "Node '{}' already exists in tree '{}'",
                node.id.as_str(),
                self.id.as_str()
            ))
        } else {
            // Insert into the map
            self.map.insert(node.id.clone(), node.clone());
            // Insert into the graph
            self.graph.add_node(node.id.clone());
            Ok(self)
        }
    }

    pub fn remove_node(&mut self, id: String) -> Result<&mut Self, String> {
        panic!("Unimplemented");
        // TODO: Remove deps *but* if one end of the dep exists put it back into the unlinked_deps cache
        Ok(self)
    }

    fn link_deps(&mut self, id: String) -> Result<&mut Self, String> {
        panic!("Unimplemented");
        // TODO: Link deps that exist
        // TODO: Search the unlinked_deps cache for any matching deps, link and remove them
        // TODO: Cache unlinkable deps into unlinked_deps
        Ok(self)
    }

    fn unlink_deps(&mut self, id: String) -> Result<&mut Self, String> {
        panic!("Unimplemented");
        Ok(self)
    }
}
