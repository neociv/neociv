use bevy_math::Vec3;
use rlua::UserData;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Camera {
    pub position: Vec3,
}

impl UserData for Camera {}
