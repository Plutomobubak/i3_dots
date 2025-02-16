use vosk::{Model, Recognizer};
use std::io::Read;

pub struct SpeechRecognizer {
    recognizer: Recognizer,
}

impl SpeechRecognizer {
    pub fn new() -> Self {
        let model = Model::new("./model-en").expect("Could not create model");
        let recognizer = Recognizer::new(&model, 16000.0).expect("Could not create recognizer");
        Self { recognizer }
    }

    pub fn speech_to_text(&mut self, audio_path: &str) {
        let mut audio = std::fs::File::open(audio_path).expect("Could not open audio file");
        let mut buf = Vec::new();
        
        // Skip WAV header (usually 44 bytes, but could vary)
        audio.read_to_end(&mut buf).expect("Could not read audio file");
        let samples = u8_to_i16(&buf);

        for sample in samples.chunks(100) {
            self.recognizer.accept_waveform(sample);
        }

        if let Some(result) = self.recognizer.result().single() {
            println!("Recognized Text: {}", result.text);
        }
    }
}

// Helper function to convert byte array to i16 samples
fn u8_to_i16(input: &[u8]) -> Vec<i16> {
    input.chunks_exact(2)
        .map(|chunk| i16::from_le_bytes([chunk[0], chunk[1]]))
        .collect()
}