use crate::vertex::Vertex;
use easy_ash::{
    BindingDesc, Buffer, BufferType, ClearValue, Context, DescriptorBufferInfo, DescriptorInfo,
    DescriptorPool, DescriptorSet, DescriptorType, Device, GraphicsPipeline, GraphicsProgram,
    PushConstant, RenderPass, RenderPassAttachment, Sampler, SamplerFilter, SamplerWrapMode,
    Shader, ShaderStage, Swapchain,
};
use egui::{epaint::Primitive, ClippedPrimitive, FullOutput, Mesh, Rect};
use math::vec::{Vec2, Vec4};
use winit::window::Window;

#[repr(C)]
#[derive(Clone, Debug, Copy)]
struct EguiPushConstantData {
    width: u32,
    height: u32,
    pad_1: u32,
    pad_2: u32,
}

pub struct Painter {
    egui_render_pass: RenderPass,
    egui_pipeline: GraphicsPipeline,
    egui_descriptor_pool: DescriptorPool,
    egui_descriptor_set: DescriptorSet,
    egui_push_constant: PushConstant,
    sampler: Sampler,
    // TODO: Better way to do this? Better way to handle these transient buffers in general
    vertex_buffer: Option<Buffer>,
    index_buffer: Option<Buffer>,
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

        let sampler = Sampler::new(&device, SamplerFilter::Nearest, SamplerWrapMode::Clamp)
            .expect("Could not create sampler");

        let egui_descriptor_pool =
            DescriptorPool::new(&device).expect("Could not create descriptor pool");
        let bind_desc = vec![
            BindingDesc::new(DescriptorType::StorageBuffer, 1, ShaderStage::Vertex),
            //BindingDesc::new(
            //    DescriptorType::CombinedImageSampler,
            //    1,
            //    ShaderStage::Fragment,
            //),
        ];

        let egui_push_constant = PushConstant {
            stage: ShaderStage::Vertex,
            offset: 0,
            size: std::mem::size_of::<EguiPushConstantData>() as u32,
        };

        let egui_descriptor_set = DescriptorSet::new(&device, &egui_descriptor_pool, &bind_desc)
            .expect("Could not create descriptor set");

        let egui_program = GraphicsProgram::new(
            Shader::new(
                &device,
                "carbon/src/egui_integration/shaders/spv/egui.vert.spv",
            )
            .expect("Could not create vertex shader"),
            Shader::new(
                &device,
                "carbon/src/egui_integration/shaders/spv/egui.frag.spv",
            )
            .expect("Could not create fragment shader"),
        );
        let egui_pipeline = GraphicsPipeline::new(
            &device,
            &swapchain,
            &egui_render_pass,
            &egui_program,
            &[&egui_descriptor_set],
            &[&egui_push_constant],
            false,
        )
        .expect("Could not create graphics pipeline");

        Self {
            egui_render_pass,
            egui_pipeline,
            egui_descriptor_pool,
            egui_descriptor_set,
            egui_push_constant,
            sampler,
            vertex_buffer: None,
            index_buffer: None,
        }
    }

    pub fn paint(
        &mut self,
        device: &Device,
        context: &Context,
        window: &Window,
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
                        window,
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
        window: &Window,
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
                pos: Vec4::new(vertex.pos.x, vertex.pos.y, 0.03, 1.0),
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

        self.egui_descriptor_set.bind(&[
            DescriptorInfo::StorageBuffer(DescriptorBufferInfo::new(&vertex_buffer, None, None)),
            //DescriptorInfo::CombinedImageSampler(vec![]),
        ]);
        self.egui_descriptor_set.update(&device);

        // Dummy draw logic, no texture (yet)
        let size = window.inner_size();
        self.egui_render_pass.begin(device, context, present_index);
        {
            device.push_constant(
                context,
                &self.egui_pipeline,
                &self.egui_push_constant,
                easy_ash::as_u8_slice(&EguiPushConstantData {
                    width: size.width,
                    height: size.height,
                    pad_1: 0,
                    pad_2: 0,
                }),
            );

            self.egui_pipeline.bind(device, context);
            device.bind_index_buffer(context, &index_buffer);
            self.egui_pipeline
                .bind_descriptor_set(device, context, &self.egui_descriptor_set);
            device.draw_indexed(context, 0, indices.len() as u32);
        }
        self.egui_render_pass.end(device, context);

        self.vertex_buffer = Some(vertex_buffer);
        self.index_buffer = Some(index_buffer);
    }

    pub fn resize(&mut self, device: &Device, swapchain: &Swapchain) {
        self.egui_render_pass
            .resize(device, swapchain)
            .expect("Could not resize RenderPass");
    }

    pub fn clean_buffers(&mut self, device: &Device) {
        unsafe {
            if let Some(vertex_buffer) = &self.vertex_buffer {
                vertex_buffer.clean(&device);
            }
            if let Some(index_buffer) = &self.index_buffer {
                index_buffer.clean(&device);
            }
        }
    }
}
