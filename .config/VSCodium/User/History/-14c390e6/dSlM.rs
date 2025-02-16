use std::net::TcpStream;

use msedge_tts::tts::client::{connect, MSEdgeTTSClient};
use msedge_tts::tts::SpeechConfig;
use msedge_tts::voice::get_voices_list;
use hound::{WavSpec,WavWriter};

use cpal::traits::{DeviceTrait, HostTrait};
use crate::test::Playback;
pub struct TextToSpeech {
    narrator: MSEdgeTTSClient<TcpStream>,
}

impl TextToSpeech {
    pub fn new() -> TextToSpeech {
        let host = cpal::default_host();
    let default_device = host.default_output_device().expect("Failed to get default output device");
    let format = default_device.default_output_config().expect("Failed to get default output format");

    println!("Device: {}", default_device.name().expect("Failed to get device name"));
    println!("Config: {:?}", format);
        let voices = get_voices_list().unwrap();
        let mut conf = SpeechConfig::from(&voices[5]);
        for voice in &voices{
            println!("{}",voice.name);
            if voice.name.contains("Ryan"){
                conf = SpeechConfig::from(voice);
            }
        }
        let mut tts = connect().unwrap();
        let audio = tts.synthesize("ligma balls nigga. Why is no workie?", &conf).unwrap().audio_bytes;
        TextToSpeech::save_wav(&audio);
        let pl = Playback::new().unwrap();
        pl.play("speechre.wav".to_string());
        TextToSpeech {
            narrator : tts,
        }
    }

    pub fn speak(&mut self, text: &str) {
        let voices = get_voices_list().unwrap();
        let mut conf = SpeechConfig::from(&voices[5]);
        for voice in &voices{
            println!("{}",voice.name);
            if voice.name.contains("Ryan"){
                conf = SpeechConfig::from(voice);
            }
        }
        let mut tts = connect().unwrap();
        let audio = tts.synthesize(text, &conf).unwrap().audio_bytes;
    }

    fn u8_to_i16(input: &[u8]) -> Vec<i16> {
    input.chunks_exact(2)
        .map(|chunk| i16::from_le_bytes([chunk[0], chunk[1]]))
        .collect()
}

    fn save_wav(audio:&[u8]){
        let audio = TextToSpeech::u8_to_i16(audio);
                println!("{}",audio.len());
                let spec = WavSpec {
                channels: 1,             // Mono
                sample_rate: 44100,       // 16kHz sample rate
                bits_per_sample: 16,      // 16-bit samples
                sample_format: hound::SampleFormat::Int
            };
                let mut writer = WavWriter::create("./speech.wav", spec).expect("writer create oopsie");
                for &sample in audio.iter(){
                    writer.write_sample(sample).unwrap();
                }
                writer.finalize().expect("writer oopsie");

        std::process::Command::new("/bin/ffmpeg")..arg("-y").arg("-i").arg("speech.wav").arg("-ar").arg("44100").arg("speechre.wav").spawn().unwrap();
    }
}
