// TODO
// The audio processing pipeline

// Some code taken from the CPAL Feedback example

use clap::Parser;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use ringbuf::{
    traits::{Consumer, Producer, Split},
    HeapRb,
};

#[derive(Parser, Debug)]
#[command(version, about = "CPAL feedback example", long_about = None)]
struct Opt {
    /// The input audio device to use
    #[arg(short, long, value_name = "IN", default_value_t = String::from("default"))]
    input_device: String,

    /// The output audio device to use
    #[arg(short, long, value_name = "OUT", default_value_t = String::from("default"))]
    output_device: String,

    /// Specify the delay between input and output
    #[arg(short, long, value_name = "DELAY_MS", default_value_t = 150.0)]
    latency: f32,

    /// Use the JACK host
    #[cfg(all(
        any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd"
        ),
        feature = "jack"
    ))]
    #[arg(short, long)]
    #[allow(dead_code)]
    jack: bool,
}

pub fn pipeline() -> anyhow::Result<()> {
    let opt = Opt::parse();

    // Conditionally compile with jack if the feature is specified.
    #[cfg(all(
        any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd"
        ),
        feature = "jack"
    ))]
    // Manually check for flags. Can be passed through cargo with -- e.g.
    // cargo run --release --example beep --features jack -- --jack
    let host = if opt.jack {
        cpal::host_from_id(cpal::available_hosts()
            .into_iter()
            .find(|id| *id == cpal::HostId::Jack)
            .expect(
                "make sure --features jack is specified. only works on OSes where jack is available",
            )).expect("jack host unavailable")
    } else {
        cpal::default_host()
    };

    #[cfg(any(
        not(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd"
        )),
        not(feature = "jack")
    ))]
    let host = cpal::default_host();

    // Find devices.
    let input_device = if opt.input_device == "default" {
        host.default_input_device()
    } else {
        host.input_devices()?
            .find(|x| x.name().map(|y| y == opt.input_device).unwrap_or(false))
    }
    .expect("failed to find input device");

    let output_device = if opt.output_device == "default" {
        host.default_output_device()
    } else {
        host.output_devices()?
            .find(|x| x.name().map(|y| y == opt.output_device).unwrap_or(false))
    }
    .expect("failed to find output device");

    println!("Using input device: \"{}\"", input_device.name()?);
    println!("Using output device: \"{}\"", output_device.name()?);

    // We'll try and use the same configuration between streams to keep it simple.
    let config: cpal::StreamConfig = input_device.default_input_config()?.into();

    // Create a delay in case the input and output devices aren't synced.
    let latency_frames = (opt.latency / 1_000.0) * config.sample_rate.0 as f32;
    let latency_samples = latency_frames as usize * config.channels as usize;

    // The buffer to share samples
    let ring = HeapRb::<f32>::new(latency_samples * 2);
    let (mut producer, mut consumer) = ring.split();

    // Fill the samples with 0.0 equal to the length of the delay.
    for _ in 0..latency_samples {
        // The ring buffer has twice as much space as necessary to add latency here,
        // so this should never fail
        producer.try_push(0.0).unwrap();
    }

    let input_data_fn = move |data: &[f32], _: &cpal::InputCallbackInfo| {
        let mut output_fell_behind = false;
        for &sample in data {
            if producer.try_push(sample).is_err() {
                output_fell_behind = true;
            }
        }
        if output_fell_behind {
            eprintln!("output stream fell behind: try increasing latency");
        }
    };

    let output_data_fn = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
        let mut input_fell_behind = false;
        for sample in data {
            *sample = match consumer.try_pop() {
                Some(s) => s,
                None => {
                    input_fell_behind = true;
                    0.0
                }
            };
        }
        if input_fell_behind {
            eprintln!("input stream fell behind: try increasing latency");
        }
    };

    // Build streams.
    println!("Attempting to build both streams with f32 samples and `{config:?}`.");
    let input_stream = input_device.build_input_stream(&config, input_data_fn, err_fn, None)?;
    let output_stream = output_device.build_output_stream(&config, output_data_fn, err_fn, None)?;
    println!("Successfully built streams.");

    // Play the streams.
    println!(
        "Starting the input and output streams with `{}` milliseconds of latency.",
        opt.latency
    );
    input_stream.play()?;
    output_stream.play()?;

    // Run for 3 seconds before closing.
    println!("Playing for 3 seconds... ");
    std::thread::sleep(std::time::Duration::from_secs(3));
    drop(input_stream);
    drop(output_stream);
    println!("Done!");
    Ok(())
}

