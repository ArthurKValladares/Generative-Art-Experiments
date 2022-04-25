use easy_ash::{
    ApiVersion, ApplicationInfo, BindingDesc, Buffer, BufferType, Context, DescriptorBufferInfo,
    DescriptorPool, DescriptorSet, DescriptorType, Device, Entry, GraphicsPipeline,
    GraphicsProgram, Image, ImageResolution, ImageType, InstanceInfo, RenderPass, Shader,
    ShaderStage, Surface, Swapchain,
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
    let swapchain = Swapchain::new(
        &entry,
        &device,
        surface,
        window_size.width,
        window_size.height,
    )
    .expect("Could not create swapchain");
    let setup_context = Context::new(&device).expect("Could not create setup context");
    let draw_context = Context::new(&device).expect("Could not create draw context");

    let render_pass = RenderPass::new(&device, &swapchain).expect("Could not create RenderPass");

    let graphics_program = GraphicsProgram::new(
        Shader::new(&device, "src/shaders/triangle_vert.spv")
            .expect("Could not create vertex shader"),
        Shader::new(&device, "src/shaders/triangle_frag.spv")
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

    event_loop.run(move |event, _, control_flow| {
        *control_flow = winit::event_loop::ControlFlow::Poll;
        match event {
            Event::WindowEvent {
                event: window_event,
                ..
            } => match window_event {
                winit::event::WindowEvent::CloseRequested => {
                    *control_flow = winit::event_loop::ControlFlow::Exit
                }
                _ => {}
            },
            _ => {}
        }
    });
}
