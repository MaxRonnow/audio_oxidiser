//! Feeds back the input stream directly into the output stream.
//!
//! Assumes that the input and output devices can use the same stream configuration and that they
//! support the f32 sample format.
//!
//! Uses a delay of `LATENCY_MS` milliseconds in case the default input and output streams are not
//! precisely synchronised.
use std::thread;
use std::sync::{
    atomic::{AtomicBool},
    Arc,
};


mod pipeline;
mod ui;
mod effects;



fn main() -> anyhow::Result<()> {
    let running = Arc::new(AtomicBool::new(true));
    let pipeline_running = Arc::clone(&running);
    let ui_running = Arc::clone(&running);

    let pipeline_handle = thread::spawn(|| pipeline::init_pipeline(pipeline_running).unwrap());
    let ui_handle = thread::spawn(|| ui::init_ui(ui_running).unwrap());

    pipeline_handle.join().unwrap();
    ui_handle.join().unwrap();

    Ok(())
}
