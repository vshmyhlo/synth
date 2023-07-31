use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::StreamConfig;
// use dasp::signal;
use dasp_signal::{self as signal, Signal};

use std::thread::sleep;
use std::time::Duration;
fn main() {
    let host = cpal::default_host();

    // println!("{}", host)
    let device = host
        .default_output_device()
        .expect("no output device available");

    let mut supported_configs_range = device
        .supported_output_configs()
        .expect("error while querying configs");

    // println!("{:?}", supported_configs_range)
    let supported_config = supported_configs_range
        .next()
        .expect("no supported config?!")
        .with_max_sample_rate();

    let config: StreamConfig = supported_config.into();

    let mut s = signal::rate(config.sample_rate.0.into())
        .const_hz(220.)
        .sine()
        .scale_amp(0.1);

    let stream = device
        .build_output_stream(
            &config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                // println!("{:?}", data)
                // data.len()

                // data.len() =

                // *data =
                for x in data.iter_mut() {
                    *x = s.next() as f32;
                }

                // react to stream events and read or write stream data here.
            },
            move |err| {
                println!("error: {}", err)
                // react to errors here.
            },
            // None, // None=blocking, Some(Duration)=timeout
        )
        .unwrap();

    stream.play().unwrap();

    // for _ in 0..10 {
    //     let x = s.next();
    //     println!("{}", x);
    // }

    sleep(Duration::from_secs(3));
}
