#[derive(Clone,Copy)]
pub struct VideoConfig {
    pub vsync: bool,
    pub msaa_samples: u32,
}

impl Default for VideoConfig {
    fn default() -> Self {
        VideoConfig { vsync: false, msaa_samples: 1 }
    }
}

#[derive(Clone,Copy,Default)]
pub struct NeocivConfig {
    pub video: VideoConfig,
}
