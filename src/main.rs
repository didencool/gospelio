//! src/main.rs
//! Project: Gospelio Radio Player

mod audio;
mod config;
mod i18n;

use std::cell::RefCell;
use std::rc::Rc;

use slint::{ComponentHandle, ModelRc, VecModel};

use audio::noseek::AudioPlayer;
use config::AppConfig;
use i18n::{tr, tr_args};

// Підключає код, згенерований з src/ui/appwindow.slint (MainWindow, Tr, StationItem...)
slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    i18n::init_i18n();

    // Rc<RefCell<..>> — щоб кілька callback'ів UI могли читати й змінювати конфіг
    let config = Rc::new(RefCell::new(AppConfig::load()));
    let player = Rc::new(AudioPlayer::new().expect("Не вдалося запустити аудіоплеєр"));

    let ui = MainWindow::new()?;

    // Тексти інтерфейсу з .ftl файлів
    let texts = ui.global::<Tr>();
    texts.set_app_title(tr("app-title").into());
    texts.set_stations_list(tr("stations-list").into());
    texts.set_stop(tr("stop").into());
    texts.set_volume(tr("volume").into());

    // Початковий стан з конфігу
    {
        let config = config.borrow();
        let stations: Vec<StationItem> = config
            .stations
            .iter()
            .map(|s| StationItem { name: s.name.as_str().into() })
            .collect();
        ui.set_stations(ModelRc::new(VecModel::from(stations)));

        let volume_percent = (config.volume * 100.0).round() as i32;
        ui.set_volume(volume_percent);
        player.set_volume(config.volume);
    }
    ui.set_status(tr("status-ready").into());

    // Клік по станції — відтворюємо її
    ui.on_play_station({
        let ui = ui.as_weak();
        let config = Rc::clone(&config);
        let player = Rc::clone(&player);
        move |index| {
            let ui = ui.unwrap();
            let mut config = config.borrow_mut();
            let Some(station) = config.stations.get(index as usize).cloned() else {
                return;
            };

            player.play_url(&station.url);
            ui.set_playing_index(index);
            ui.set_status(tr_args("status-playing", &[("name", &station.name)]).into());

            config.last_station_url = station.url;
            config.save();
        }
    });

    // Кнопка «Стоп»
    ui.on_stop({
        let ui = ui.as_weak();
        let player = Rc::clone(&player);
        move || {
            let ui = ui.unwrap();
            player.stop();
            ui.set_playing_index(-1);
            ui.set_status(tr("status-stopped").into());
        }
    });

    // Повзунок рухається — одразу змінюємо гучність
    ui.on_volume_changed({
        let player = Rc::clone(&player);
        move |percent| player.set_volume(percent as f32 / 100.0)
    });

    // Повзунок відпустили — зберігаємо в конфіг (а не на кожен піксель руху)
    ui.on_volume_released({
        let config = Rc::clone(&config);
        move |percent| {
            let mut config = config.borrow_mut();
            config.volume = percent as f32 / 100.0;
            config.save();
        }
    });

    ui.run()
}
