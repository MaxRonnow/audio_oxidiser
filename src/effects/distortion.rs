// TODO
use crate::effect_params::EffectParams;
use std::sync::{Arc, Mutex, atomic::Ordering};

pub struct Distortion {
    params: Arc<EffectParams>,
}

impl Distortion {
    pub fn new(params: Arc<EffectParams>) -> Self {
        Self { params }
    }

    pub fn process(&self, sample: f32) -> f32 {
        return sample.powf(3.0) * self.params.distortion.level.load(Ordering::Relaxed) as f32;
    }
}
