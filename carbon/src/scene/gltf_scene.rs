use super::compiled_scene::CompiledScene;
use anyhow::Result;
use bytes::Bytes;
use image::GenericImageView;
use math::vec::Vec3;
use std::{
    fs::{self, File},
    io::{self, BufReader},
    path::{Path, PathBuf},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GltfSceneError {
    #[error("Binary buffers not yet supporterd")]
    BinaryBuffer,
    #[error("Image views not yet supporterd")]
    ViewImage,
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("Gltf file contained no default scene")]
    NoDefaultScene,
}

// Thanks to:
// https://github.com/EmbarkStudios/kajiya/blob/0382cfa57e2eb4cf4816e32fdea50d6ef2c9f263/crates/lib/kajiya-asset/src/import_gltf.rs
fn read_to_bytes(path: impl AsRef<Path>) -> Result<Vec<u8>, GltfSceneError> {
    use io::Read;
    let path = path.as_ref();
    let file = fs::File::open(path)?;
    let length = file.metadata().map(|x| x.len() + 1).unwrap_or(0);
    let mut reader = io::BufReader::new(file);
    let mut data = Vec::with_capacity(length as usize);
    reader.read_to_end(&mut data)?;
    Ok(data)
}

fn compile_gltf_node<F>(node: &gltf::scene::Node, f: &mut F)
where
    F: FnMut(&gltf::scene::Node),
{
    f(&node);

    for child in node.children() {
        compile_gltf_node(&child, f);
    }
}

pub struct GltfScene {
    file_root: PathBuf,
    gltf: gltf::Gltf,
}

impl GltfScene {
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let document = gltf::Gltf::open(path)?;
        let file_root = path.parent().unwrap_or(Path::new("./"));
        Ok(Self {
            file_root: file_root.to_owned(),
            gltf: document,
        })
    }

    pub fn compile(&self) -> Result<CompiledScene, GltfSceneError> {
        let buffers = self.buffer_data()?;

        // TODO: Have a way to compile other scenes
        if let Some(scene) = self
            .gltf
            .default_scene()
            .or_else(|| self.gltf.scenes().next())
        {
            let mut compiled_scene = CompiledScene::default();

            let mut process_node = |node: &gltf::scene::Node| {
                // Process Mesh
                if let Some(mesh) = node.mesh() {
                    // Process Mesh primitives
                    for prim in mesh.primitives() {
                        let reader = prim.reader(|buffer| Some(&buffers[buffer.index()]));

                        // Process vertex positions
                        let mut positions = if let Some(iter) = reader.read_positions() {
                            iter.map(|data| data.into()).collect::<Vec<Vec3>>()
                        } else {
                            return;
                        };

                        // Process Mesh indices
                        let mut indices = {
                            let mut indices = if let Some(indices_reader) = reader.read_indices() {
                                indices_reader.into_u32().collect::<Vec<u32>>()
                            } else {
                                (0..positions.len() as u32).collect::<Vec<u32>>()
                            };

                            let base_index = compiled_scene.positions.len() as u32;
                            for i in &mut indices {
                                *i += base_index;
                            }

                            indices
                        };

                        // TODO: remove need for mut bindings
                        compiled_scene.positions.append(&mut positions);
                        compiled_scene.indices.append(&mut indices);
                    }
                }
            };
            for node in scene.nodes() {
                compile_gltf_node(&node, &mut process_node);
            }
            Ok(compiled_scene)
        } else {
            Err(GltfSceneError::NoDefaultScene)
        }
    }

    pub fn buffer_data(&self) -> Result<Vec<Bytes>, GltfSceneError> {
        let mut buffer_data = Vec::new();
        for buffer in self.gltf.buffers() {
            let mut data = match buffer.source() {
                gltf::buffer::Source::Bin => Err(GltfSceneError::BinaryBuffer),
                gltf::buffer::Source::Uri(uri) => read_to_bytes(self.file_root.join(uri)),
            }?;
            while data.len() % 4 != 0 {
                data.push(0);
            }
            buffer_data.push(Bytes::from(data));
        }
        Ok(buffer_data)
    }

    pub fn image_data(&self) -> Result<Vec<((u32, u32), Bytes)>> {
        let mut image_data = Vec::new();
        for image in self.gltf.images() {
            let (dims, data) = match image.source() {
                gltf::image::Source::View {
                    view: _,
                    mime_type: _,
                } => Err(GltfSceneError::ViewImage),
                gltf::image::Source::Uri { uri, mime_type: _ } => {
                    // TODO: Same as in `Image::from_file`, refector. Refactor dims as well
                    let im = image::load(
                        BufReader::new(File::open(self.file_root.join(uri))?),
                        image::ImageFormat::Png,
                    )?;
                    Ok((im.dimensions(), im.into_bytes()))
                }
            }?;
            image_data.push((dims, Bytes::from(data)));
        }
        Ok(image_data)
    }
}
