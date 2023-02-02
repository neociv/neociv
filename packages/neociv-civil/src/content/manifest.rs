use std::fs;
use std::path::{Path, PathBuf};

#[derive(
    Clone,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    bevy_ecs::component::Component,
    bevy_ecs::system::Resource,
)]
pub struct NeocivManifest {
    pub id: String,
    pub title: String,
    pub version: String,
    pub authors: Vec<String>,
}

impl NeocivManifest {
    pub fn from_child_path_str(ps: &str) -> Option<Self> {
        let mut path: PathBuf = Path::new(ps).into();
        let file = Path::new("manifest.json");

        let manifest_path = loop {
            path.push(file);

            if path.is_file() {
                break Some(path);
            } else if !(path.pop() && path.pop()) {
                break None;
            }
        };

        if manifest_path.is_some() {
            let manifest_path_str = manifest_path.clone().unwrap();
            let manifest_content = fs::read_to_string(manifest_path_str.to_str().unwrap());

            if manifest_content.is_ok() {
                return serde_json::from_str(manifest_content.unwrap().as_str()).unwrap();
            } else {
                return None;
            }
        } else {
            return None;
        }
    }
}
