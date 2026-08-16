// --src/audio.rs

use std::io::{BufReader, Read, Seek, SeekFrom, Result as IoResult, Error, ErrorKind};
use std::sync::mpsc::{self, Sender};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use rodio::{Decoder, OutputStream, Sink};

//Обгортка для мережевих потоків, які не підтримують перемотку (Seek)
struct NoSeek<R> {
    inner: R,
    pos: u64,
} // <-- struct NoSeek

impl<R> NoSeek<R> {
    fn new(inner: R) -> Self{
        Self { inner, pos: 0 }
    } // <-- fn new
} // impl NoSeek

impl<R: Read> Read for NoSeek<R> {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        let bytes_read = self.inner.read(buf)?;
        self.pos += bytes_read as u64;
        Ok(bytes_read)
    }// <-- fn read
} // <-- impl Read for NoSeek


impl <R: Read> Seek for NoSeek<R> {
    fn seek(&mut self, pos: SeekFrom) -> IoResult<u64> {
        let target = match pos {
		SeekFrom::Start(p) => p,
		SeekFrom::Current(n) => {
			if n >=0 {
				self.pos.saturating_add(n as u64)
			} else {
				self.pos.saturating_sub((-n) as u64)
			} //<-- if n
		}//<--SeekFrom::Current(n)
		SeekFrom::End(_) => return Ok(self.pos),
	};//<-- let target match pos
	if target > self.pos {
		let mut diff = target -self.pos;
		let mut buf = [0u8; 1024];
		while diff > 0 {
			let to_read = (diff as usize).min(buf.len());
			let read = self.inner.read(&mut buf[..to_read])?;
			if read == 0 {
				break;
			}//<-- if read == 0
			self.pos += read as u64;
			diff -= read as u64;
		} // <--while diff > 0
	}//<-- if target
	Ok(self.pos)
    } // <-- fn seek
} // <-- impl Read for NoSeek

/*impl <R> Seek for NoSeek<R> {
    fn seek(&mut self, pos: SeekFrom) -> IoResult<u64> {
        match pos {
            SeekFrom::Current(0) => Ok(self.pos),
            _ => Err(Error::new(
                    ErrorKind::Unsupported, "Інтернет потік не підтримує перемотку",
                    )),
        } // <-- match pos
    } // <-- fn seek
} // <-- impl Read for NoSeek
*/

/// Внутрішній перелік команд для аудіопотоку
enum AudioCommand {
    Play(String),
    Stop,
    SetVolume(f32),
}// <-- enum AudioCommand

pub struct AudioPlayer {
	tx: Sender<AudioCommand>,
} //<--pub struct AudioPlayer 

impl AudioPlayer {
// Ініціалізує аудіопристрій та запускає фоновий потік обробки відтворення
	pub fn new() -> Option<Self> {
		let (tx, rx) = mpsc::channel::<AudioCommand>();
		
		// Фоновий потік для управління аудіосистемою та мережею
		thread::spawn(move || {
			let(_stream, stream_handle) = match OutputStream::try_default() {
				Ok(val) => val,
				Err(e) => {
					eprintln!("Помилка ініціалізації аудіопристрою: {e}");
					return;
				} //<-- Err(e)

			}; // <-- let(_stream, stream_handle)
			let mut current_sink: Option<Arc<Sink>> = None;
			let mut current_volume: f32 = 0.8; 

			while let Ok(cmd) =rx.recv() {
				match cmd {
					AudioCommand::Play(url) => {
						if let Some(sink) = current_sink.take() {
							sink.stop();
						}// <--if let Some(sink)
						
						let sink = match Sink::try_new(&stream_handle) {
							Ok(s) => Arc::new(s),
							Err(e) => {
								eprintln!("Не вдалося створити Sink: {e}");
								continue;
							} // <--Err(e)
						}; //<--let sink = match Sink::try_new

						sink.set_volume(current_volume);
						let sink_handle = Arc::clone(&sink);
						
						thread::spawn(move || {
							let client = reqwest::blocking::Client::builder()
								.timeout(Duration::from_secs(10))
								.build();
							let response = match client {
								Ok(c) => c.get(&url).send(),
								Err(e) => {
									eprintln!("Помилка створення HTTP клієнта: {e}");
									return;
								}//<--Err(e)
							}; //<--let response
							
							match response {
								Ok(res) if res.status().is_success() => {
									let buffered_stream = BufReader::with_capacity(64 * 1024, res);
									let seekable_stream = NoSeek::new(buffered_stream);
									match Decoder::new(seekable_stream) {
										Ok(source) => {
											sink_handle.append(source);
											sink_handle.sleep_until_end();

										} //<-- Ok(source)
										Err(e) => eprintln!("Помилка декодування аудіопотоку: {e}"),	
									}//<-- match Decoder
								}//<--Ok(res) if res.tatus
							
								Ok(res) => eprintln!("Помилка сервера HTTP: status{}", res.status()),
								Err(e) => eprintln!("Помилка з'єднання з радіостанцією: {e}"),
							} //<--match response
						}); //<--thread::spawn
						current_sink = Some(sink);
					} //<--AudioCommand::Play
					
					AudioCommand::Stop => {
						if let Some(sink) = current_sink.take() {
							sink.stop();
						}//<--if let Some(sink)
					} //AudioCommand::Stop 
					
					AudioCommand::SetVolume(volume) => {
						current_volume = volume.clamp(0.0, 1.0);
						if let Some(ref sink) = current_sink {
							sink.set_volume(current_volume);
						} //<--if let Some(ref sink)
					}//<--AudioCommand::SetVolume(volume)
				} // <--match cmd 
			} // <--while let Ok(cmd)


		}); // <--thread::spawn

		Some(Self { tx })
	} // <--pub fn new
	pub fn play_url(&self, url: &str) {
		let _ = self.tx.send(AudioCommand::Play(url.to_string()));
	}//<-- fn play_url

	pub fn stop(&self) {
		let _ = self.tx.send(AudioCommand::Stop);
	}//<--fn stop

	pub fn set_volume(&self, volume: f32) {
		let _ = self.tx.send(AudioCommand::SetVolume(volume));
	} // <-- fn set_volume
} // <-- impl AudioPlayer 

