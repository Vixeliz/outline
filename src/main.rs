//! A very simple shader example.

use crevice::std140::AsStd140;
use ggegui::{egui, Gui};
use ggez::event;
use ggez::glam::Vec2;
use ggez::graphics::{self, Color, DrawParam, Drawable, Quad};
use ggez::{Context, GameResult};
use std::env;
use std::path;

#[derive(AsStd140)]
struct Stripes {
    stripe_size: f32,
    border_size: f32,
    stroke_size: f32,
    time: f32,
    speed: f32,
    tilt: f32,
    freq: f32,
}

struct MainState {
    stripes: Stripes,
    shader: graphics::Shader,
    params: graphics::ShaderParams<Stripes>,
    gui: Gui,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let shader = graphics::ShaderBuilder::new()
            .fragment_path("/striped.wgsl")
            .build(&ctx.gfx)?;
        let stripes = Stripes {
            stripe_size: 2.0,
            border_size: 0.02,
            stroke_size: 0.02,
            time: 0.0,
            speed: 0.5,
            tilt: -0.9,
            freq: 6.0,
        };
        let params = graphics::ShaderParamsBuilder::new(&stripes).build(ctx);
        let gui = Gui::new(ctx);
        Ok(MainState {
            stripes,
            shader,
            params,
            gui,
        })
    }
}

impl event::EventHandler for MainState {
    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        _input: ggez::input::keyboard::KeyInput,
        _repeated: bool,
    ) -> Result<(), ggez::GameError> {
        Ok(())
    }

    fn resize_event(&mut self, _: &mut Context, width: f32, height: f32) -> GameResult {
        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let gui_ctx = self.gui.ctx();

        egui::Window::new("UI").show(&gui_ctx, |ui| {
            ui.add(egui::Label::new(
                "Try to keep values to a only two points of precision ie 0.00, these are also all not in pixels but instead based on a percentage of the size",
            ));
            ui.horizontal(|ui| {
                ui.add(egui::Label::new("strip size:"));
                ui.add(egui::Slider::new(&mut self.stripes.stripe_size, 0.01..=5.0));
            });
            ui.horizontal(|ui| {
                ui.add(egui::Label::new("border size:"));
                ui.add(egui::Slider::new(&mut self.stripes.border_size, 0.01..=0.5));
            });
            ui.horizontal(|ui| {
                ui.add(egui::Label::new("stroke size:"));
                ui.add(egui::Slider::new(&mut self.stripes.stroke_size, 0.01..=0.25));
            });
            ui.horizontal(|ui| {
                ui.add(egui::Label::new("frequency:"));
                ui.add(egui::Slider::new(&mut self.stripes.freq, 0.01..=50.0));
            });
            ui.horizontal(|ui| {
                ui.add(egui::Label::new("speed:"));
                ui.add(egui::Slider::new(&mut self.stripes.speed, 0.01..=5.0));
            });
            ui.horizontal(|ui| {
                ui.add(egui::Label::new("tilt(in degrees):"));
                if ui.add(egui::Slider::new(&mut self.stripes.tilt, -180.0..=180.0)).changed() {
                    self.stripes.tilt = self.stripes.tilt.to_radians();
                }
            });
        });
        self.gui.update(ctx);

        self.stripes.time = ctx.time.time_since_start().as_secs_f32();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::from_rgba(255, 255, 255, 255));

        self.params.set_uniforms(ctx, &self.stripes);
        canvas.set_shader(&self.shader);
        canvas.set_shader_params(&self.params);
        canvas.draw(
            &Quad,
            DrawParam::default()
                .scale(Vec2::splat(100.0))
                .dest(Vec2::splat(100.0)),
        );
        canvas.set_default_shader();
        self.gui.draw(&mut canvas, DrawParam::default());
        canvas.finish(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("stripes", "ggez").add_resource_path(resource_dir);
    let (mut ctx, event_loop) = cb.build()?;

    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
