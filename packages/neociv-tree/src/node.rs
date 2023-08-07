use petgraph::graph::NodeIndex;

#[derive(Clone, Debug)]
pub struct Node {
    pub id: String,
    pub index: Option<NodeIndex>,
    pub cost: u32,
    pub total: u32,
    pub ord: u16,
    pub deps: Vec<String>,
}
