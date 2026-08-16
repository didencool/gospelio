use egui::Context;
use crate::app::{GospelioApp, PlaybackState};
use crate::ui::station_button::render_station_button;
use crate::ui::stop_button::render_stop_button;
use crate::ui::volume_slider::render_volume_slider;

/// Окремий файл, який повністю відповідає за компонування інтерфейсу
pub fn render_main_view(app: &mut GospelioApp, ctx: &Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Gospelio Audio Player");
        ui.label(&app.status_text);
        ui.add_space(10.0);

        // Блок керування
        ui.horizontal(|ui| {
            if matches!(app.playback_state, PlaybackState::Playing | PlaybackState::Buffering) {
                if render_stop_button(ui) {
                    if let Some(ref player) = app.player {
                        player.stop();
                    } // <-- if let Some
                    app.playback_state = PlaybackState::Stopped;
                    app.status_text = "Зупинено".to_string();
                    app.current_station = None;
                } // <-- if render_stop_button
            } // <-- if matches

            // Слайдер гучності (приклад підключення)
            let mut temp_volume = 0.8; // Можна додати поле volume в GospelioApp
            if render_volume_slider(ui, &mut temp_volume) {
                if let Some(ref player) = app.player {
                    player.set_volume(temp_volume);
                } // <-- if let Some
            } // <-- if render_volume_slider
        }); // <-- ui.horizontal

        ui.add_space(10.0);

        // Список станцій
        ui.label("Список станцій:");
        for station in app.config.stations.clone() {
            if render_station_button(ui, &station.name) {
                println!("Обрано станцію: {}", station.url);

                app.current_station = Some(station.name.clone());
                app.playback_state = PlaybackState::Playing;
                app.status_text = format!("Грає: {}", station.name);

                if let Some(ref player) = app.player {
                    //player.play_url();
                    player.play_url(&station.url);
                } // <-- if let Some
            } // <-- if render_station_button
        } // <-- for station
    }); // <-- CentralPanel
} // <-- fn render_main_view
