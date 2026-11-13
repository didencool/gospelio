// --src/audio/command.rs
// Команди, які UI надсилає у фоновий аудіопотік

pub enum AudioCommand {
    Play(String),
    Stop,
    SetVolume(f32),
} // <-- enum AudioCommand
