use anyhow::Result;
use hotwatch::{Event, Hotwatch};
use rust_shader_tools::ShaderCompiler;
// TODO: handle shader stage better
pub use rust_shader_tools::ShaderStage as ShaderStageC;
use std::path::Path;

pub struct ShaderWatcher {
    hotwatch: Hotwatch,
}

impl ShaderWatcher {
    pub fn new() -> Result<Self> {
        let hotwatch = Hotwatch::new()?;
        Ok(Self { hotwatch })
    }

    pub fn watch(&mut self, path: impl AsRef<Path>, shader_stage: ShaderStageC) -> Result<()> {
        self.hotwatch.watch(path, move |event: Event| {
            if let Event::Write(path) = event {
                let shader_compiler =
                    ShaderCompiler::new().expect("Could not create shader compiler");
                shader_compiler
                    .compile_shader(path, shader_stage)
                    .expect("Could not compile shader");
            }
        })?;
        Ok(())
    }
}
