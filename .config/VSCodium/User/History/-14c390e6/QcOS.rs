use std::fs::File;
use std::io::{BufReader, Cursor, Read};
use std::net::TcpStream;

use cpal::StreamConfig;
use msedge_tts::tts::client::{connect, MSEdgeTTSClient};
use msedge_tts::tts::SpeechConfig;
use msedge_tts::voice::get_voices_list;
use hound::{WavSpec,WavWriter};
use rodio::{OutputStream, OutputStreamHandle, Sink, Source};
use cpal::traits::{DeviceTrait, HostTrait};

pub struct TextToSpeech {
    narrator: MSEdgeTTSClient<TcpStream>,
    stream: OutputStream,
    stream_handle: OutputStreamHandle
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
        //TextToSpeech::play_wav();
        let (stream,stream_handle)= rodio::OutputStream::try_default().unwrap();
        TextToSpeech {
            narrator : tts,
            stream,
            stream_handle
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
        //TextToSpeech::save_wav(&audio);
        self.play_wav();
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
    }
    fn play_wav(&mut self){
        
        let file = match File::open("speech.wav") {
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

        let sink_clone = Arc::clone(&self.sink);

        let thread = thread::spawn(move || {
            let sink = sink_clone.lock().unwrap();

            sink.sleep_until_end();

            // thread::sleep(Duration::from_secs(2));
        });

        thread.join().unwrap();
    }
}
