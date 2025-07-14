use crate::Coordinates;
use agb::{
    fixnum::{num, vec2},
    rng::RandomNumberGenerator,
};

#[derive(Debug, Clone, Copy)]
pub struct ScreenShake {
    duration: u32,
    intensity: i32,
    offset: Coordinates,
}

impl ScreenShake {
    pub fn new(duration: u32, intensity: i32) -> Self {
        Self {
            duration,
            intensity,
            offset: vec2(num!(0), num!(0)),
        }
    }

    pub fn inactive() -> Self {
        Self::new(0, 0)
    }

    pub fn is_active(&self) -> bool {
        self.duration > 0
    }

    pub fn update(&mut self, rng: &mut RandomNumberGenerator) {
        if self.duration > 0 {
            self.duration -= 1;

            let x_offset = rng.next_i32().abs() % (self.intensity * 2 + 1)
                - self.intensity;
            let y_offset = rng.next_i32().abs() % (self.intensity * 2 + 1)
                - self.intensity;

            self.offset = vec2(x_offset.into(), y_offset.into());
        } else {
            self.offset = vec2(num!(0), num!(0));
        }
    }

    pub fn start(&mut self, duration: u32, intensity: i32) {
        self.duration = duration;
        self.intensity = intensity;
    }

    pub fn offset(&self) -> Coordinates {
        self.offset
    }
}

#[derive(Debug, Clone, Copy)]
pub struct WhiteFlash {
    duration: u32,
}

impl WhiteFlash {
    pub fn new() -> Self {
        Self { duration: 0 }
    }

    pub fn is_active(&self) -> bool {
        self.duration > 0
    }

    pub fn update(&mut self) {
        if self.duration > 0 {
            self.duration -= 1;
        }
    }

    pub fn start(&mut self, duration: u32) {
        self.duration = duration;
    }
}

