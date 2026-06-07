use crate::audio::{AudioBackend, AudioDecoder};

pub struct AudioEngine {
    decoder: Box<dyn AudioDecoder>,
    backend: Box<dyn AudioBackend>,
}
