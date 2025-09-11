// TODO
use crate::effect_params::EffectParams;
use ringbuf::{
    HeapRb, SharedRb,
    storage::Heap,
    traits::{Consumer, Producer, Split},
    wrap::caching::Caching,
};
use std::sync::{Arc, Mutex, atomic::Ordering};

pub struct Delay {
    params: Arc<EffectParams>,
    // buffer: HeapRb::<f32>,
    prod: Caching<Arc<SharedRb<Heap<f32>>>, true, false>,
    cons: Caching<Arc<SharedRb<Heap<f32>>>, false, true>,
}

impl Delay {
    pub fn new(sample_rate: f32, params: Arc<EffectParams>) -> Self {
        let buffer = HeapRb::<f32>::new((sample_rate * 2.0 * 2.0) as usize);
        let (mut prod, mut cons) = buffer.split();

        for _ in 0..(sample_rate * 2.0) as usize {
            prod.try_push(0.0).unwrap();
        }

        Self { params, prod, cons }
    }

    pub fn process(&mut self, sample: f32) -> f32 {
        let delayed_sample =
            self.cons.try_pop().unwrap() * self.params.delay.decay.load(Ordering::Relaxed) as f32;
        self.prod.try_push(sample + delayed_sample).unwrap();

        return sample + delayed_sample;
    }
}
