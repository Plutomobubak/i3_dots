use vosk::{Model, Recognizer};
use std::io::Read;

fn u8_to_i16(input: &[u8]) -> Vec<i16> {
    input.chunks_exact(2)
        .map(|chunk| i16::from_le_bytes([chunk[0], chunk[1]]))
        .collect()
}
pub fn speech_to_text(){
    let model = Model::new("./model-en").expect("Could not create model");
    let mut recognizer = Recognizer::new(&model, 16000.0).expect("Could not create recognizer");
    let mut audio = std::fs::File::open("audio.wav").expect("Could not open audio file");
    let mut buf = Vec::new();
    audio.read_to_end(&mut buf).expect("Could not read audio file");

    let samples = u8_to_i16(&buf);
    for sample in samples.chunks(100){
        recognizer.accept_waveform(sample);
    }
    print!("{:#?}",recognizer.result().single().unwrap().text);
}