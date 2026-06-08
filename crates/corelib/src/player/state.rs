#[derive(Debug, Clone, Copy)]
pub enum PlaybackState {
    Playing,
    Paused,
    Stopped,
}

#[derive(Clone, Copy)]
pub enum RepeatMode {
    None,
    Queue,
    Track,
}

#[derive(Debug, Clone, Copy)]
pub enum BackendState {
    None,
    TrackEnded,
    Playing,
}
