use crate::vertex::Vertex;
use easy_ash::{Buffer, BufferType, Device};
use egui::{epaint::Primitive, ClippedPrimitive, FullOutput, Mesh, Rect};
use math::vec::{Vec2, Vec4};

pub struct Painter {}

impl Painter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn paint(
        &mut self,
        device: &Device,
        pixels_per_point: f32,
        clipped_primitives: &[ClippedPrimitive],
    ) {
        for egui::ClippedPrimitive {
            clip_rect,
            primitive,
        } in clipped_primitives
        {
            match primitive {
                Primitive::Mesh(mesh) => {
                    self.paint_mesh(device, pixels_per_point, clip_rect, mesh);
                }
                Primitive::Callback(_) => {
                    todo!("Custom rendering callbacks are not implemented");
                }
            }
        }
    }

    fn paint_mesh(
        &mut self,
        device: &Device,
        pixels_per_point: f32,
        clip_rect: &Rect,
        mesh: &Mesh,
    ) {
        // TODO: ideally we want an egui shader that can take this vertex output as is
        let vertices = mesh
            .vertices
            .iter()
            .map(|vertex| Vertex {
                pos: Vec4::new(vertex.pos.x, vertex.pos.y, 0.0, 1.0),
                color: Vec4::new(
                    vertex.color.r() as f32 / 255.0,
                    vertex.color.g() as f32 / 255.0,
                    vertex.color.b() as f32 / 255.0,
                    vertex.color.a() as f32 / 255.0,
                ),
                uv: Vec2::new(vertex.uv.x, vertex.uv.y),
                pad: Vec2::zero(),
            })
            .collect::<Vec<_>>();
        let vertex_buffer = Buffer::from_data(&device, BufferType::Storage, &vertices)
            .expect("Could not create vertex buffer");

        let indices = &mesh.indices;
        let index_buffer = Buffer::from_data(&device, BufferType::Index, &indices)
            .expect("Could not create index buffer");
    }
}
