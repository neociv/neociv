use petgraph::{adj::EdgeIndex, graph::NodeIndex};

#[derive(Clone, Debug)]
pub struct Node {
    pub id: String,
    pub index: Option<NodeIndex>,
    pub cost: u32,
    pub total: u32,
    pub col: u16,
    pub row: u8,
    pub deps: Vec<String>,
    pub linked_deps: Vec<EdgeIndex>,
}

impl Node {
    /// Whether or not this node has an index in the graph assigned to it.
    pub fn has_index(&self) -> bool {
        self.index.is_some()
    }

    /// Set the graph index of this node, note that it can only be done *once* and try to set it
    /// again will result in an Error.
    pub fn set_index(&mut self, index: NodeIndex) -> Result<&mut Self, String> {
        if !self.has_index() {
            self.index = Some(index);
            Ok(self)
        } else {
            Err(format!("Cannot set a new index on node '{}'", self.id))
        }
    }

    /// "Completes" the node by setting the total to the cost, the actual completion of the
    /// dependency edges is handled by the tree.
    pub fn complete(&mut self) -> &mut Self {
        self.total = self.cost;
        return self;
    }

    /// Whether or not this particular node has been completed
    pub fn is_complete(&self) -> bool {
        self.total >= self.cost
    }
}
