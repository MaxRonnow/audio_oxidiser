//! Feeds back the input stream directly into the output stream.
//!
//! Assumes that the input and output devices can use the same stream configuration and that they
//! support the f32 sample format.
//!
//! Uses a delay of `LATENCY_MS` milliseconds in case the default input and output streams are not
//! precisely synchronised.
use std::sync::Mutex;
use std::sync::{Arc, atomic::AtomicBool};
use std::thread;

use effect_params::EffectParams;

mod app;
mod effect_params;
mod effect_ui;
mod effects;
mod pipeline;
mod ui;

fn main() -> anyhow::Result<()> {
    let running = Arc::new(AtomicBool::new(true));
    let pipeline_running = Arc::clone(&running);
    let ui_running = Arc::clone(&running);

    let params = Arc::new(EffectParams::default());
    let ui_params = Arc::clone(&params);
    let pipeline_params = Arc::clone(&params);

    let pipeline_handle =
        thread::spawn(|| pipeline::init_pipeline(pipeline_running, pipeline_params).unwrap());
    let ui_handle = thread::spawn(|| app::init_ui(ui_running, ui_params).unwrap());

    pipeline_handle.join().unwrap();
    ui_handle.join().unwrap();

    Ok(())
}
