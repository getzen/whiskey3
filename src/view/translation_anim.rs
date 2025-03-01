use macroquad::math::{Vec2, vec2};

pub struct TranslationAnimator {
    pub current: Vec2,
    end: Vec2,
    vector: Vec2,
    time_remaining: f32,
    pub completed: bool,
}

impl TranslationAnimator {
    pub fn new(start: Vec2, end: Vec2, velocity: f32) -> Self {
        let radians = (end.y - start.y).atan2(end.x - start.x);
        Self {
            current: start,
            end,
            vector: vec2(radians.cos(), radians.sin()) * velocity,
            time_remaining: start.distance(end) / velocity,
            completed: false,
        }
    }

    pub fn update(&mut self, time_delta: f32) -> Vec2 {
        self.time_remaining -= time_delta;
        if self.time_remaining <= 0.0 {
            self.current = self.end;
            self.time_remaining = 0.0;
            self.completed = true;
        } else {
            self.current += self.vector * time_delta;
        }
        self.current
    }
}
