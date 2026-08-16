// --- src/app.rs ---
use crate::audio::AudioPlayer;
use crate::config::AppConfig;
use crate::ui::main_view::render_main_view;
//use crate::config::Station;
//use eframe::egui;


#[derive(Debug, Clone, PartialEq)]
//#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum PlaybackState {
    Stopped,
    Playing,
    Buffering,
    Error(String),
}

pub struct GospelioApp {
    pub config: AppConfig,
    pub player: Option<AudioPlayer>,
    pub playback_state: PlaybackState,
    pub current_station: Option<String>,
//    pub is_editing_stations: bool,
    // Стан вікна
//    pub is_window_visible: bool,
    pub status_text: String,
//    pub volume_slider_active: bool,
//    pub new_station_name: String,
//    pub new_station_url: String,
//    pub track_title: String,
    }

impl GospelioApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Завантажуємо конфіг під час старту
        // let config = AppConfig::load();
        // let player = AudioPlayer::new();
        
        Self {
                config: AppConfig::load(),
                player: AudioPlayer::new(),
                playback_state: PlaybackState::Stopped,
                current_station: None,
                //is_editing_stations: false,
                // is_window_visible: true,
                status_text: "Готовий до роботи".to_string(),
                // volume_slider_active: false,
                // new_station_name: String::new(),
                // new_station_url:String::new(),
                // track_title: "Немає інформації про трек".to_string(),
            }
        }
    }

impl eframe::App for GospelioApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Панель Заг0ловок
        // egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        //    ui.heading("📻 Gospelio Radio Player");
   // }); // <-- egui::top...show

    // Центральна панель з елементами керування
    // egui::CentralPanel::default().show(ctx, |ui| {
        // TODO implement multilingual support
    //    ui.label(format!("Збарежено станцій: {}", self.config.stations.len()));
    //    ui.separator();

        // Слайдер гучності
    //    ui.horizontal(|ui| {
    //        ui.label("Гучність:");
    //            if ui.add(egui::Slider::new(&mut self.config.volume, 0.0..=1.0).text("%")).changed() {
                    // Зберігаємо конфіг при зміні гучності
    //                self.config.save();
    //                }
    //            }); // <-- Закриваємо ui.horizontal

    //    ui.separator();

        // Список станцій
    //    ui.label("Список станцій:");
    //    for station in &self.config.stations{
    //        if ui.button(format!("▶ {}", station.name)).clicked() {
    //            println!("Обрано станцію: {}", station.url);

                // Стан Додатку
    //            self.current_station = Some(station.name.clone());
    //            self.playback_state = PlaybackState::Playing;
    //            self.status_text = format!("Грає: {}", station.name);

            //Відтворення
    //                if let Some(ref player) = self.player {
    //                player.play_url(&station.url);
    //                    }
    //                } // <-- Закриваємо ui.button
    //            } // <-- Закриття циклу for 
    //    }); // <-- Закриття CentralPanel
    render_main_view(self, ctx);

    } // <-- Закриття функції update
} // <-- Закриття impl eframe::App
