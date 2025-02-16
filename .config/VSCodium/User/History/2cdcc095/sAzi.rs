mod audio;
mod speech_to_text;
mod text_to_speech;
mod test;
use std::{thread::sleep, time::Duration};

use speech_to_text::SpeechRecognizer;
use audio::VoiceBuffer;
use text_to_speech::TextToSpeech;
fn main() {
    let mut speech = SpeechRecognizer::new();
    let mut voice = VoiceBuffer::new();
    let mut text_to_speech = TextToSpeech::new();
    text_to_speech.speak("why do i do this");
    //text_to_speech.play_wav();
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
                match command {
                    "cancel" => break,
                    "search" => {
                        voice.remove_first_n_samples(buf.len()-3000);
                        println!("start listen...");
                        let mut l=0.0;
                        while(l < 3.0){
                            sleep(Duration::from_millis(200));
                            l = voice.length_of_continuous_silence(300, 16000);
                        }
                        println!("end listen");
                        let buf = voice.get_audio();
                        let args = speech.speech_to_text(&buf);
                        std::process::Command::new("/bin/chromium").arg("http://google.com/search?q=".to_owned()+args).spawn().expect("epic fail");
                        break;
                    },
                    "remind" => {

                    },
                    &_ => {},
                }
                
                voice.remove_first_n_samples(buf.len()-3000);
                sleep(Duration::from_secs(1));
            }
        }
        voice.remove_first_n_samples(buf.len()-3000);
    }
}
