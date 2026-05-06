#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct AnimationState {
    step: u8,
}

impl AnimationState {
    pub const MAX_STEP: u8 = 8;

    pub fn advance(&mut self) {
        self.step = (self.step + 1) % (Self::MAX_STEP + 1);
    }

    pub const fn step(self) -> u8 {
        self.step
    }

    pub fn bar_value(self, value: f32) -> f32 {
        if value <= 0.0 {
            0.0
        } else {
            (value + self.step as f32 * 0.01).min(1.0)
        }
    }
}
