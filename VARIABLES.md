# Структура конфігурації Gospelio (`config.toml`)

У цьому документі описані всі змінні налаштувань додатка, їхні типи даних, значення за замовчуванням та призначення.

Файл конфігурації автоматично зберігається за шляхом:
`~/.config/gospelio/config.toml` (у Linux systems).

---

## 🔊 Налаштування гучності (Volume Settings)

| Змінна | Тип | Значення за замовчуванням | Опис |
| :--- | :--- | :--- | :--- |
| `volume` | `f32` | `0.5` (50%) | Поточний рівень гучності від `0.0` (тиша) до `1.0` (максимум). |
| `volume_safety_limit` | `f32` | `0.70` (70%) | Поріг безпечної гучності. Запобігає випадковому різкому ввімкненню занадто гучного звуку. |
| `volume_step_normal` | `f32` | `0.02` (2%) | Крок зміни гучності при звичайній прокрутці миші або натисканні клавіш `Up`/`Down`. |
| `volume_step_fine` | `f32` | `0.01` (1%) | Мікро-крок зміни гучності при точному налаштуванні (наприклад, із затиснутою клавішею `Shift`). |

---

## 🖥️ Поведінка вікна та трею (Window & System Tray)

| Змінна | Тип | Значення за замовчуванням | Опис |
| :--- | :--- | :--- | :--- |
| `minimize_to_tray_on_close` | `bool` | `true` | Якщо `true` — при натисканні на хрестик `[X]` вікно ховається в системний трей замість завершення програми. |
| `start_minimized` | `bool` | `false` | Якщо `true` — додаток запускається одразу згорнутим у системний трей без показу головного вікна. |

---

## 📻 Відтворення та станції (Playback & Stations)

| Змінна | Тип | Значення за замовчуванням | Опис |
| :--- | :--- | :--- | :--- |
| `last_station_url` | `String` | `"http://ua.jtradio.net/live192"` | URL-адреса радіостанції, яка відтворювалася востаннє перед закриттям програми. |
| `stations` | `Vec<Station>` | Список із 2-х станцій | Вектор (список) збережених користувачем радіостанцій. |

### Структура `Station` (елемент списку `stations`):

* **`name`** (`String`): Зрозуміла назва станції (наприклад, `"JT Radio 192Kbps"`).
* **`url`** (`String`): Пряме посилання на аудіопотік (наприклад, `"http://ua.jtradio.net/live192"`).

---

## 📄 Приклад згенерованого `config.toml`

```toml
volume = 0.5
volume_safety_limit = 0.7
volume_step_normal = 0.02
volume_step_fine = 0.01
minimize_to_tray_on_close = true
start_minimized = false
last_station_url = "[http://ua.jtradio.net/live192](http://ua.jtradio.net/live192)"

[[stations]]
name = "JT Radio 192Kbps"
url = "[http://ua.jtradio.net/live192](http://ua.jtradio.net/live192)"

[[stations]]
name = "JT Radio 48Kbps"
url = "[http://ua.jtradio.net/live48](http://ua.jtradio.net/live48)"
