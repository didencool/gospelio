//! Path: src/audio/noseek.rs
//! Project: Gospelio Radio Player

use std::io::{BufRead, BufReader, Error as IoError, ErrorKind, Read, Result as IoResult, Seek, SeekFrom};
use std::sync::mpsc::{self, Sender};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use rodio::{Decoder, DeviceSinkBuilder, Player};

use super::command::AudioCommand;

// Обгортка для мережевих потоків, які не підтримують перемотку (Seek).
// Дозволяє тільки "перемотку вперед" (пропуск байтів) — цього достатньо декодеру
// для пропуску ID3-тегів. Назад або від кінця перемотати неможливо.
struct NoSeek<R> {
    inner: R,
    pos: u64,
} // <-- struct NoSeek

impl<R> NoSeek<R> {
    fn new(inner: R) -> Self {
        Self { inner, pos: 0 }
    } // <-- fn new
} // <-- impl NoSeek

impl<R: Read> Read for NoSeek<R> {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        let bytes_read = self.inner.read(buf)?;
        self.pos += bytes_read as u64;
        Ok(bytes_read)
    } // <-- fn read
} // <-- impl Read for NoSeek

impl<R: Read> Seek for NoSeek<R> {
    fn seek(&mut self, pos: SeekFrom) -> IoResult<u64> {
        let target = match pos {
            SeekFrom::Start(p) => p,
            SeekFrom::Current(n) if n >= 0 => self.pos.saturating_add(n as u64),
            SeekFrom::Current(_) | SeekFrom::End(_) => {
                return Err(IoError::new(
                    ErrorKind::Unsupported,
                    "мережевий потік не підтримує перемотку назад",
                ));
            }
        }; // <-- let target

        if target < self.pos {
            return Err(IoError::new(
                ErrorKind::Unsupported,
                "мережевий потік не підтримує перемотку назад",
            ));
        }

        // Перемотка вперед: просто читаємо та відкидаємо зайві байти
        let mut diff = target - self.pos;
        let mut buf = [0u8; 1024];
        while diff > 0 {
            let to_read = (diff as usize).min(buf.len());
            let read = self.inner.read(&mut buf[..to_read])?;
            if read == 0 {
                break;
            }
            self.pos += read as u64;
            diff -= read as u64;
        } // <-- while diff > 0
        Ok(self.pos)
    } // <-- fn seek
} // <-- impl Seek for NoSeek

pub struct AudioPlayer {
    tx: Sender<AudioCommand>,
} // <-- pub struct AudioPlayer

impl AudioPlayer {
    // Ініціалізує аудіопристрій та запускає фоновий потік обробки відтворення
    pub fn new() -> Option<Self> {
        let (tx, rx) = mpsc::channel::<AudioCommand>();

        // Фоновий потік для управління аудіосистемою та мережею
        thread::spawn(move || {
            // Аудіовихід має жити, поки працює цей потік
            let mut device_sink = match DeviceSinkBuilder::open_default_sink() {
                Ok(sink) => sink,
                Err(e) => {
                    eprintln!("Помилка ініціалізації аудіопристрою: {e}");
                    return;
                }
            }; // <-- let device_sink
            // Не друкувати попередження про закриття аудіовиходу під час виходу з програми
            device_sink.log_on_drop(false);

            let mut current_player: Option<Arc<Player>> = None;
            let mut current_volume: f32 = 0.8;

            while let Ok(cmd) = rx.recv() {
                match cmd {
                    AudioCommand::Play(url) => {
                        if let Some(player) = current_player.take() {
                            player.stop();
                        }

                        let player = Arc::new(Player::connect_new(device_sink.mixer()));
                        player.set_volume(current_volume);
                        let player_handle = Arc::clone(&player);

                        thread::spawn(move || stream_url(&url, &player_handle));
                        current_player = Some(player);
                    } // <-- AudioCommand::Play

                    AudioCommand::Stop => {
                        if let Some(player) = current_player.take() {
                            player.stop();
                        }
                    } // <-- AudioCommand::Stop

                    AudioCommand::SetVolume(volume) => {
                        current_volume = volume.clamp(0.0, 1.0);
                        if let Some(ref player) = current_player {
                            player.set_volume(current_volume);
                        }
                    } // <-- AudioCommand::SetVolume
                } // <-- match cmd
            } // <-- while let Ok(cmd)
        }); // <-- thread::spawn

        Some(Self { tx })
    } // <-- pub fn new

    pub fn play_url(&self, url: &str) {
        let _ = self.tx.send(AudioCommand::Play(url.to_string()));
    } // <-- fn play_url

    pub fn stop(&self) {
        let _ = self.tx.send(AudioCommand::Stop);
    } // <-- fn stop

    pub fn set_volume(&self, volume: f32) {
        let _ = self.tx.send(AudioCommand::SetVolume(volume));
    } // <-- fn set_volume
} // <-- impl AudioPlayer

/// Завантажує радіопотік по HTTP і відтворює його, поки `player` не зупинять.
fn stream_url(url: &str, player: &Player) {
    // Тайм-аут лише на з'єднання та на кожне читання. Загальний тайм-аут
    // ставити не можна: радіопотік нескінченний і обірвався б через N секунд.
    let agent: ureq::Agent = ureq::Agent::config_builder()
        .timeout_connect(Some(Duration::from_secs(10)))
        .timeout_per_read(Some(Duration::from_secs(15)))
        .build()
        .into();

    let response = match agent.get(url).call() {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Помилка з'єднання з радіостанцією: {e}");
            return;
        }
    }; // <-- let response

    let reader = response.into_body().into_reader();

    // 1. Створюємо буфер
    let mut buffered_stream = BufReader::with_capacity(64 * 1024, reader);
    // 2. Примусово прогріваємо мережевий буфер перед передачею в декодер
    if let Ok(initial_bytes) = buffered_stream.fill_buf() {
        println!("Буфер прогріто! Завантажено перші {} байтів", initial_bytes.len());
    }
    // 3. Передаємо прогрітий буфер у NoSeek і явно кажемо декодеру, що перемотки немає
    let source = Decoder::builder()
        .with_data(NoSeek::new(buffered_stream))
        .with_seekable(false)
        .build();

    match source {
        Ok(source) => {
            player.append(source);
            player.sleep_until_end();
        }
        Err(e) => eprintln!("Помилка декодування аудіопотоку: {e}"),
    } // <-- match source
} // <-- fn stream_url
