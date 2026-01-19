use portable_atomic::AtomicF32;
use std::sync::{Arc, atomic::AtomicBool};

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
    Bypass,
    Level,
    Distortion,
    LevelMinMax,
    DistortionMinMax,
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

    //pub fn get_param_names(&self) -> Vec<&str> {
    //    vec!["bypass", "level", "distortion"]
    //}
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
