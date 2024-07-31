mod gui;

use egui;
use gui::GameWindow;

fn main() {
    let egui_ctx = egui::Context::default();
    egui_ctx.run

    let mut window = GameWindow::new(egui_ctx, "Hello, world!".to_string(), (800.0, 600.0));
    window.render(0.0);
}
