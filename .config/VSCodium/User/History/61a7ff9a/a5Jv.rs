use vosk::{Model, Recognizer};
use std::io::Read;

pub struct SpeechRecognizer {
    recognizer: Recognizer,
    recognizer_light: Recognizer
}

impl SpeechRecognizer {
    pub fn new() -> Self {
        let grammar = ["jarvis","search","remind","cancel"];
        let model_light = Model::new("./model-en-light").expect("Could not create model");
        let model = Model::new("./model-en").expect("Could not create model");
        let recognizer = Recognizer::new(&model, 16000.0).expect("Could not create recognizer");
        let recognizer_light = Recognizer::new_with_grammar(&model_light, 16000.0,&grammar).expect("Could not create recognizer");
        Self { recognizer,recognizer_light }
    }

    pub fn speech_to_text(&mut self, buf:&[i16]) -> &str{
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
    } pub fn speech_to_text_light(&mut self, buf:&[i16]) -> &str{
        let samples = buf;//u8_to_i16(buf);

        for sample in samples.chunks(100) {
            self.recognizer_light.accept_waveform(sample);
        }

        if let Some(result) = self.recognizer_light.result().single() {
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