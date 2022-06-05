use carbon::{
    camera::{Camera, OrtographicData},
    scene::GltfScene,
};
use easy_ash::{
    math::{
        mat::Mat4,
        vec::{Vec2, Vec3, Vec4},
    },
    AccessMask, ApiVersion, ApplicationInfo, BindingDesc, Buffer, BufferType, ClearValue, Context,
    DescriptorBufferInfo, DescriptorImageInfo, DescriptorPool, DescriptorSet, DescriptorType,
    Device, Entry, Fence, GraphicsPipeline, GraphicsProgram, Image, ImageLayout,
    ImageMemoryBarrier, InstanceInfo, PipelineStages, PushConstant, RenderPass, Sampler,
    SamplerFilter, SamplerWrapMode, Semaphore, Shader, ShaderStage, Surface, Swapchain,
};
use winit::{dpi::LogicalSize, event::Event, event_loop::EventLoop, window::WindowBuilder};

// TODO: This will be defined in the shader later
#[repr(C)]
#[derive(Clone, Debug, Copy)]
struct Vertex {
    pos: Vec4,
    color: Vec4,
    uv: Vec2,
    pad: Vec2,
}

#[repr(C)]
#[derive(Clone, Debug, Copy)]
struct PushConstantData {
    model_matrix: Mat4,
}

