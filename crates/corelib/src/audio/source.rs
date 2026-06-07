use rodio::Decoder;
use std::{fs::File, io::BufReader};

pub type AudioSource = Decoder<BufReader<File>>;