fn err_fn(err: cpal::StreamError) {
    eprintln!("an error occurred on stream: {err}");
}


//use std::f32::MIN_EXP;
//
//use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
//use ringbuf::{
//    producer,
//    traits::{Consumer, Producer, RingBuffer, Split},
//    HeapRb,
//};
//
//fn main() -> anyhow::Result<()> {
//    // define the latency in milliseconds
//    let latency: f32 = 100.00;
//
//    let decay: f32 = 0.5;
//    let mix: f32 = 0.5;
//
//    // create the host
//    let host = cpal::default_host();
//
//    // get the default input and output devices
//    let input_device = host
//        .default_input_device()
//        .expect("Could not find input device");
//    let output_device = host
//        .default_output_device()
//        .expect("Could not find output device");
//
//    println!("Using input device: \"{}\"", input_device.name()?);
//    println!("Using output device: \"{}\"", output_device.name()?);
//
//    // get the default input and outpur config
//    let input_config: cpal::StreamConfig = input_device.default_input_config()?.into();
//    let output_config: cpal::StreamConfig = output_device.default_output_config()?.into();
//
//    // create a small delay for syncing input and output
//    let latency_frames = (latency / 1_000.0) * input_config.sample_rate.0 as f32;
//    let latency_samples = latency_frames as usize * input_config.channels as usize;
//
//    // create a ring buffer for the delay
//    let ring = HeapRb::<f32>::new(latency_samples * 2);
//    // split the buffer into variables producer and consumer
//    let (mut producer, mut consumer) = ring.split();
//
//    // introduces the latency
//    for _ in 0..latency_samples {
//        // The ring buffer has twice as much space as necessary to add latency here,
//        // so this should never fail
//        producer.try_push(0.0).unwrap();
//    }
//
//    let decay_frames = decay * input_config.sample_rate.0 as f32;
//    let decay_samples = decay_frames as usize * input_config.channels as usize;
//
//    let mut reverb: Reverb = Reverb::new(decay, mix, decay_samples);
//
//    let input_data_fn = move |data: &[f32], _: &cpal::InputCallbackInfo| {
//        let mut output_fell_behind = false;
//
//        for &sample in data {
//            let processed = reverb.process(sample);
//
//            if producer.try_push(processed).is_err() {
//                output_fell_behind = true;
//            }
//        }
//        if output_fell_behind {
//            eprintln!("output stream fell behind: try increasing latency");
//        }
//    };
//
//    let output_data_fn = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
//        let mut input_fell_behind = false;
//        for sample in data {
//            *sample = match consumer.try_pop() {
//                Some(s) => s,
//                None => {
//                    input_fell_behind = true;
//                    0.0
//                }
//            };
//        }
//        if input_fell_behind {
//            eprintln!("input stream fell behind: try increasing latency");
//        }
//    };
//
//    println!(
//        "Attempting to build both streams with f32 samples and `{:?}`.",
//        input_config
//    );
//    let input_stream =
//        input_device.build_input_stream(&input_config, input_data_fn, err_fn, None)?;
//    let output_stream =
//        output_device.build_output_stream(&output_config, output_data_fn, err_fn, None)?;
//    println!("Successfully built streams.");
//
//    // Play the streams.
//    println!(
//        "Starting the input and output streams with `{}` milliseconds of latency.",
//        latency
//    );
//    input_stream.play()?;
//    output_stream.play()?;
//
//    // Run for 10 seconds before closing.
//    println!("Playing for 10 seconds... ");
//    std::thread::sleep(std::time::Duration::from_secs(10));
//    drop(input_stream);
//    drop(output_stream);
//    println!("Done!");
//
//    Ok(())
//}
//
//fn err_fn(err: cpal::StreamError) {
//    eprintln!("an error occurred on stream: {}", err);
//}
//
//struct Reverb {
//    reverb_producer: ringbuf::HeapProd<f32>,
//    reverb_consumer: ringbuf::HeapCons<f32>,
//    decay: f32,
//    mix: f32,
//}
//
//impl Reverb {
//    fn new(decay: f32, mix: f32, decay_samples: usize) -> Self {
//        let buffer = ringbuf::HeapRb::<f32>::new(decay_samples);
//        let (mut reverb_producer, mut reverb_consumer) = buffer.split();
//        Self {
//            reverb_producer: reverb_producer,
//            reverb_consumer: reverb_consumer,
//            decay,
//            mix,
//        }
//    }
//
//    fn process(&mut self, sample: f32) -> f32 {
//        return self.reverb_consumer.try_pop().unwrap();
//    }
//}
