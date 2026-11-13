//! src/audio/mod.rs
pub mod command;
pub mod noseek;
pub mod volume;

// Дозволяє викликати `crate::audio::VolumeControl`
// замість довший шлях `crate::audio::volume::VolumeControl`
pub use volume::VolumeControl;
