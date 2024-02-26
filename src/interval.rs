#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub const EMPTY: Interval = Interval::new(f32::INFINITY, f32::NEG_INFINITY);
    pub const UNIVERSE: Interval = Interval::new(f32::NEG_INFINITY, f32::INFINITY);

    pub const fn new(min: f32, max: f32) -> Self {
        Interval { min, max }
    }

    pub fn contains(&self, x: f32) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f32) -> bool {
        self.min < x && x < self.max
    }
}

pub const fn interval(min: f32, max: f32) -> Interval {
    Interval::new(min, max)
}
