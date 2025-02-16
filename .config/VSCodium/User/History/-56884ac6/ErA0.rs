use cpal::{traits::{DeviceTrait, HostTrait, StreamTrait}, StreamConfig};
use std::{sync::{Arc, Mutex}, time::Duration};

use crate::speech_to_text::{self, SpeechRecognizer};
use hound::{WavWriter, WavSpec};

pub(crate) fn get_audio(){
    let mut speech = SpeechRecognizer::new();
    let host = cpal::default_host();
    let device = host.default_input_device().expect("No input device");
    let config:StreamConfig = device.default_input_config().expect("No config").into();
    
    // let buffer = Arc::new(Mutex::new(Vec::new()));

    // let buffer_ref = buffer.clone();
    
    let spec = WavSpec {
        channels: config.channels,             // Mono
        sample_rate: config.sample_rate.0,       // 16kHz sample rate
        bits_per_sample: 16,      // 16-bit samples
        sample_format: hound::SampleFormat::Int
    };

    let writer = Arc::new(Mutex::new(Some(WavWriter::create("input.wav", spec).unwrap())));

    // Clone writer reference to use in the audio stream callback
    let writer_ref = writer.clone();


    let stream = device.build_input_stream(
        &config, 
        move |data: &[i16], _: &cpal::InputCallbackInfo| {
                // let mut buffer = buffer_ref.lock().unwrap();

                // // Append audio data to the buffer
                // buffer.extend_from_slice(data);
                let mut writer = writer_ref.lock().unwrap();
                if let Some(writer) = writer.as_mut() {
                    for &sample in data.iter() {
                        writer.write_sample(sample).unwrap();
                    }
                }
            },
        
        move |err| {
            eprintln!("Error occurred on input stream: {}", err);
        },
        Some(Duration::from_secs(10))
    ).expect("Failed input stream build");

    stream.play().expect("failed to start");

    for _ in 0..5 {
        std::thread::sleep(Duration::from_secs(10));
        println!("Captured 10 seconds of audio...");
        speech.speech_to_text("input.wav");
    }

    // Finish the recording
    let mut writer = writer.lock().unwrap();
    if let Some(writer) = writer.take() {
        writer.finalize().expect("Failed to finalize WAV file");
    }

    println!("Recording completed!");
}