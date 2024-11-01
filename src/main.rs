use std::f32::MIN_EXP;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use ringbuf::{
    producer,
    traits::{Consumer, Producer, RingBuffer, Split},
    HeapRb,
};

fn main() -> anyhow::Result<()> {
    // define the latency in milliseconds
    let latency: f32 = 100.00;

    let decay: f32 = 0.5;
    let mix: f32 = 0.5;

    // create the host
    let host = cpal::default_host();

    // get the default input and output devices
    let input_device = host
        .default_input_device()
        .expect("Could not find input device");
    let output_device = host
        .default_output_device()
        .expect("Could not find output device");

    println!("Using input device: \"{}\"", input_device.name()?);
    println!("Using output device: \"{}\"", output_device.name()?);

    // get the default input and outpur config
    let input_config: cpal::StreamConfig = input_device.default_input_config()?.into();
    let output_config: cpal::StreamConfig = output_device.default_output_config()?.into();

    // create a small delay for syncing input and output
    let latency_frames = (latency / 1_000.0) * input_config.sample_rate.0 as f32;
    let latency_samples = latency_frames as usize * input_config.channels as usize;

    // create a ring buffer for the delay
    let ring = HeapRb::<f32>::new(latency_samples * 2);
    // split the buffer into variables producer and consumer
    let (mut producer, mut consumer) = ring.split();

    // introduces the latency
    for _ in 0..latency_samples {
        // The ring buffer has twice as much space as necessary to add latency here,
        // so this should never fail
        producer.try_push(0.0).unwrap();
    }

    let decay_frames = decay * input_config.sample_rate.0 as f32;
    let decay_samples = decay_frames as usize * input_config.channels as usize;

    let mut reverb: Reverb = Reverb::new(decay, mix, decay_samples);

    let input_data_fn = move |data: &[f32], _: &cpal::InputCallbackInfo| {
        let mut output_fell_behind = false;

        for &sample in data {
            let processed = reverb.process(sample);

            if producer.try_push(processed).is_err() {
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

    println!(
        "Attempting to build both streams with f32 samples and `{:?}`.",
        input_config
    );
    let input_stream =
        input_device.build_input_stream(&input_config, input_data_fn, err_fn, None)?;
    let output_stream =
        output_device.build_output_stream(&output_config, output_data_fn, err_fn, None)?;
    println!("Successfully built streams.");

    // Play the streams.
    println!(
        "Starting the input and output streams with `{}` milliseconds of latency.",
        latency
    );
    input_stream.play()?;
    output_stream.play()?;

    // Run for 10 seconds before closing.
    println!("Playing for 10 seconds... ");
    std::thread::sleep(std::time::Duration::from_secs(10));
    drop(input_stream);
    drop(output_stream);
    println!("Done!");

    Ok(())
}

fn err_fn(err: cpal::StreamError) {
    eprintln!("an error occurred on stream: {}", err);
}

struct Reverb {
    reverb_producer: ringbuf::HeapProd<f32>,
    reverb_consumer: ringbuf::HeapCons<f32>,
    decay: f32,
    mix: f32,
}

impl Reverb {
    fn new(decay: f32, mix: f32, decay_samples: usize) -> Self {
        let buffer = ringbuf::HeapRb::<f32>::new(decay_samples);
        let (mut reverb_producer, mut reverb_consumer) = buffer.split();
        Self {
            reverb_producer: reverb_producer,
            reverb_consumer: reverb_consumer,
            decay,
            mix,
        }
    }

    fn process(&mut self, sample: f32) -> f32 {
        return self.reverb_consumer.try_pop().unwrap();
    }
}
