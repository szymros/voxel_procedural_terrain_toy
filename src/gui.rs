use egui::*;
use egui_wgpu::{Renderer, ScreenDescriptor};
use egui_winit::State;
use wgpu::{CommandEncoder, Device, Queue, StoreOp, TextureFormat, TextureView};
use winit::event::WindowEvent;
use winit::window::Window;

use crate::generation_params::GenerationParams;
// use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};
//

pub struct GuiRenderer {
    state: State,
    renderer: Renderer,
    pub updated: bool,
    pub seed: u32,
    pub octaves: usize,
    pub frequency: f64,
    pub ground_level: u32,
    pub water_level: u32,
    pub noise_multiplier: f64,
    pub dirt_layer_height: u32,
}

impl GuiRenderer {
    pub fn context(&self) -> &Context {
        return self.state.egui_ctx();
    }

    pub fn new(
        device: &Device,
        output_color_format: TextureFormat,
        output_depth_format: Option<TextureFormat>,
        msaa_samples: u32,
        window: &Window,
    ) -> GuiRenderer {
        let egui_context = Context::default();

        let egui_state = egui_winit::State::new(
            egui_context,
            egui::viewport::ViewportId::ROOT,
            &window,
            Some(window.scale_factor() as f32),
            None,
        );
        let egui_renderer = Renderer::new(
            device,
            output_color_format,
            output_depth_format,
            msaa_samples,
        );

        GuiRenderer {
            state: egui_state,
            renderer: egui_renderer,
            updated: false,
            seed: 2,
            octaves: 2,
            frequency: 2.0,
            ground_level: 20,
            water_level: 10,
            noise_multiplier: 20.0,
            dirt_layer_height: 2,
        }
    }
    pub fn get_generation_params(&self) -> GenerationParams {
        GenerationParams {
            seed: self.seed,
            octaves: self.octaves,
            frequency: self.frequency,
            ground_level: self.ground_level,
            water_level: self.water_level,
            noise_multiplier: self.noise_multiplier,
            dirt_layer_height: self.dirt_layer_height,
        }
    }

    pub fn handle_input(&mut self, window: &Window, event: &WindowEvent) {
        self.state.on_window_event(window, &event);
    }

    pub fn ppp(&mut self, v: f32) {
        self.state.egui_ctx().set_pixels_per_point(v);
    }

    pub fn draw(
        &mut self,
        device: &Device,
        queue: &Queue,
        encoder: &mut CommandEncoder,
        window: &Window,
        window_surface_view: &TextureView,
        screen_descriptor: ScreenDescriptor,
    ) {
        self.state
            .egui_ctx()
            .set_pixels_per_point(screen_descriptor.pixels_per_point);
        let raw_input = self.state.take_egui_input(&window);
        let full_output =
            self.state.egui_ctx().run(raw_input, |ctx| {
                egui::Window::new("Generation settings")
                    .resizable(true)
                    .vscroll(true)
                    .default_open(false)
                    .show(&ctx, |mut ui| {
                        ui.label("Seed:");
                        let mut raw_seed_input: String = self.seed.to_string();
                        let mut responses: Vec<egui::Response> = vec![];
                        let seed_resp = ui
                            .add(egui::TextEdit::singleline(&mut raw_seed_input).hint_text("Seed"));
                        self.seed = raw_seed_input.parse().unwrap();
                        ui.separator();
                        ui.label("Octaves:");
                        responses.push(
                            ui.add(egui::Slider::new(&mut self.octaves, 1..=10).text("Octaves")),
                        );
                        ui.separator();
                        ui.label("Frequency:");
                        responses.push(ui.add(
                            egui::Slider::new(&mut self.frequency, 1.0..=20.0).text("Frequency"),
                        ));
                        ui.separator();
                        ui.label("Ground level:");
                        responses.push(ui.add(
                            egui::Slider::new(&mut self.ground_level, 1..=32).text("Ground level"),
                        ));
                        ui.separator();
                        ui.label("Water level:");
                        responses.push(ui.add(
                            egui::Slider::new(&mut self.water_level, 1..=63).text("Water level"),
                        ));
                        ui.separator();
                        ui.label("Noise multiplier:");
                        responses.push(
                            ui.add(
                                egui::Slider::new(&mut self.noise_multiplier, 1.0..=32.0)
                                    .text("Noise multiplier"),
                            ),
                        );
                        ui.separator();
                        ui.label("Dirt layer height:");
                        responses.push(
                            ui.add(
                                egui::Slider::new(&mut self.dirt_layer_height, 1..=5)
                                    .text("Dirt layer height"),
                            ),
                        );
                        if responses.iter().any(|x| x.changed()) {
                            self.updated = true;
                        }
                    });
            });

        self.state
            .handle_platform_output(&window, full_output.platform_output);

        let tris = self
            .state
            .egui_ctx()
            .tessellate(full_output.shapes, self.state.egui_ctx().pixels_per_point());
        for (id, image_delta) in &full_output.textures_delta.set {
            self.renderer
                .update_texture(&device, &queue, *id, &image_delta);
        }
        self.renderer
            .update_buffers(&device, &queue, encoder, &tris, &screen_descriptor);
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &window_surface_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            label: Some("egui main render pass"),
            occlusion_query_set: None,
        });
        self.renderer.render(&mut rpass, &tris, &screen_descriptor);
        drop(rpass);
        for x in &full_output.textures_delta.free {
            self.renderer.free_texture(x)
        }
    }
}
