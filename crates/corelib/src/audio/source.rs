use rodio::Decoder;
use std::{fs::File, io::BufReader};

pub(crate) type AudioSource = Decoder<BufReader<File>>;
