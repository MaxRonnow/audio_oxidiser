use portable_atomic::AtomicF32;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

pub enum EffectType {
    Distortion(DistortionParams),
    Delay(DelayParams),
}

pub struct EffectParams {
    pub distortion: DistortionParams,
    pub delay: DelayParams,
}

impl EffectParams {
    pub fn new() -> Self {
        Self {
            distortion: DistortionParams::new(),
            delay: DelayParams::new(),
        }
    }
}

pub enum DistortionEnum {
    Bypass(AtomicBool),
    Level(AtomicF32),
    Distortion(AtomicF32),
    LevelMinMax { min: AtomicF32, max: AtomicF32 },
    DistortionMinMax { min: AtomicF32, max: AtomicF32 },
}

pub struct DistortionParams {
    pub bypass: AtomicBool,
    pub level: AtomicF32,
    pub level_min_max: Vec<AtomicF32>,
    pub distortion: AtomicF32,
    pub distortion_min_max: Vec<AtomicF32>,
}

impl DistortionParams {
    pub fn new() -> Self {
        Self {
            bypass: AtomicBool::new(false),
            level: AtomicF32::new(1.0),
            level_min_max: vec![AtomicF32::new(0.0), AtomicF32::new(1.0)],
            distortion: AtomicF32::new(0.5),
            distortion_min_max: vec![AtomicF32::new(0.0), AtomicF32::new(1.0)],
        }
    }

    pub fn change_level(&self, value: f32) {
        self.level.store(
            self.level.load(Ordering::Relaxed) + value * 0.1,
            Ordering::Relaxed,
        );
    }

    pub fn change_distortion(&self, value: f32) {
        self.distortion.store(value, Ordering::Relaxed);
    }
}

pub struct DelayParams {
    pub bypass: AtomicBool,
    pub time: AtomicF32,
    pub time_min_max: Vec<AtomicF32>,
    pub decay: AtomicF32,
    pub decay_min_max: Vec<AtomicF32>,
}

impl DelayParams {
    fn new() -> Self {
        Self {
            bypass: AtomicBool::new(false),
            time: AtomicF32::new(0.5),
            time_min_max: vec![AtomicF32::new(0.0), AtomicF32::new(4.0)],
            decay: AtomicF32::new(0.8),
            decay_min_max: vec![AtomicF32::new(0.0), AtomicF32::new(1.0)],
        }
    }
}