fn main() {
    let app_title = "Generative Art";
    let window_width = 1200;
    let window_height = 700;

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title(app_title)
        .with_inner_size(LogicalSize::new(
            f64::from(window_width),
            f64::from(window_height),
        ))
        .build(&event_loop)
        .unwrap();
    let window_size = window.inner_size();

    let entry = Entry::new(
        ApplicationInfo::default()
            .with_application_name(app_title)
            .with_api_version(ApiVersion::new(0, 1, 2, 0)),
        InstanceInfo::default(),
        &window,
    )
    .expect("Could not create Easy-Ash instance");
    let surface = Surface::new(&entry, &window).expect("Could not create Easy-Ash Surface");
    let device = Device::new(&entry, &surface).expect("Could not create Easy-Ash Device");
    let mut swapchain = Swapchain::new(
        &entry,
        &device,
        surface,
        window_size.width,
        window_size.height,
        true,
    )
    .expect("Could not create swapchain");
    let setup_context = Context::new(&device).expect("Could not create setup context");
    let draw_context = Context::new(&device).expect("Could not create draw context");

    let draw_commands_reuse_fence = Fence::new(&device).expect("Could not create Fence");
    let setup_commands_reuse_fence = Fence::new(&device).expect("Could not create Fence");

    swapchain
        .transition_depth_image(&device, &setup_context, &setup_commands_reuse_fence)
        .expect("could not transition depth image");

    setup_context
        .record(
            &device,
            &[],
            &[],
            &setup_commands_reuse_fence,
            &[],
            |device, context| {
                let layout_transition_barrier = ImageMemoryBarrier::new(
                    &swapchain.depth_image,
                    AccessMask::DepthStencil,
                    ImageLayout::Undefined,
                    ImageLayout::DepthStencil,
                );
                device.pipeline_image_barrier(
                    &setup_context,
                    PipelineStages::BottomOfPipe,
                    PipelineStages::LateFragmentTests,
                    &[layout_transition_barrier],
                );
            },
        )
        .expect("Could not record setup context");

    let mut render_pass = RenderPass::new(
        &device,
        &swapchain,
        &[
            ClearValue::Color(Vec4::new(1.0, 0.0, 1.0, 0.0)),
            ClearValue::Depth {
                depth: 1.0,
                stencil: 0,
            },
        ],
    )
    .expect("Could not create RenderPass");

    let graphics_program = GraphicsProgram::new(
        Shader::new(&device, "src/shaders/spv/triangle.vert.spv")
            .expect("Could not create vertex shader"),
        Shader::new(&device, "src/shaders/spv/triangle.frag.spv")
            .expect("Could not create fragment shader"),
    );
    let sampler = Sampler::new(&device, SamplerFilter::Nearest, SamplerWrapMode::Clamp)
        .expect("Could not create sampler");

    // Scene setup start
    // TODO: GLtf camera stuff
    let gltf_scene = GltfScene::new("glTF-Sample-Models/2.0/MultiUVTest/glTF/MultiUVTest.gltf")
        .expect("Coult not load gltf scene");
    let compiled_scene = gltf_scene.compile().expect("Could not compile Gltf Scene");

    let images = gltf_scene.image_data();
    let images_data = images
        .into_iter()
        .map(|image_data| {
            Image::from_data_and_dims(
                &device,
                &setup_context,
                &setup_commands_reuse_fence,
                image_data.width,
                image_data.height,
                &image_data.bytes,
            )
            .expect("Could not crate image")
        })
        .collect::<Vec<_>>();

    let index_buffer = Buffer::from_data(&device, BufferType::Index, &compiled_scene.indices)
        .expect("Could not create index buffer");

    let vertex_buffer_data = {
        let mut ret: Vec<Vertex> = Vec::with_capacity(compiled_scene.positions.len());
        for idx in 0..compiled_scene.positions.len() {
            ret.push(Vertex {
                pos: compiled_scene.positions[idx].into(),
                color: compiled_scene.colors[idx],
                uv: compiled_scene.uvs[idx],
                pad: Default::default(),
            });
        }
        ret
    };
    let vertex_buffer = Buffer::from_data(&device, BufferType::Storage, &vertex_buffer_data)
        .expect("Could not create vertex buffer");

    let camera_type = compiled_scene.cameras.first().unwrap();
    let camera = camera_type.build(window_size.width as f32, window_size.height as f32);

    let camera_buffer = Buffer::from_data(
        &device,
        BufferType::Uniform,
        std::slice::from_ref(camera.raw_matrix()),
    )
    .expect("Could not create vertex buffer");
    // Scene setup end

    let descriptor_pool = DescriptorPool::new(&device).expect("Could not create descriptor pool");
    let global_descriptor_set = DescriptorSet::new(
        &device,
        &descriptor_pool,
        &[
            BindingDesc::new(
                DescriptorType::StorageBuffer(DescriptorBufferInfo::new(
                    &vertex_buffer,
                    None,
                    None,
                )),
                1,
                ShaderStage::Vertex,
            ),
            BindingDesc::new(
                DescriptorType::CombinedImageSampler(DescriptorImageInfo::new(
                    &images_data[0].0, // TODO: actually handle this correctly later
                    &sampler,
                )),
                1,
                ShaderStage::Fragment,
            ),
            BindingDesc::new(
                DescriptorType::UniformBuffer(DescriptorBufferInfo::new(
                    &camera_buffer,
                    None,
                    None,
                )),
                1,
                ShaderStage::Vertex,
            ),
        ],
    )
    .expect("Could not create descriptor set");
    global_descriptor_set.update(&device);

    let camera_push_constant = PushConstant {
        stage: ShaderStage::Vertex,
        offset: 0,
        size: std::mem::size_of::<PushConstantData>() as u32,
    };

    let graphics_pipeline = GraphicsPipeline::new(
        &device,
        &swapchain,
        &render_pass,
        &graphics_program,
        &[&global_descriptor_set],
        &[&camera_push_constant],
    )
    .expect("Could not create graphics pipeline");

    let present_complete_semaphore = Semaphore::new(&device).expect("Could not create semaphore");
    let rendering_complete_semaphore = Semaphore::new(&device).expect("Could not create semaphore");

    let mut rotate_idx: u64 = 0;
    event_loop.run(move |event, _, control_flow| {
        *control_flow = winit::event_loop::ControlFlow::Poll;
        match event {
            Event::WindowEvent {
                event: window_event,
                ..
            } => match window_event {
                winit::event::WindowEvent::CloseRequested => {
                    *control_flow = winit::event_loop::ControlFlow::Exit;

                    device.wait_idle();
                    unsafe {
                        graphics_pipeline.clean(&device);
                        graphics_program.clean(&device);
                        index_buffer.clean(&device);
                        vertex_buffer.clean(&device);
                        camera_buffer.clean(&device);
                        render_pass.clean(&device);
                        present_complete_semaphore.clean(&device);
                        rendering_complete_semaphore.clean(&device);
                        draw_commands_reuse_fence.clean(&device);
                        setup_commands_reuse_fence.clean(&device);
                        global_descriptor_set.clean(&device);
                        descriptor_pool.clean(&device);
                        swapchain.clean(&device);
                        sampler.clean(&device);
                        for (image, staging_buffer) in &images_data {
                            image.clean(&device);
                            // TODO: This should be freed much sooner, handle later after we figure out syncronization
                            staging_buffer.clean(&device);
                        }
                        device.clean();
                        entry.clean();
                    }
                }
                winit::event::WindowEvent::Resized(new_size) => {
                    device.wait_idle();
                    swapchain.resize(
                        &entry,
                        &device,
                        &setup_context,
                        &setup_commands_reuse_fence,
                        new_size.width,
                        new_size.height,
                    );
                    render_pass.resize(&device, &swapchain);
                }
                winit::event::WindowEvent::KeyboardInput { input, .. } => match input {
                    winit::event::KeyboardInput {
                        virtual_keycode,
                        state,
                        ..
                    } => match (virtual_keycode, state) {
                        (
                            Some(winit::event::VirtualKeyCode::Escape),
                            winit::event::ElementState::Pressed,
                        ) => *control_flow = winit::event_loop::ControlFlow::Exit,
                        (
                            Some(winit::event::VirtualKeyCode::R),
                            winit::event::ElementState::Pressed,
                        ) => {
                            rotate_idx += 1;
                        }
                        _ => {}
                    },
                },
                _ => {}
            },
            _ => {}
        }

        let present_index = swapchain
            .acquire_next_image_index(&present_complete_semaphore)
            .expect("Could not acquire present image");

        draw_context
            .record(
                &device,
                &[present_complete_semaphore],
                &[rendering_complete_semaphore],
                &draw_commands_reuse_fence,
                &[PipelineStages::ColorAttachmentOutput],
                |device, context| {
                    render_pass.begin(device, context, present_index);
                    graphics_pipeline.bind(device, context);
                    device.set_viewport_and_scissor(context, &swapchain);
                    device.bind_index_buffer(context, &index_buffer);
                    graphics_pipeline.bind_descriptor_set(device, context, &global_descriptor_set);
                    device.push_constant(
                        context,
                        &graphics_pipeline,
                        &camera_push_constant,
                        easy_ash::as_u8_slice(&Mat4::rotate(
                            rotate_idx as f32 * 0.04,
                            Vec3::new(0.0, 1.0, 0.0),
                        )),
                    );
                    device.draw_indexed(context, compiled_scene.indices.len() as u32);
                    render_pass.end(device, context);
                },
            )
            .expect("Could not record draw context");

        swapchain
            .present(&device, &[&rendering_complete_semaphore], &[present_index])
            .expect("Could not present to swapchain");
    });
}
