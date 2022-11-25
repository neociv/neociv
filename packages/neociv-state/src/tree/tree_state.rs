use std::collections::HashMap;

use neociv_macros::StateTable;
use serde::{Deserialize, Serialize};
use serde_diff::SerdeDiff;

use crate::tree::tree_def::{TreeId, TreeNodeCost, TreeNodeId};

#[derive(Clone, Debug, Serialize, Deserialize, Default, SerdeDiff, StateTable)]
pub struct TreeStateNode {
    id: TreeNodeId,
    supply: TreeNodeCost,
    active: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, SerdeDiff, StateTable)]
pub struct TreeState {
    id: TreeId,
    nodes: HashMap<TreeNodeId, TreeStateNode>,
    target: TreeNodeId,
}
