mod camera;
mod chunk;
mod gui;
mod instance;
mod quad;
mod region;
mod state;
mod texture;
mod vertex;
mod voxel;
use cgmath::prelude::*;
use chunk::Chunk;
use egui_wgpu::ScreenDescriptor;
use state::State;
use vertex::Vertex;
use std::sync::Arc;
use wgpu::util::DeviceExt;
use winit::keyboard::{Key, ModifiersState, NamedKey};
use winit::{event::WindowEvent, event_loop};

async fn run() {
    let event_loop = event_loop::EventLoop::new().unwrap();
    let window = Arc::new(
        winit::window::WindowBuilder::new()
            .build(&event_loop)
            .unwrap(),
    );
    let mut state = State::new(window.clone()).await.unwrap();
    let mut last_render_time = instant::Instant::now(); // NEW!

    let vertex_chunk = Chunk::new_random([0.0, -1.0, 0.0]);
    let (mut vertices, mut indices) = vertex_chunk.build_mesh(0);
    state.set_buffers(vertices, indices);
    state.render();
    event_loop.set_control_flow(event_loop::ControlFlow::Poll);

    //let mut egui_renderer = GuiRenderer::new(&state.device, state.surface_format, None, 1, &window);
    // let mut close_requested = false;
    // let mut modifiers = ModifiersState::default();
    // let mut scale_factor = 1.0;

    let _ = event_loop.run(move |event, window_target| match event {
        winit::event::Event::DeviceEvent {
            device_id: _,
            event,
        } => match event {
            winit::event::DeviceEvent::MouseMotion { delta } => {
                if state.mouse_pressed {
                    state.camera_controller.process_mouse(delta.0, delta.1);
                }
            }
            _ => {}
        },
        winit::event::Event::WindowEvent {
            ref event,
            window_id: _,
        } if !state.handle_input(event) => match event {
            WindowEvent::CloseRequested => {
                window_target.exit();
            }
            WindowEvent::RedrawRequested => {
                let now = instant::Instant::now();
                let dt: instant::Duration = now - last_render_time;
                last_render_time = now;
                state.update(dt);
                state.render();
                window.request_redraw();
            }
            _ => {}
        },
        _ => (),
    });
}

fn main() {
    println!("Hello, world!");
    env_logger::init();
    pollster::block_on(run());
}
