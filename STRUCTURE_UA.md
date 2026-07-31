gospelio/
├── Cargo.toml
├── README.md
├── assets/
│   └── icon.png             # Іконка для трею та вікна
└── src/
    ├── main.rs              # Точка входу, ініціалізація додатка
    ├── app.rs               # Головний UI (egui/eframe)
    ├── config.rs            # Завантаження та збереження config.toml
    ├── audio/               # Модуль відтворення та мережі
    │   ├── mod.rs
    │   ├── player.rs        # Автономний аудіо-рушій (rodio / symphonia)
    │   ├── stream.rs        # Завантаження потоку з мережі
    │   └── icy.rs           # Парсер ICY-метаданих (назва треків)
    └── tray/                # Модуль системного трею
        ├── mod.rs
        └── sni.rs           # Реалізація StatusNotifierItem (ksni / tray-icon)
