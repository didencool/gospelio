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
// Поля, яких немає у файлі (напр. додані в новій версії), беруться з Default,
// тож старий config.toml не "ламається" і не перезаписується
#[serde(default)]
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
	pub buffer_size_kb: usize,
}

// Значення за замовчуванням (якщо файлу config.toml ще немає)
impl Default for AppConfig {
    fn default() -> Self {
        Self {
            volume: 0.5,
            volume_safety_limit: 0.70,
            volume_step_normal: 0.02,
            volume_step_fine: 0.01,
            buffer_size_kb: 1024,
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
        let Some(path) = Self::get_config_path() else {
            return Self::default();
        };

        if path.exists() {
            // Файл є, але пошкоджений — НЕ перезаписуємо його, щоб не втратити станції користувача
            return match fs::read_to_string(&path).map(|c| toml::from_str::<AppConfig>(&c)) {
                Ok(Ok(config)) => config,
                Ok(Err(e)) => {
                    eprintln!("Помилка у {}: {e}. Використовую налаштування за замовчуванням", path.display());
                    // Наступний save() перезапише файл, тож зберігаємо копію пошкодженого
                    let backup = path.with_extension("toml.bak");
                    if fs::copy(&path, &backup).is_ok() {
                        eprintln!("Копію пошкодженого файлу збережено у {}", backup.display());
                    }
                    Self::default()
                }
                Err(e) => {
                    eprintln!("Не вдалося прочитати {}: {e}", path.display());
                    Self::default()
                }
            };
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
