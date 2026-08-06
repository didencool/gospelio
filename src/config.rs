// --- src/config.rs ---
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Station {
    pub name: String,
    pub url: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub volume: f32,
    pub volume_safety_limit: f32,
    pub volume_step_normal: f32,
    pub volume_step_fine: f32,
    
    // Нові налаштування для трею
    pub minimize_to_tray_on_close: bool,
    pub start_minimized: bool,

    pub last_station_url: String,
    pub stations: Vec<Station>,
}


// Значення за замовчуванням (якщо файлу config.toml ще немає)
impl Default for AppConfig {
    fn default() -> Self {
        Self {
            volume: 0.5,
            volume_safety_limit: 0.70,
            volume_step_normal: 0.02,
            volume_step_fine: 0.01,
            
            minimize_to_tray_on_close: true, 
            start_minimized: false, // Можливо пізніше можна буде стартувати апку дразу в треї
            
            last_station_url: "http://ua.jtradio.net/live192".to_string(),
            stations: vec![ 
                Station {
                    name: "JT Radio 192Kbps".to_string(),
                    url: "http://ua.jtradio.net/live192".to_string(),
                    },
                Station {
                    name: "JT Radio 48Kbps".to_string(),
                    url: "http://ua.jtradio.net/live48".to_string(),
                    },
                ],
            }
        }
    }

impl AppConfig {
    /// Повертаємо шлях до директорії конфігурації в моїй ОС (~/.config/gospelio/config.toml)
    fn get_config_path() -> Option<PathBuf> {
        if let Some(proj_dirs) = ProjectDirs::from("org", "gospelio", "gospelio") {
            let config_dir = proj_dirs.config_dir();
            return Some(config_dir.join("config.toml"));
            }
        None
        }

    /// Завантажуємо файл конфігурації з диска. Якщо файлу немає або він пошкоджений - повертаємо дефолтний
    pub fn load() -> Self {
        if let Some(path) = Self::get_config_path() {
            if path.exists() {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(config) = toml::from_str::<AppConfig>(&content) {
                        return config;
                        }
                    }
                }
            }

    // Якщо файлу не існує, створюємо за замовченням і одразу зберігаємо його
    let default_config = Self::default();
    default_config.save();
    default_config
    }

    /// Зберігаємо поточні налаштування у файл ~/.config/gospelio/config.toml
    pub fn save(&self) {
        if let Some(path) = Self::get_config_path() {
           if let Some(parent) = path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            if let Ok(toml_string) = toml::to_string_pretty(self) {
                let _ = fs::write(path, toml_string);
            }
        }
    }
}
