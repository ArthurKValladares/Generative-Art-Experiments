use crate::vertex::Vertex;
use anyhow::Result;
use easy_ash::{
    ash::vk, new_descriptor_image_info, BindingDesc, Buffer, BufferType, ClearValue, Context,
    DescriptorBufferInfo, DescriptorInfo, DescriptorPool, DescriptorSet, DescriptorType, Device,
    Fence, GraphicsPipeline, GraphicsProgram, Image, MemoryMappablePointer, PushConstant,
    RenderPass, RenderPassAttachment, Sampler, SamplerFilter, SamplerWrapMode, Shader, ShaderStage,
    Swapchain, VertexInputData,
};
use egui::{
    epaint::{ImageDelta, Primitive},
    ClippedPrimitive, FullOutput, ImageData, Mesh, Rect, TextureId, TexturesDelta,
};
use math::vec::{Vec2, Vec4};
use std::collections::HashMap;
use winit::window::Window;

static VERTEX_BUFFER_SIZE: u64 = 1024 * 1024 * 4;
static INDEX_BUFFER_SIZE: u64 = 1024 * 1024 * 2;

#[repr(C)]
#[derive(Clone, Debug, Copy)]
struct EguiPushConstantData {
    size: Vec2,
}

pub struct Painter {
    egui_render_pass: RenderPass,
    egui_pipeline: GraphicsPipeline,
    egui_descriptor_pool: DescriptorPool,
    egui_descriptor_set: DescriptorSet,
    egui_push_constant: PushConstant,
    sampler: Sampler,
    texture_map: HashMap<TextureId, Image>,
    // TODO: Better way to do this? Better way to handle these transient buffers in general
    vertex_buffers: Vec<Buffer>,
    index_buffers: Vec<Buffer>,
}

