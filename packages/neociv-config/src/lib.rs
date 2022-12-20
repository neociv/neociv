use std::env;

use bevy::prelude::Resource;

#[derive(Clone, Copy, Debug)]
pub struct VideoConfig {
    pub vsync: bool,
    pub msaa_samples: u32,
}

impl Default for VideoConfig {
    fn default() -> Self {
        VideoConfig {
            vsync: false,
            msaa_samples: 1,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct WindowConfig {
    pub fullscreen: bool,
    pub resolution_width: u32,
    pub resolution_height: u32,
}

impl Default for WindowConfig {
    fn default() -> Self {
        WindowConfig {
            fullscreen: false,
            resolution_width: 1920,
            resolution_height: 1080,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ContentConfig {
    pub root: String,
}

impl Default for ContentConfig {
    fn default() -> Self {
        // Define the root, first by checking an environment variable and then by checking the runtime path.
        // This is just the *default* methodology and can be set to any value desired elsewhere.
        let root = match env::var("NEOCIV_CONTENT_ROOT") {
            Ok(p) => p,
            Err(_) => match env::current_dir() {
                Ok(p) => p.join("content").display().to_string(),
                Err(_) => panic!("Failed to define a content root directory"),
            },
        };

        ContentConfig { root }
    }
}

#[derive(Clone, Debug, Default, Resource)]
pub struct NeocivConfig {
    pub video: VideoConfig,
    pub window: WindowConfig,
    pub content: ContentConfig,
}
