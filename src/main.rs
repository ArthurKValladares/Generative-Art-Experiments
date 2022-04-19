use easy_ash::{ApplicationInfo, Context, Device, Entry, InstanceInfo, SurfaceBuilder, Swapchain};
use winit::{dpi::LogicalSize, event::Event, event_loop::EventLoop, window::WindowBuilder};

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

    let app_info = ApplicationInfo::default().with_application_name(app_title);
    let instance_info = InstanceInfo::default();
    let entry =
        Entry::new(app_info, instance_info, &window).expect("Could not create Easy-Ash instance");
    let surface_builder =
        SurfaceBuilder::new(&entry, &window, window_size.width, window_size.height)
            .expect("Could not create Easy-Ash SurfaceBuilder");
    let device = Device::new(&entry, &surface_builder).expect("Could not create Easy-Ash Device");
    let surface = surface_builder
        .build(&device)
        .expect("Could not create Easy-Ash Surface");
    let swapchain = Swapchain::new(&entry, &device, &surface);

    let setup_context = Context::new(&device);
    let draw_context = Context::new(&device);

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
