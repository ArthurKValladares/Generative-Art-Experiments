use super::compiled_scene::CompiledScene;
use anyhow::Result;
use bytes::Bytes;
use math::vec::{Vec2, Vec3, Vec4};
use std::{
    path::Path,
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

fn compile_gltf_node<F>(node: &gltf::scene::Node, f: &mut F)
where
    F: FnMut(&gltf::scene::Node),
{
    f(&node);

    for child in node.children() {
        compile_gltf_node(&child, f);
    }
}

pub struct ImageData {
    pub width: u32,
    pub height: u32,
    pub bytes: Bytes,
}

pub struct GltfScene {
    gltf: gltf::Document,
    buffers: Vec<gltf::buffer::Data>,
    images: Vec<ImageData>,
}

impl GltfScene {
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let (document, buffers, images) = gltf::import(path)?;
        let images = images
            .into_iter()
            .map(|image_data| ImageData {
                width: image_data.width,
                height: image_data.height,
                bytes: Bytes::from(build_rgba_buffer(image_data)),
            })
            .collect::<Vec<_>>();
        Ok(Self {
            gltf: document,
            buffers,
            images,
        })
    }

    pub fn compile(&self) -> Result<CompiledScene, GltfSceneError> {
        let buffers = self.buffer_data();

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

                        // Process colors
                        let mut colors = if let Some(iter) = reader.read_colors(0) {
                            iter.into_rgba_f32()
                                .map(|data| data.into())
                                .collect::<Vec<Vec4>>()
                        } else {
                            vec![Vec4::new(1.0, 1.0, 1.0, 1.0); positions.len()]
                        };

                        // Process uvs
                        let mut uvs = if let Some(iter) = reader.read_tex_coords(0) {
                            iter.into_f32()
                                .map(|data| data.into())
                                .collect::<Vec<Vec2>>()
                        } else {
                            vec![Vec2::new(0.0, 0.0); positions.len()]
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
                        compiled_scene.colors.append(&mut colors);
                        compiled_scene.uvs.append(&mut uvs);
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

    pub fn buffer_data(&self) -> &[gltf::buffer::Data] {
        &self.buffers
    }

    pub fn image_data(&self) -> &[ImageData] {
        &self.images
    }
}

fn build_rgba_buffer(image: gltf::image::Data) -> Vec<u8> {
    let mut buffer = Vec::new();
    let size = image.width * image.height;
    for index in 0..size {
        let rgba = get_next_rgba(&image.pixels, image.format, index as usize);
        buffer.extend_from_slice(&rgba);
    }
    buffer
}

fn get_next_rgba(pixels: &[u8], format: gltf::image::Format, index: usize) -> [u8; 4] {
    match format {
        gltf::image::Format::R8 => [pixels[index], 0, 0, std::u8::MAX],
        gltf::image::Format::R8G8 => [pixels[index * 2], pixels[index * 2 + 1], 0, std::u8::MAX],
        gltf::image::Format::R8G8B8 => [
            pixels[index * 3],
            pixels[index * 3 + 1],
            pixels[index * 3 + 2],
            std::u8::MAX,
        ],
        gltf::image::Format::B8G8R8 => [
            pixels[index * 3 + 2],
            pixels[index * 3 + 1],
            pixels[index * 3],
            std::u8::MAX,
        ],
        gltf::image::Format::R8G8B8A8 => [
            pixels[index * 4],
            pixels[index * 4 + 1],
            pixels[index * 4 + 2],
            pixels[index * 4 + 3],
        ],
        gltf::image::Format::B8G8R8A8 => [
            pixels[index * 4 + 2],
            pixels[index * 4 + 1],
            pixels[index * 4],
            pixels[index * 4 + 3],
        ],
        _ => {
            panic!("image format not supported")
        }
    }
}
