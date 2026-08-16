// src/ui/volume_slider.rs

use egui::Ui;

// Окремий файл для слайдера гучності
pub fn render_volume_slider(ui: &mut Ui, volume: &mut f32) -> bool{
    ui.add(egui::Slider::new(volume, 0.0..=1.0).text("Гучність")).changed()
} // <--  fn render_volume_slider
