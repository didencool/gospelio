//! src/main.rs
//!

mod audio;
mod i18n;

// Завдяки `pub use volume::VolumeControl;` у mod.rs, імпорт короткий:
use audio::VolumeControl;

fn main() {
    i18n::init_i18n();

    let mut volume = VolumeControl::new(0.5, 0.7);
    println!("Gospelio запуск");
    println!("Мова додатка: {}", i18n::tr("app-title"));
    println!("Початкова гучність: {:.0}%", volume.value() * 100.0);

    volume.increase();
    println!("Гучність після збільшення: {:.0}%", volume.value() * 100.0);
}
