use easy_ash::{
    math::vec::Vec4, ApiVersion, ApplicationInfo, BindingDesc, Buffer, BufferType, ClearValue,
    Context, DescriptorBufferInfo, DescriptorPool, DescriptorSet, DescriptorType, Device, Entry,
    Fence, GraphicsPipeline, GraphicsProgram, Image, ImageResolution, ImageType, InstanceInfo,
    PipelineStages, RenderPass, Semaphore, Shader, ShaderStage, Surface, Swapchain,
};
use winit::{dpi::LogicalSize, event::Event, event_loop::EventLoop, window::WindowBuilder};

// TODO: This will be defined in the shader later
#[derive(Clone, Debug, Copy)]
struct Vertex {
    pos: [f32; 4],
    color: [f32; 4],
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

    // TODO: Does `Entry` warrant being it's own struct? Should I just fold it into `Device`?
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
    )
    .expect("Could not create swapchain");
    let setup_context = Context::new(&device).expect("Could not create setup context");
    let draw_context = Context::new(&device).expect("Could not create draw context");

    let mut render_pass = RenderPass::new(
        &device,
        &swapchain,
        &[ClearValue::Color(Vec4::new(1.0, 0.0, 1.0, 0.0))],
    )
    .expect("Could not create RenderPass");

    let graphics_program = GraphicsProgram::new(
        Shader::new(&device, "src/shaders/spv/triangle.vert.spv")
            .expect("Could not create vertex shader"),
        Shader::new(&device, "src/shaders/spv/triangle.frag.spv")
            .expect("Could not create fragment shader"),
    );

    let index_buffer_data = [0u32, 1, 2];
    let index_buffer = Buffer::from_data(&device, BufferType::Index, &index_buffer_data)
        .expect("Could not create index buffer");

    let vertex_buffer_data = [
        Vertex {
            pos: [-1.0, 1.0, 0.0, 1.0],
            color: [0.0, 1.0, 0.0, 1.0],
        },
        Vertex {
            pos: [1.0, 1.0, 0.0, 1.0],
            color: [0.0, 0.0, 1.0, 1.0],
        },
        Vertex {
            pos: [0.0, -1.0, 0.0, 1.0],
            color: [1.0, 0.0, 0.0, 1.0],
        },
    ];
    let vertex_buffer = Buffer::from_data(&device, BufferType::Storage, &vertex_buffer_data)
        .expect("Could not create vertex buffer");

    let descriptor_pool = DescriptorPool::new(&device).expect("Could not create descriptor pool");
    let global_descriptor_set = DescriptorSet::new(
        &device,
        &descriptor_pool,
        &[BindingDesc::new(
            DescriptorType::StorageBuffer(DescriptorBufferInfo::new(&vertex_buffer, None, None)),
            1,
            ShaderStage::Vertex,
        )],
    )
    .expect("Could not create descriptor set");
    global_descriptor_set.update(&device);

    let graphics_pipeline = GraphicsPipeline::new(
        &device,
        &swapchain,
        &render_pass,
        &graphics_program,
        &[&global_descriptor_set],
    )
    .expect("Could not create graphics pipeline");

    let present_complete_semaphore = Semaphore::new(&device).expect("Could not create semaphore");
    let rendering_complete_semaphore = Semaphore::new(&device).expect("Could not create semaphore");

    let draw_commands_reuse_fence = Fence::new(&device).expect("Could not create Fence");
    let setup_commands_reuse_fence = Fence::new(&device).expect("Could not create Fence");

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
                        render_pass.clean(&device);
                        present_complete_semaphore.clean(&device);
                        rendering_complete_semaphore.clean(&device);
                        draw_commands_reuse_fence.clean(&device);
                        setup_commands_reuse_fence.clean(&device);
                        global_descriptor_set.clean(&device);
                        descriptor_pool.clean(&device);
                        swapchain.clean(&device);
                        device.clean();
                        entry.clean();
                    }
                }
                winit::event::WindowEvent::Resized(new_size) => {
                    // TODO: Should I wait idle here?
                    device.wait_idle();
                    swapchain.resize(&entry, &device, new_size.width, new_size.height);
                    render_pass.resize(&device, &swapchain);
                }
                _ => {}
            },
            _ => {}
        }

        let present_index = swapchain
            .acquire_next_image_index(&present_complete_semaphore)
            .expect("Could not acquire present image");

        draw_context.record(
            &device,
            &present_complete_semaphore,
            &rendering_complete_semaphore,
            &draw_commands_reuse_fence,
            &[PipelineStages::ColorAttachmentOutput],
            |device, context| {
                render_pass.begin(device, context, present_index);
                graphics_pipeline.bind(device, context);
                device.set_viewport_and_scissor(context, &swapchain);
                device.bind_index_buffer(context, &index_buffer);
                graphics_pipeline.bind_descriptor_set(device, context, &global_descriptor_set);
                device.draw_indexed(context, index_buffer_data.len() as u32);
                render_pass.end(device, context);
            },
        );

        swapchain.present(&device, &[&rendering_complete_semaphore], &[present_index]);
    });
}
