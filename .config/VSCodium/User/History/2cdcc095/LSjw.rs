mod audio;
mod speech_to_text;
use std::{thread::sleep, time::Duration};

use speech_to_text::SpeechRecognizer;
use audio::VoiceBuffer;
fn main() {
    let mut speech = SpeechRecognizer::new();
    let mut voice = VoiceBuffer::new();
    loop{
        sleep(Duration::from_secs(1));
        let buf = voice.get_audio();
        let key = speech.speech_to_text_light(&buf);
        if key.contains("jarvis"){
            println!("yay we got jarvis");
            loop{
                let buf = voice.get_audio();
                let command = speech.speech_to_text_light(&buf);
                
                println!("{}",command);
                let n = match command {
                    "cancel" => break,
                    "search" => let n = std::process::Command::new("/bin/chromium").arg("http://google.com/search\\?q\\=balls"),
                    &_ => todo!()
                }
                
                voice.remove_first_n_samples(buf.len()-3000);
                sleep(Duration::from_secs(1));
            }
        }
        voice.remove_first_n_samples(buf.len()-3000);
    }
}
