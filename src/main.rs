//! Feeds back the input stream directly into the output stream.
//!
//! Assumes that the input and output devices can use the same stream configuration and that they
//! support the f32 sample format.
//!
//! Uses a delay of `LATENCY_MS` milliseconds in case the default input and output streams are not
//! precisely synchronised.

mod pipeline;

fn main() -> anyhow::Result<()> {

    pipeline::pipeline();
    
    Ok(())
}



