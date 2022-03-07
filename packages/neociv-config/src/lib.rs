#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy, Default)]
pub struct NeocivConfig {
    pub video: VideoConfig,
    pub window: WindowConfig,
}
