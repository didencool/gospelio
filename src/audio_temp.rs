// -- 
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::io::{BufReader, Read, Seek, SeekFrom, Cursor};
//use std::io::{Read, Seek, SeekFrom};
use std::sync::mpsc::{self, sender};
//use std::sync::{Arc, Mutex};
use std::thread;

let mut initial_buffer = vec![0u8; 256 * 1024];
let mut bytes_read = 0;

while bytes_read < initial_buffer.len() {
    match response.read(&mut initial_buffer[bytes_read..]) {
        Ok(0) => break, // кінець потоку
        Ok(n) => bytes_read += n,
        Err(e) => break,
    } // <-- match response.read
} // <-- while bytes_read < initial_buffer.len








pub struct StreamReader<R> {
        inner: R,
}

impl<R: Read> Read for StreamReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.inner.read(buf)
    }
}

impl <R: Read> Seek for StreamReader<R> {
    fn seek(&mut self, _pos: SeekFrom) -> std::io::Result<u64> {
        Ok(0)
    }
}

pub struct AudioPlayer {
    _stream: OutputStream,
    _stream_handle: OutputStreamHandle,
    sink: Arc<Mutex<Sink>>,
}

impl AudioPlayer {
    pub fn new() -> Option<Self> {
        let (_stream, stream_handle) = OutputStream::try_default().ok()?;
        let sink = Sink::try_new(&stream_handle).ok()?;

        Some(Self {
            _stream,
            _stream_handle : stream_handle,
            sink: Arc::new(Mutex::new(sink)),
        }) //<-- Some
        //if let Ok((stream, stream_handle)) = OutputStream::try_default() {
        //    if let Ok(sink) = Sink::try_new(&stream_handle) {
        //        return Some(Self {
        //            _stream: stream,
        //            _stream_handle: stream_handle,
        //            sink: Arc::new(sink),
        //        });
        //    }
        //}
    
    //None
    } // <-- fn new
    
    pub fn play_url(&self, url: &str) {
        //self.sink.stop(); // Зупиняємо попередній потік

        let url_string = url.to_string();
        //let sink_clone = self.sink.clone();
        //let sink = Arc::clone(&self.sink);
        let sink_clone = Arc::clone(&self.sink);
        // Зупиняємо потік 
        if let Ok(sink) = self.sink.lock() {
            sink.stop(); 
        } //<-- if let Ok
        // Зупиняємо завантаження та декодування у фоновому потоці
        thread::spawn(move || {
            //////////////////////////////////////////////////////
            // ВІдкриваємо HTTP потік
            let response = match reqwest::blocking::get(&url_string[..]) {
                Ok(resp) => resp,
                Err(e) => {
                    eprintln!("Помилка підключення до URL{}: {}", url_string, e);
                    return;
                } // <-- Err(e)
            }; // <-- let response
            let stream_reader = StreamReader { inner: response};
            
            //\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\
            //Декодуємо аудіопотік
            match Decoder::new(stream_reader) {
                Ok(source) => {
                    if let Ok(sink) = sink_clone.lock() {
                        // Створюємо новий sink або очищуємо старий
                        sink.append(source);
                        sink.play();
                    } // <-- if let Ok(sink) = sink_clone.lock()
                } // <-- Ok(source)i
                Err(e) => {
                    eprintln!("Помилка декодування аудіо: {}", e);
                } // <-- Err
            } // <--  match Decoder::new       
        }); // <-- thread::spawn

        // Запускаємо фоновий потік для завантаження аудіо, щоб UI не "зависав"
        // std::thread::spawn(move || {
        //     if let Ok(response) = ureq::get(&url_string).call() {
        //         //let reader = BufReader::new(response.into_reader());
        //         let reader = response.into_reader();
        //         let stream_reader = StreamReader { inner: reader };
        //         let buf_reader = BufReader::new(stream_reader);
        //         if let Ok(source) = Decoder::new(buf_reader) {
        //             sink.append(source);
        //             sink.play();
        //         }
        //     }
        // });
    } // <-- fn play_url 

    pub fn stop(&self) {
        if let Ok(sink) = self.sink.lock(){
            sink.stop();
        } // <-- if let Ok(sink) = self.sink.lock(){
        //self.sink.stop();
    } // <-- fn stop

    pub fn set_volume(&self, volume: f32) {
        if let Ok(sink) = self.sink.lock() {
            sink.set_volume(volume);
        } // <-- if let Ok(sink)        
       // self.sink.set_volume(volume);
    } // <-- fn set_volume

}

