use anyhow::Result;
use hotwatch::{Event, Hotwatch};

struct ShaderWatcher {
    hotwatch: Hotwatch,
}

impl ShaderWatcher {
    pub fn new() -> Result<Self> {
        let hotwatch = hotwatch::new()?;
        Ok(Self { hotwatch })
    }
}
