# 📻 Cross-Platform Stream Radio Player

A lightweight, standalone, and cross-platform radio player written in **Rust**.

The project is built with a focus on minimalism, low resource consumption, and seamless integration into Linux environments (specifically **Sway / Waybar** via Freedesktop StatusNotifierItem), with support for building on **Windows** and **Android**.

---

## ⚙️ Key Features

* **Standalone Audio Engine:** Stream playback without external dependencies (no `mpv` or `vlc` required).
* **Format Support:** MP3, AAC, OGG (Vorbis / Opus), FLAC.
* **System Tray Integration (Waybar / SNI):**
  * **LMB:** Show / Hide the main window.
  * **MMB:** Quick Play / Pause.
  * **RMB:** Context menu (Station selection, settings, exit).
  * **Mouse Scroll:** Volume control with a safety limit.
* **Hearing Protection (Safety Volume Limit):**
  * Smooth volume adjustment with a **2%** step.
  * Safety barrier set at **70%** (pauses automatic scrolling adjustment).
  * Fine step of **1%** once the threshold is explicitly crossed.
* **ICY Metadata:** Real-time stream parsing for track and artist information.
* **Station Manager:** Save and select radio stations via a user-friendly ComboBox.

---

## 🎛️ Variables & Configuration (`config.toml`)

The application stores its settings in a configuration file (on Linux: `~/.config/radio-player/config.toml`).

### Configuration Structure and Variables:

| Variable | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `volume` | `f32` | `0.5` | Current volume level (from `0.0` to `1.0`). |
| `volume_safety_limit` | `f32` | `0.7` | Safe volume threshold (70%). |
| `volume_step_normal` | `f32` | `0.02` | Normal volume adjustment step (2%). |
| `volume_step_fine` | `f32` | `0.01` | Fine adjustment step above the safety limit (1%). |
| `last_station_url` | `String` | `""` | URL of the last played station. |
| `stations` | `Array` | `[...]` | List of saved radio stations (`name`, `url`). |

#### Example `config.toml`:
```toml
volume = 0.45
volume_safety_limit = 0.70
volume_step_normal = 0.02
volume_step_fine = 0.01
last_station_url = "[http://ua.jtradio.net/live192](http://ua.jtradio.net/live192)"

[[stations]]
name = "JT Radio"
url = "[http://ua.jtradio.net/live192](http://ua.jtradio.net/live192)"

[[stations]]
name = "Radio Bayrak"
url = "[https://example.com/stream.ogg](https://example.com/stream.ogg)"

Структура UI Проекту
src/
├── main.rs                 # Точка входу в програму
├── app.rs                  # Головна структура стану GospelioApp
├── audio.rs                # Внутрішня робота: відтворення та HTTP-потік
├── config.rs               # Внутрішня робота: збереження та конфігурація
├── ui/                     # Модуль інтерфейсу
│   ├── mod.rs              # Декларація та експорт UI-компонентів
│   ├── main_view.rs        # Головне вікно інтерфейсу (складання блоків)
│   ├── stop_button.rs      # Кнопка зупинки
│   ├── station_button.rs   # Кнопка вибору станції
│   └── volume_slider.rs    # Слайдер гучності
