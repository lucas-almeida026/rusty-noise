pub trait NoiseGenerator: Send {
    fn next_sample(&mut self) -> f32;
}

struct WhiteNoise;
impl NoiseGenerator for WhiteNoise {
    fn next_sample(&mut self) -> f32 {
        rand::random::<f32>() * 2.0 - 1.0
    }
}

struct BrowNoise {
    last: f32,
}
impl BrowNoise {
    fn new() -> Self {
        BrowNoise { last: 0.0 }
    }
}
impl NoiseGenerator for BrowNoise {
    fn next_sample(&mut self) -> f32 {
        let white = rand::random::<f32>() * 2.0 - 1.0;
        self.last += white * 0.05;
        self.last = self.last.clamp(-1.0, 1.0);
        self.last
    }
}

struct PinkNoise {
    white: [f32; 7],
    index: usize,
}
impl PinkNoise {
    fn new() -> Self {
        PinkNoise {
            white: [0.0; 7],
            index: 0,
        }
    }
}
impl NoiseGenerator for PinkNoise {
    fn next_sample(&mut self) -> f32 {
        let i = self.index;
        self.index = (self.index + 1) % 7;
        self.white[i] = rand::random::<f32>() * 2.0 - 1.0;
        self.white.iter().sum::<f32>() / 7.0
    }
}

struct BlueNoise {
    last: f32,
}
impl BlueNoise {
    fn new() -> Self {
        Self { last: 0.0 }
    }
}
impl NoiseGenerator for BlueNoise {
    fn next_sample(&mut self) -> f32 {
        let white = rand::random::<f32>() * 2.0 - 1.0;
        let blue = white - self.last;
        self.last = white;
        blue.clamp(-1.0, 1.0)
    }
}

pub enum NoiseType {
    White,
    Brown,
    Pink,
    Blue,
}

pub fn noise_generator_from_type(t: NoiseType) -> Box<dyn NoiseGenerator> {
    match t {
        NoiseType::White => Box::new(WhiteNoise),
        NoiseType::Blue => Box::new(BlueNoise::new()),
        NoiseType::Brown => Box::new(BrowNoise::new()),
        NoiseType::Pink => Box::new(PinkNoise::new()),
    }
}

pub struct ControlledNoise {
    pub generator: Box<dyn NoiseGenerator + Send>,
    pub volume: f32,
}
impl ControlledNoise {
    pub fn new(t: NoiseType, vol: f32) -> Self {
        ControlledNoise {
            generator: noise_generator_from_type(t),
            volume: vol,
        }
    }
}
