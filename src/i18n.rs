//! src/i18n.rs
//! 

use i18n_embed::{
    fluent::{fluent_language_loader, FluentLanguageLoader},
    unic_langid::LanguageIdentifier,
    DesktopLanguageRequester, LanguageLoader,
};
use rust_embed::RustEmbed;
use std::collections::HashMap;
use std::sync::LazyLock;

// 1. Вбудовуємо .ftl файли в бінарний файл під час компіляції
#[derive(RustEmbed)]
#[folder = "src/locales/"]
struct Localizations;

// 2. Глобальний лоадер для доступу до текстів з будь-якого місця програми
pub static LANGUAGE_LOADER: LazyLock<FluentLanguageLoader> = LazyLock::new(|| {
    let loader = fluent_language_loader!();
    // Встановлюємо дефолтну мову (fallback)
    let requested_languages = vec!["uk".parse::<LanguageIdentifier>().unwrap()];
    
    let _ = loader.load_languages(&Localizations, &requested_languages);
    loader
});

/// Ініціалізація вибору мови з урахуванням системної змінної (LANG / LC_MESSAGES)
pub fn init_i18n() {
    let requested_languages = DesktopLanguageRequester::requested_languages();
    
    // `select` підбирає з мов системи лише ті, для яких є .ftl файли
    // (напр. en-US -> en) і завантажує їх. Якщо жодної немає — лишається фолбек (uk).
    // Пряме load_languages з мовою без .ftl файлу призводить до паніки в i18n-embed.
    if let Err(e) = i18n_embed::select(&*LANGUAGE_LOADER, &Localizations, &requested_languages) {
        eprintln!("Помилка завантаження мови: {e}");
    }
}

/// Допоміжна макро-функція або обгортка для зручного отримання тексту
pub fn tr(key: &str) -> String {
    LANGUAGE_LOADER.get(key)
}

/// Текст з параметрами, напр. `tr_args("status-playing", &[("name", "JT Radio")])`
pub fn tr_args(key: &str, args: &[(&str, &str)]) -> String {
    let args: HashMap<&str, &str> = args.iter().copied().collect();
    LANGUAGE_LOADER.get_args(key, args)
}
