use rodio::{Decoder, OutputStream, OutputStreamHandle, PlayError, Sink, StreamError};
use std::{
    fs::File,
    io::BufReader,
    sync::{Arc, Mutex},
    thread, time::Duration,
};

pub struct Playback {
    sink: Arc<Mutex<Sink>>,
    stream_handle: OutputStreamHandle,
    stream: OutputStream,
    is_playing: bool,
}

#[derive(Debug)]
pub enum PlaybackError {
    StreamCreationError(StreamError),
    SinkCreationError(PlayError),
}

impl Playback {
    pub fn new() -> Result<Self, PlaybackError> {
        let (stream, stream_handle) = match OutputStream::try_default() {
            Ok(s) => s,
            Err(e) => {
                return Err(PlaybackError::StreamCreationError(e));
            }
        };

        let sink = match Sink::try_new(&stream_handle) {
            Ok(s) => s,
            Err(e) => {
                return Err(PlaybackError::SinkCreationError(e));
            }
        };

        Ok(Self {
            sink: Arc::new(Mutex::new(sink)),
            stream_handle,
            stream,
            is_playing: false,
        })
    }

    pub fn play(&self, filepath: String) {

        let file = match File::open(&filepath) {
            Ok(f) => f,
            Err(e) => {
                return;
            }
        };

        let buf_reader = BufReader::new(file);

        let source = match Decoder::new(buf_reader) {
            Ok(s) => s,
            Err(e) => {
                return;
            }
        };

        let sink = self.sink.lock().unwrap();

        sink.append(source);

        thread::sleep(Duration::from_secs((buf_reader.buffer().len()/44100)as u64));
    }
}