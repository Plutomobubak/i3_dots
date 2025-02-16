use vosk::{Model, Recognizer};
use std::io::Read;

pub struct SpeechRecognizer {
    recognizer: Recognizer,
    recongnizer_light: Recognizer
}

impl SpeechRecognizer {
    pub fn new() -> Self {
        let model = Model::new("./model-en-light").expect("Could not create model");
        let recognizer = Recognizer::new(&model, 16000.0).expect("Could not create recognizer");
        Self { recognizer,recongnizer_light }
    }

    pub fn speech_to_text_light(&mut self, buf:&[i16]) -> &str{
        let samples = buf;//u8_to_i16(buf);

        for sample in samples.chunks(100) {
            self.recognizer.accept_waveform(sample);
        }

        if let Some(result) = self.recognizer.result().single() {
            println!("{}",result.text);
            result.text
        }
        else{
            ""
        }
    }
}

// Helper function to convert byte array to i16 samples
fn u8_to_i16(input: &[u8]) -> Vec<i16> {
    input.chunks_exact(2)
        .map(|chunk| i16::from_le_bytes([chunk[0], chunk[1]]))
        .collect()
}