// https://rust-tutorials.github.io/learn-gfx-hal/03_clear_the_window.html

use winit::{
    dpi,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use simple_logger;

#[cfg(feature = "dx12")]
use gfx_backend_dx12 as gfx_backend;
#[cfg(feature = "metal")]
use gfx_backend_metal as gfx_backend;
#[cfg(feature = "vulkan")]
use gfx_backend_vulkan as gfx_backend;

struct HalState {}

struct LocalState {}

struct WinitState {}

impl HalState {
    pub fn draw_clear_frame(&mut self, color: [f32; 4]) -> Result<(), &'static str> {
        unimplemented!()
    }
}

fn main_loop() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Shady")
        .with_inner_size(dpi::LogicalSize::new(600, 600))
        .build(&event_loop)
        .unwrap();

    // let window2 = WindowBuilder::new()
    //     .with_title("Sample")
    //     .with_inner_size(dpi::LogicalSize::new(400, 400))
    //     .build(&event_loop)
    //     .unwrap();

    let now = std::time::SystemTime::now();

    let window_id = window.id();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } => *control_flow = ControlFlow::Exit,

            Event::RedrawRequested(window_id) => {
                let delta_sec = now.elapsed().unwrap().as_micros() as f64 / 1_000_000.0;

                window.request_redraw();
            }

            _ => (),
        }
    });
}