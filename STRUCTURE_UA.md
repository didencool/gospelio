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

////////////////////////////////

gospelio/
├── Cargo.toml
├── README.md
├── assets/                  # PNG іконки (play, pause)
├── locales/                 # Локалізація (Fluent)
│   ├── uk/
│   │   └── main.ftl
│   └── en/
│       └── main.ftl
└── src/
    ├── main.rs              # Ініціалізація додатку, точки входу
    ├── config.rs            # Збереження/завантаження конфігурації (TOML/JSON)
    ├── i18n.rs              # Модуль локалізації Fluent
    ├── audio/               # Логіка аудіо
    │   ├── mod.rs
    │   ├── player.rs        # Управління потоком (rodio / gstreamer)
    │   └── volume.rs        # Нелінійна регуляція гучності
    ├── tray/                # Системний трей (StatusNotifierItem для Wayland)
    │   ├── mod.rs
    │   └── events.rs        # Обробник подій (LMB, MMB, RMB, Scroll)
    └── ui/                  # Графічний інтерфейс (egui / slint)
        ├── mod.rs
        ├── settings.rs      # Вікно налаштувань (MMB)
        └── stations.rs      # Вікно вибору/додавання потоку (RMB)


