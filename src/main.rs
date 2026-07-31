mod config;
use config::AppConfig;

fn main() {
    println!("Завантаження конфігурації Gospelio...");

    let mut config = AppConfig::load();

    println!("Поточна гучність: {}%", config.volume * 100.0);
    println!("Згортати в трей при закритті {}", config.minimize_to_tray_on_close);
    println!("Збереження станцій: {}", config.stations.len());

    // Перевірка запису
    config.save();
    println!("Конфіг успішно перевірено та збереження!");
}
