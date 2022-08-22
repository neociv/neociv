use bevy_math::Vec3;
use rlua::{ToLua, Value as LuaValue};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Camera {
    pub position: Vec3,
}

impl<'lua> ToLua<'lua> for Camera {
    fn to_lua(self, ctx: rlua::Context<'lua>) -> rlua::Result<LuaValue<'lua>> {
        let camera_tbl = ctx.create_table()?;
        let position_tbl = ctx.create_table()?;
        position_tbl.set("x", self.position.x)?;
        position_tbl.set("y", self.position.y)?;
        position_tbl.set("z", self.position.z)?;
        camera_tbl.set("position", position_tbl)?;
        Ok(LuaValue::Table(camera_tbl))
    }
}
