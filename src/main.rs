#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod tetris;

use std::time::Duration;

use eframe::{
    egui::{self, Key},
    epaint::Stroke,
};
use tetris::GameLoop;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 620.0]),
        ..Default::default()
    };
    eframe::run_native(
        "egui TETRIS",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::<TetrisApp>::default()
        }),
    )
}

struct TetrisApp {
    game: GameLoop,
}

impl Default for TetrisApp {
    fn default() -> Self {
        Self {
            game: GameLoop::new(),
        }
    }
}

impl eframe::App for TetrisApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint_after(Duration::from_millis(1000 / 60));
        self.game.tick();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("TETRIS");

            self.render_game_map(ui);

            self.inputs(ctx);
        });
    }
}

impl TetrisApp {
    fn inputs(&mut self, ctx: &egui::Context) {
        if ctx.input(|i| i.key_down(Key::ArrowLeft)) {
            self.game.input(tetris::Input::Left);
        }
        if ctx.input(|i| i.key_down(Key::ArrowRight)) {
            self.game.input(tetris::Input::Right);
        }
        if ctx.input(|i| i.key_down(Key::ArrowDown)) {
            self.game.input(tetris::Input::Down);
        }
        if ctx.input(|i| i.key_pressed(Key::Z)) {
            self.game.input(tetris::Input::RotateCounterClockwise);
        }
        if ctx.input(|i| i.key_pressed(Key::X)) {
            self.game.input(tetris::Input::RotateClockwise);
        }
    }
    fn render_game_map(&self, ui: &mut egui::Ui) {
        for y in 0..20 {
            for x in 0..10 {
                let color = match self.game.map[y][x] {
                    Some(0) => egui::Color32::from_rgb(255, 255, 255),
                    Some(1) => egui::Color32::from_rgb(255, 0, 0),
                    Some(2) => egui::Color32::from_rgb(0, 255, 0),
                    Some(3) => egui::Color32::from_rgb(0, 0, 255),
                    Some(4) => egui::Color32::from_rgb(255, 255, 0),
                    Some(5) => egui::Color32::from_rgb(255, 0, 255),
                    Some(6) => egui::Color32::from_rgb(0, 255, 255),
                    _ => egui::Color32::from_rgb(0, 0, 0),
                };

                ui.painter().rect(
                    self.render_coord(x, y, 10.0, 10.0, 30.0),
                    3.0,
                    color,
                    Stroke::new(0.0, color),
                );
            }
        }
    }
    fn render_coord(&self, x: usize, y: usize, x_off: f32, y_off: f32, n: f32) -> egui::Rect {
        return egui::Rect::from_min_size(
            egui::Pos2::new(x as f32 * n + x_off, y as f32 * n + y_off),
            egui::Vec2::new(n, n),
        );
    }
}
