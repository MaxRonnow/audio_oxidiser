use portable_atomic::AtomicF32;
use std::sync::{Arc, atomic::AtomicBool};

#[derive(Debug)]
pub struct EffectParams {
    pub distortion: DistortionParams,
    pub delay: DelayParams,
}

impl Default for EffectParams {
    fn default() -> Self {
        Self {
            distortion: DistortionParams::default(),
            delay: DelayParams::default(),
        }
    }
}

#[derive(Debug)]
pub struct DistortionParams {
    pub bypass: AtomicBool,
    pub level: AtomicF32,
    pub distortion: AtomicF32,
}

impl Default for DistortionParams {
    fn default() -> Self {
        Self {
            bypass: AtomicBool::new(false),
            level: AtomicF32::new(1.0),
            distortion: AtomicF32::new(0.5),
        }
    }
}

#[derive(Debug)]
pub struct DelayParams {
    pub bypass: AtomicBool,
    pub time: AtomicF32,
    pub decay: AtomicF32,
}

impl Default for DelayParams {
    fn default() -> Self {
        Self {
            bypass: AtomicBool::new(false),
            time: AtomicF32::new(0.5),
            decay: AtomicF32::new(0.8),
        }
    }
}
