// --- src/app.rs ---
pub struct GospelioApp {
    pub config: AppConfig,
    pub playback_state: PlaybackState,
    pub current_station: Option<Station>,
    pub track_title: String,
    pub safety_unlocked: bool,
    
    // Стан вікна
    pub is_window_visible: bool,

    pub search_query: String,
    pub is_editing_stations: bool,
}
