use crate::scene::CompiledScene;
use anyhow::Result;
use std::path::Path;

pub struct GltfScene {
    document: gltf::Document,
    buffers: Vec<gltf::buffer::Data>,
    images: Vec<gltf::image::Data>,
}

impl GltfScene {
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let (document, buffers, images) = gltf::import(path)?;
        Ok(Self {
            document,
            buffers,
            images,
        })
    }

    pub fn default_compiled_scene(&self) -> Option<CompiledScene> {
        self.document.default_scene().map(|scene| CompiledScene {})
    }
}
