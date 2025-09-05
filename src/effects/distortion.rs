// TODO

pub struct Distortion {
    level: f32,
    distortion: f32,
}

impl Distortion {
    pub fn new() -> Self {
        Self {
            level: 1.0,
            distortion: 1.0,
        }
    }
    

    pub fn process(&self, sample: f32) -> f32 {
        return sample.powf(3.0) * self.level;
    }
}