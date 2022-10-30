use std::collections::HashMap;

use rlua::{Error as LuaError, FromLua, ToLua, Value as LuaValue};
use serde::{Deserialize, Serialize};
use serde_diff::SerdeDiff;

use crate::tree::tree_def::{TreeId, TreeNodeId, TreeNodeCost};

#[derive(Clone, Debug, Serialize, Deserialize, Default, SerdeDiff)]
pub struct TreeState {
    id: TreeId,
    nodes: HashMap<TreeNodeId, TreeStateNode>,
    target: TreeNodeId,
}

impl<'lua> ToLua<'lua> for TreeState {
    fn to_lua(self, ctx: rlua::Context<'lua>) -> rlua::Result<rlua::Value<'lua>> {
        let tree_state_tbl = ctx.create_table()?;
        tree_state_tbl.set("id", self.id)?;
        tree_state_tbl.set("nodes", self.nodes)?;
        tree_state_tbl.set("target", self.target)?;
        Ok(LuaValue::Table(tree_state_tbl))
    }
}

impl<'lua> FromLua<'lua> for TreeState {
    fn from_lua(lua_value: LuaValue<'lua>, _: rlua::Context<'lua>) -> rlua::Result<Self> {
        match lua_value {
            LuaValue::Table(table) => Ok(TreeState {
                id: table.get::<_, String>("id")?,
                nodes: table.get::<_, HashMap<TreeNodeId, TreeStateNode>>("nodes")?,
                target: table.get::<_, String>("target")?,
            }),
            _ => Err(LuaError::FromLuaConversionError {
                from: lua_value.type_name(),
                to: "TreeState",
                message: None,
            }),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, SerdeDiff)]
pub struct TreeStateNode {
    id: TreeNodeId,
    supply: TreeNodeCost,
    active: bool,
}

impl<'lua> ToLua<'lua> for TreeStateNode {
    fn to_lua(self, ctx: rlua::Context<'lua>) -> rlua::Result<rlua::Value<'lua>> {
        let tree_state_node_tbl = ctx.create_table()?;
        tree_state_node_tbl.set("id", self.id)?;
        tree_state_node_tbl.set("supply", self.supply)?;
        tree_state_node_tbl.set("active", self.active)?;
        Ok(LuaValue::Table(tree_state_node_tbl))
    }
}

impl<'lua> FromLua<'lua> for TreeStateNode {
    fn from_lua(lua_value: LuaValue<'lua>, _: rlua::Context<'lua>) -> rlua::Result<Self> {
        match lua_value {
            LuaValue::Table(table) => Ok(TreeStateNode {
                id: table.get::<_, String>("id")?,
                supply: table.get::<_, TreeNodeCost>("supply")?,
                active: table.get::<_, bool>("active")?,
            }),
            _ => Err(LuaError::FromLuaConversionError {
                from: lua_value.type_name(),
                to: "TreeStateNode",
                message: None,
            }),
        }
    }
}