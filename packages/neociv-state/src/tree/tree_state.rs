use std::collections::HashMap;

use crate::{
    state_table,
    tree::tree_def::{TreeId, TreeNodeCost, TreeNodeId},
};

state_table! {
    pub struct TreeStateNode {
        id: TreeNodeId,
        supply: TreeNodeCost,
        active: bool,
    }
}

state_table! {
    pub struct TreeState {
        id: TreeId,
        nodes: HashMap<TreeNodeId, TreeStateNode>,
        target: TreeNodeId,
    }
}
