use std::collections::HashMap;

use rlua::{Error as LuaError, FromLua, ToLua, Value as LuaValue};
use serde::{Deserialize, Serialize};
use serde_diff::SerdeDiff;

pub type TreeId = String;
pub type TreeNodeId = String;
pub type TreeNodeCost = u64;

#[derive(Clone, Debug, Serialize, Deserialize, Default, SerdeDiff)]
pub struct TreeDef {
    id: TreeId,
    nodes: HashMap<TreeNodeId, TreeDefNode>,
}

impl<'lua> ToLua<'lua> for TreeDef {
    fn to_lua(self, ctx: rlua::Context<'lua>) -> rlua::Result<rlua::Value<'lua>> {
        let tree_def_tbl = ctx.create_table()?;
        tree_def_tbl.set("id", self.id)?;
        tree_def_tbl.set("nodes", self.nodes)?;
        Ok(LuaValue::Table(tree_def_tbl))
    }
}

impl<'lua> FromLua<'lua> for TreeDef {
    fn from_lua(lua_value: LuaValue<'lua>, _: rlua::Context<'lua>) -> rlua::Result<Self> {
        match lua_value {
            LuaValue::Table(table) => Ok(TreeDef {
                id: table.get::<_, String>("id")?,
                nodes: table.get::<_, HashMap<TreeNodeId, TreeDefNode>>("nodes")?,
            }),
            _ => Err(LuaError::FromLuaConversionError {
                from: lua_value.type_name(),
                to: "TreeDef",
                message: None,
            }),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, SerdeDiff)]
pub struct TreeDefNode {
    id: TreeNodeId,
    title: String,
    cost: TreeNodeCost,
}

impl<'lua> ToLua<'lua> for TreeDefNode {
    fn to_lua(self, ctx: rlua::Context<'lua>) -> rlua::Result<rlua::Value<'lua>> {
        let tree_def_node_tbl = ctx.create_table()?;
        tree_def_node_tbl.set("id", self.id)?;
        tree_def_node_tbl.set("title", self.title)?;
        tree_def_node_tbl.set("cost", self.cost)?;
        Ok(LuaValue::Table(tree_def_node_tbl))
    }
}

impl<'lua> FromLua<'lua> for TreeDefNode {
    fn from_lua(lua_value: LuaValue<'lua>, _: rlua::Context<'lua>) -> rlua::Result<Self> {
        match lua_value {
            LuaValue::Table(table) => Ok(TreeDefNode {
                id: table.get::<_, String>("id")?,
                title: table.get::<_, String>("title")?,
                cost: table.get::<_, TreeNodeCost>("cost")?,
            }),
            _ => Err(LuaError::FromLuaConversionError {
                from: lua_value.type_name(),
                to: "TreeDefNode",
                message: None,
            }),
        }
    }
}
