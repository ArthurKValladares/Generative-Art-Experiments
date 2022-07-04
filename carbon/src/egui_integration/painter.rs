use crate::vertex::Vertex;
use easy_ash::{
    Buffer, BufferType, ClearValue, Context, Device, RenderPass, RenderPassAttachment, Swapchain,
};
use egui::{epaint::Primitive, ClippedPrimitive, FullOutput, Mesh, Rect};
use math::vec::{Vec2, Vec4};

pub struct Painter {
    egui_render_pass: RenderPass,
}

impl Painter {
    pub fn new(device: &Device, swapchain: &Swapchain) -> Self {
        let egui_render_pass = RenderPass::new(
            &device,
            &swapchain,
            RenderPassAttachment::ColorLoad,
            RenderPassAttachment::DepthLoad,
            &[
                ClearValue::Color(Vec4::new(1.0, 0.0, 1.0, 0.0)),
                ClearValue::Depth {
                    depth: 1.0,
                    stencil: 0,
                },
            ],
        )
        .expect("Could not create RenderPass");
        Self { egui_render_pass }
    }

    pub fn paint(
        &mut self,
        device: &Device,
        context: &Context,
        present_index: u32,
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
                    self.paint_mesh(
                        device,
                        context,
                        present_index,
                        pixels_per_point,
                        clip_rect,
                        mesh,
                    );
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
        context: &Context,
        present_index: u32,
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
        let indices = &mesh.indices;

        let vertex_buffer = Buffer::from_data(&device, BufferType::Storage, &vertices)
            .expect("Could not create vertex buffer");
        let index_buffer = Buffer::from_data(&device, BufferType::Index, &indices)
            .expect("Could not create index buffer");

        // Dummy draw logic, no texture (yet)
        self.egui_render_pass.begin(device, context, present_index);
        {
            //egui_pipeline.bind(device, context);
            //device.bind_index_buffer(context, &index_buffer);
            //egui_pipeline.bind_descriptor_set(device, context, &egui_descriptor_set);
            //device.push_constant(
            //    context,
            //    &egui_pipeline,
            //    &material_push_constant,
            //    easy_ash::as_u8_slice(&material_data),
            //);
            //device.draw_indexed(context, mesh_draw.start_idx, mesh_draw.num_indices);
        }
        self.egui_render_pass.end(device, context);
    }

    pub fn resize(&mut self, device: &Device, swapchain: &Swapchain) {
        self.egui_render_pass
            .resize(device, swapchain)
            .expect("Could not resize RenderPass");
    }
}