impl Painter {
    pub fn new(device: &Device, swapchain: &Swapchain) -> Result<Self> {
        let egui_render_pass = RenderPass::new(
            &device,
            &swapchain,
            RenderPassAttachment::ColorLoad,
            None,
            Some(vec![vk::SubpassDependency::builder()
                .src_subpass(vk::SUBPASS_EXTERNAL)
                .dst_subpass(0)
                .src_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_WRITE)
                .dst_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_WRITE)
                .src_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
                .dst_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
                .build()]),
            &[ClearValue::Color(Vec4::new(1.0, 0.0, 1.0, 0.0))],
        )
        .expect("Could not create RenderPass");

        let sampler = Sampler::new(&device, SamplerFilter::Nearest, SamplerWrapMode::Clamp)
            .expect("Could not create sampler");

        let egui_descriptor_pool =
            DescriptorPool::new(&device).expect("Could not create descriptor pool");
        let bind_desc = vec![BindingDesc::new(
            DescriptorType::CombinedImageSampler,
            1,
            ShaderStage::Fragment,
        )];

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
            Some(VertexInputData {
                bindings: vec![vk::VertexInputBindingDescription::builder()
                    .binding(0)
                    .input_rate(vk::VertexInputRate::VERTEX)
                    .stride(
                        4 * std::mem::size_of::<f32>() as u32
                            + 4 * std::mem::size_of::<u8>() as u32,
                    )
                    .build()],
                attributes: vec![
                    // position
                    vk::VertexInputAttributeDescription::builder()
                        .binding(0)
                        .offset(0)
                        .location(0)
                        .format(vk::Format::R32G32_SFLOAT)
                        .build(),
                    // uv
                    vk::VertexInputAttributeDescription::builder()
                        .binding(0)
                        .offset(8)
                        .location(1)
                        .format(vk::Format::R32G32_SFLOAT)
                        .build(),
                    // color
                    vk::VertexInputAttributeDescription::builder()
                        .binding(0)
                        .offset(16)
                        .location(2)
                        .format(vk::Format::R8G8B8A8_UNORM)
                        .build(),
                ],
            }),
            &[&egui_descriptor_set],
            &[&egui_push_constant],
            false,
        )
        .expect("Could not create graphics pipeline");

        let (vertex_buffers, index_buffers) = {
            let len = egui_render_pass.framebuffers.len();
            let mut vertex_buffers = Vec::with_capacity(len);
            let mut index_buffers = Vec::with_capacity(len);
            for _ in (0..len) {
                let vertex_buffer =
                    Buffer::with_size(device, VERTEX_BUFFER_SIZE, BufferType::Vertex)?;
                vertex_buffers.push(vertex_buffer);
                let index_buffer = Buffer::with_size(device, INDEX_BUFFER_SIZE, BufferType::Index)?;
                index_buffers.push(index_buffer);
            }
            (vertex_buffers, index_buffers)
        };

        Ok(Self {
            egui_render_pass,
            egui_pipeline,
            egui_descriptor_pool,
            egui_descriptor_set,
            egui_push_constant,
            sampler,
            vertex_buffers,
            index_buffers,
            texture_map: Default::default(),
        })
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
        let mut vertex_buffer_ptr = self.vertex_buffers[present_index as usize].ptr().unwrap();
        let mut index_buffer_ptr = self.index_buffers[present_index as usize].ptr().unwrap();

        let mut vertex_base = 0;
        let mut index_base = 0;

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
                        &mut vertex_buffer_ptr,
                        &mut vertex_base,
                        &mut index_buffer_ptr,
                        &mut index_base,
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
        vertex_buffer_ptr: &mut MemoryMappablePointer,
        vertex_base: &mut i32,
        index_buffer_ptr: &mut MemoryMappablePointer,
        index_base: &mut u32,
    ) {
        // TODO: ideally we want an egui shader that can take this vertex output as is
        let vertices = &mesh.vertices;
        let indices = &mesh.indices;

        let vertex_copy_size = std::mem::size_of_val(vertices);
        let index_copy_size = std::mem::size_of_val(indices);

        vertex_buffer_ptr.copy_from(vertices, vertex_copy_size);
        index_buffer_ptr.copy_from(indices, index_copy_size);

        let vertex_buffer_ptr_next = vertex_buffer_ptr.add(vertex_copy_size);
        let index_buffer_ptr_next = index_buffer_ptr.add(index_copy_size);

        if vertex_buffer_ptr_next
            >= self.vertex_buffers[present_index as usize]
                .end_ptr()
                .unwrap()
            || index_buffer_ptr_next
                >= self.index_buffers[present_index as usize]
                    .end_ptr()
                    .unwrap()
        {
            panic!("egui out of memory");
        }
        *vertex_buffer_ptr = vertex_buffer_ptr_next;
        *index_buffer_ptr = index_buffer_ptr_next;

        let vertex_buffer = &self.vertex_buffers[present_index as usize];
        let index_buffer = &self.index_buffers[present_index as usize];

        // TODO: Write vertex and index data into buffer

        let image = self
            .texture_map
            .get(&mesh.texture_id)
            .expect("TextureId not in map");

        let size = window.inner_size();
        self.egui_render_pass.begin(device, context, present_index);
        {
            self.egui_pipeline.bind(device, context);
            device.bind_vertex_buffers(context, &[vertex_buffer]);
            device.bind_index_buffer(context, index_buffer);

            device.push_constant(
                context,
                &self.egui_pipeline,
                &self.egui_push_constant,
                easy_ash::as_u8_slice(&EguiPushConstantData {
                    size: Vec2::new(size.width as f32, size.height as f32),
                }),
            );

            self.egui_pipeline
                .bind_descriptor_set(device, context, &self.egui_descriptor_set);

            // TODO: This is still a problem
            self.egui_descriptor_set
                .bind(&[DescriptorInfo::CombinedImageSampler(vec![
                    new_descriptor_image_info(&image, &self.sampler),
                ])]);
            self.egui_descriptor_set.update(&device);

            device.draw_indexed(context, indices.len() as u32, *index_base, *vertex_base);
            *vertex_base += vertices.len() as i32;
            *index_base += indices.len() as u32;
        }
        self.egui_render_pass.end(device, context);
    }

    pub fn set_textures(
        &mut self,
        device: &Device,
        context: &Context,
        fence: &Fence,
        textures_delta: &TexturesDelta,
    ) {
        for (id, delta) in &textures_delta.set {
            self.set_image(device, context, fence, id, delta);
        }
    }

    fn set_image(
        &mut self,
        device: &Device,
        context: &Context,
        fence: &Fence,
        id: &TextureId,
        delta: &ImageDelta,
    ) {
        // TODO: Always updating full image for now
        // I think this might also be inneficient, investigate later the best way to get the byte vector with
        // no extra allocations
        // TODO: Need to figure out how syncronization works here. I need to be able to fully upload the image before rendering.
        // I think I want to have the set and free texture steps as separate steps outside command buffering recording for the draws
        match &delta.image {
            ImageData::Color(color_data) => {
                println!("IMAGE: {:?}", id);
                let (image, buffer) = Image::from_data_and_dims(
                    &device,
                    &context,
                    &fence,
                    color_data.width() as u32,
                    color_data.height() as u32,
                    easy_ash::as_u8_slice(&color_data.pixels),
                    false,
                )
                .expect("Could not crate image");
                self.texture_map.insert(*id, image);
            }
            ImageData::Font(font_data) => {
                let (image, buffer) = Image::from_data_and_dims(
                    &device,
                    &context,
                    &fence,
                    font_data.width() as u32,
                    font_data.height() as u32,
                    easy_ash::as_u8_slice(&font_data.pixels),
                    false,
                )
                .expect("Could not crate image");
                self.texture_map.insert(*id, image);
            }
        }
    }

    pub fn resize(&mut self, device: &Device, swapchain: &Swapchain) {
        self.egui_render_pass
            .resize(device, swapchain)
            .expect("Could not resize RenderPass");
    }

    pub fn clean_buffers(&mut self, device: &Device) {
        unsafe {
            for buffer in &mut self.vertex_buffers {
                buffer.clean(&device);
            }
            for buffer in &mut self.index_buffers {
                buffer.clean(&device);
            }
        }
    }
}
