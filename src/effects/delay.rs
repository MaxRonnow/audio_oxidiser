// TODO
use ringbuf::{
    traits::{Consumer, Producer, Split},
    HeapRb,
    SharedRb,
    wrap::caching::Caching,
    Arc,
    storage::Heap,
};
//use std::sync::{
//    Arc,
//};


pub struct Delay {
    time: f32, // delay time in seconds
    decay: f32,
    // buffer: HeapRb::<f32>,
    prod: Caching<Arc<SharedRb<Heap<f32>>>, true, false>,
    cons: Caching<Arc<SharedRb<Heap<f32>>>, false, true>,
}

impl Delay{
    pub fn new(sample_rate: f32) -> Self {
        let mut time = 1.0;
        let buffer = HeapRb::<f32>::new((sample_rate * time * 2.0) as usize);
        let (mut prod, mut cons) = buffer.split();

        for _ in 0..(sample_rate * time) as usize {
            prod.try_push(0.0).unwrap();
        };

        Self {
            time,
            decay: 0.8,
            prod,
            cons,
        }
    }

    pub fn process(&mut self, sample: f32) -> f32 {
        let delayed_sample = self.cons.try_pop().unwrap() * self.decay;
        self.prod.try_push(sample + delayed_sample).unwrap();

        return sample + delayed_sample;
    }

    
}