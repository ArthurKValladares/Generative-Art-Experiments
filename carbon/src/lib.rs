use anyhow::Result;
use hotwatch::{Event, Hotwatch};
use std::path::Path;

pub struct ShaderWatcher {
    hotwatch: Hotwatch,
}

impl ShaderWatcher {
    pub fn new() -> Result<Self> {
        let hotwatch = Hotwatch::new()?;
        Ok(Self { hotwatch })
    }

    pub fn watch(&mut self, path: impl AsRef<Path>) -> Result<()> {
        self.hotwatch.watch(path, |event: Event| {
            if let Event::Write(path) = event {
                println!("Shader at: {:?} has changed.", path);
            }
        })?;
        Ok(())
    }
}
