// src/ui/station_button.rs
use egui::Ui;

//pub struct Station
pub fn render_station_button(ui: &mut Ui, name: &str) -> bool {
    ui.button(format!("▶ {}", name)).clicked()
} //<-- fn render_station_button 
