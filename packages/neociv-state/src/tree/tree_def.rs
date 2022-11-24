use std::collections::HashMap;

use neociv_macros::StateTable;
use serde::{Deserialize, Serialize};
use serde_diff::SerdeDiff;

pub type TreeId = String;
pub type TreeNodeId = String;
pub type TreeNodeCost = u64;

#[derive(Clone, Debug, Serialize, Deserialize, Default, SerdeDiff, StateTable)]
pub struct TreeDefNode {
    id: TreeNodeId,
    title: String,
    cost: TreeNodeCost,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, SerdeDiff, StateTable)]
pub struct TreeDef {
    id: TreeId,
    nodes: HashMap<TreeNodeId, TreeDefNode>,
}
