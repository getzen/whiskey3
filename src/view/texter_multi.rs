use macroquad::{math::Vec2, text::Font};

use super::{
    texter::{AlignH, AlignV, Texter},
    transform_old::Transform,
};

pub struct TexterMulti {
    pub visible: bool,
    pub transform: Transform,
    pub lines: Vec<Texter>,
    pub spacings: Vec<f32>,
}

impl TexterMulti {
    pub fn new(position: Vec2) -> Self {
        Self {
            visible: true,
            transform: Transform::from_translation(position),
            lines: Vec::new(),
            spacings: Vec::new(),
        }
    }

    pub fn clear_lines(&mut self) {
        self.lines.clear();
        self.spacings.clear();
    }

    pub fn add_line(&mut self, text: &str, font: Font, font_size: u16, align_h: AlignH, spacing: f32) {
        let mut pos = Vec2::ZERO;
        for spacing in &self.spacings {
            pos.y += spacing;
        }
        let line = Texter::new(pos, text, font, font_size, align_h, AlignV::Top);
        self.lines.push(line);
        self.spacings.push(spacing);
    }

    pub fn draw(&self, parent_transform: Option<&Transform>) {
        if !self.visible {
            return;
        }

        let transform = match parent_transform {
            Some(parent) => &Transform::combine(parent, &self.transform),
            None => &self.transform,
        };

        for line in &self.lines {
            line.draw(Some(transform));
        }
    }
}
