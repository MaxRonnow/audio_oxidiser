// TODO

pub struct Distortion {
    
}

impl Distortion {
    pub fn new() -> Self {
        
    }

    pub fn process(&self, sample: f32) -> f32 {
        return sample * sample * sample
    }
}