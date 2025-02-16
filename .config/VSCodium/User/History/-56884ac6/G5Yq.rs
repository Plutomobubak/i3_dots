use cpal::{traits::{DeviceTrait, HostTrait, StreamTrait}, SampleRate, StreamConfig};
use std::{sync::{Arc, Mutex}, time::Duration};

pub(crate) fn get_audio(){
    let host = cpal::default_host();
    let device = host.default_input_device().expect("No input device");
    let mut config:StreamConfig = device.default_input_config().expect("No config").into();
    
    // let buffer = Arc::new(Mutex::new(Vec::new()));

    // let buffer_ref = buffer.clone();
    config.sample_rate = SampleRate(16000);
    config.channels = 1;

    let stream = device.build_input_stream(
        &config, 
        move |data: &[i16], _: &cpal::InputCallbackInfo| {
                let mut buffer = buffer_ref.lock().unwrap();

                // // Append audio data to the buffer
                buffer.extend_from_slice(data);
            },
        
        move |err| {
            eprintln!("Error occurred on input stream: {}", err);
        },
        Some(Duration::from_secs(10))
    ).expect("Failed input stream build");

    stream.play().expect("failed to start");

    for _ in 0..5{
        std::thread::sleep(Duration::from_secs(1));
        println!("Captured 10 seconds of audio...");
        
    }

    // Finish the recording
    let mut writer = writer.lock().unwrap();
    if let Some(writer) = writer.take() {
        writer.finalize().expect("Failed to finalize WAV file");
    }
    println!("Recording completed!");
}