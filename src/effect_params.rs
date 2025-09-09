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
    pub bypass: bool,
    pub level: f32,
    pub distortion: f32,
}

impl Default for DistortionParams {
    fn default() -> Self {
        Self {
            bypass: false,
            level: 1.0,
            distortion: 0.5,
        }
    }
}

#[derive(Debug)]
pub struct DelayParams {
    pub bypass: bool,
    pub time: f32,
    pub decay: f32,
}

impl Default for DelayParams {
    fn default() -> Self {
        Self {
            bypass: false,
            time: 0.5,
            decay: 0.8,
        }
    }
}
