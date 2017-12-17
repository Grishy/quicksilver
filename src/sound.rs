extern crate rodio;
extern crate cpal;

use rodio::{Decoder, Sink, Source};
use rodio::source::{SamplesConverter, Amplify};
use std::fs::File;
use std::path::Path;
use std::io::{BufReader, Cursor, Read};
use std::sync::Arc;

#[derive(Clone)]
pub struct Sound {
    val: Arc<Vec<u8>>,
    volume: f32
}

impl Sound {
    pub fn load<P: AsRef<Path>>(path: P) -> Sound {
        let mut bytes = Vec::new();
        BufReader::new(File::open(path).unwrap()).read_to_end(&mut bytes).unwrap();
        let val = Arc::new(bytes);
        Sound {
            val,
            volume: 1f32
        }
    }

    pub fn volume(&self) -> f32 {
        self.volume
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume;
    }

    fn get_source(&self) -> SamplesConverter<Amplify<Decoder<Cursor<Sound>>>, f32> {
        Decoder::new(Cursor::new(self.clone())).unwrap().amplify(self.volume).convert_samples()
    }


    pub fn play(&self) {
        let endpoint = cpal::default_endpoint().unwrap();
        rodio::play_raw(&endpoint, self.get_source());
    }
}

impl AsRef<[u8]> for Sound {
    fn as_ref(&self) -> &[u8] {
        self.val.as_ref().as_ref()
    }
}

pub struct MusicPlayer {
    tracks: Vec<Sound>,
    sink: Sink
}

impl MusicPlayer {
    pub fn new() -> MusicPlayer {
        MusicPlayer {
            tracks: Vec::new(),
            sink: Sink::new(&cpal::default_endpoint().unwrap())
        }
    }

    pub fn add(&mut self, tracks: &[Sound]) {
        self.tracks.extend_from_slice(tracks);
    }

    pub fn play(&self) {
        self.sink.stop();
        for track in self.tracks.iter() {
            self.sink.append(track.get_source());
        }
    }

    pub fn pause(&self) {
        self.sink.pause();
    }

    pub fn finished(&self) -> bool {
        self.sink.empty()
    }

    pub fn volume(&self) -> f32 {
        self.sink.volume()
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.sink.set_volume(volume);
    }
}

