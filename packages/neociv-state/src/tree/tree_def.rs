use std::collections::HashMap;

use crate::state_table;

pub type TreeId = String;
pub type TreeNodeId = String;
pub type TreeNodeCost = u64;

state_table! {
    pub struct TreeDefNode {
        id: TreeNodeId,
        title: String,
        cost: TreeNodeCost,
    }
}

state_table! {
    pub struct TreeDef {
        id: TreeId,
        nodes: HashMap<TreeNodeId, TreeDefNode>,
    }
}
