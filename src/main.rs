mod camera;
mod chunk;
mod generation_params;
mod gui;
mod quad;
mod region;
mod state;
mod texture;
mod vertex;
mod voxel;
use gui::GuiRenderer;
use state::State;
use std::sync::Arc;

const WINDOW_WIDTH: u32 = 1360;
const WINDOW_HEIGHT: u32 = 768;

async fn run() {
    let event_loop = winit::event_loop::EventLoop::new().unwrap();
    let window = Arc::new(
        winit::window::WindowBuilder::new()
            .build(&event_loop)
            .unwrap(),
    );
    let _ = window.request_inner_size(winit::dpi::PhysicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT));
    let mut state = State::new(window.clone()).await.unwrap();
    let mut last_render_time = instant::Instant::now();
    let mut egui_renderer = GuiRenderer::new(&state.device, state.surface_format, None, 1, &window);
    let generation_params = egui_renderer.get_generation_params();
    let region = region::Region::new([0, 0], generation_params);
    let (vertices, indices) = region.build_mesh();
    state.set_buffers(vertices, indices);
    state.render(&mut egui_renderer, &window);
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

    let _ = event_loop.run(move |event, window_target| match event {
        winit::event::Event::WindowEvent {
            ref event,
            window_id: _,
        } => {
            egui_renderer.handle_input(&window, event);
            if !state.handle_input(event) {
                match event {
                    winit::event::WindowEvent::CloseRequested => {
                        window_target.exit();
                    }
                    winit::event::WindowEvent::RedrawRequested => {
                        let now = instant::Instant::now();
                        let dt: instant::Duration = now - last_render_time;
                        last_render_time = now;
                        state.update(dt);
                        if egui_renderer.updated {
                            let generation_params = egui_renderer.get_generation_params();
                            let region = region::Region::new([0, 0], generation_params);
                            let (vertices, indices) = region.build_mesh();
                            state.set_buffers(vertices, indices);
                        }
                        egui_renderer.updated = false;
                        state.render(&mut egui_renderer, &window);
                        window.request_redraw();
                    }

                    _ => {}
                }
            }
        }
        _ => (),
    });
}

fn main() {
    println!("Hello, world!");
    env_logger::init();
    pollster::block_on(run());
}
