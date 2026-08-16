// src/ui/stop_button.rs
use egui::Ui;

pub fn render_stop_button(ui: &mut Ui) -> bool {
    ui.button("⏹ Зупинити").clicked() 
} // <--  fn.button
