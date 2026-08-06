mod app;
mod config;
mod audio;

use app::GospelioApp;
use eframe::egui;
//use config::AppConfig;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 500.0]) // Розміри вікна
            .with_title("Gospelio Radio"),
        ..Default::default()
    };
    
    eframe::run_native(
            "Gospelio",
            options,
            Box::new(|cc| Box::new(GospelioApp::new(cc))),
        )
}

//    println!("Завантаження конфігурації Gospelio...");
//
//    let config = AppConfig::load();
//
//    println!("Поточна гучність: {}%", config.volume * 100.0);
//    println!("Згортати в трей при закритті {}", config.minimize_to_tray_on_close);
//    println!("Збереження станцій: {}", config.stations.len());
//
//  Перевірка запису
//    config.save();
//    println!("Конфіг успішно перевірено та збереження!");
//}
