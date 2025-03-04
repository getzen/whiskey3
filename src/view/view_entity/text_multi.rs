use macroquad::{math::Vec2, text::Font};

use crate::view::transform::Transform;

use super::{
    text::{AlignH, AlignV, Text},
    view_entity::ViewEntity,
};

pub struct TextMulti {
    pub visible: bool,
    pub transform: Transform,
    pub lines: Vec<Text>,
    pub spacings: Vec<f32>,
}

impl TextMulti {
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
        let mut line = Text::new(pos, text, font, font_size);
        line.align_h = align_h;
        line.align_v = AlignV::Top;
        self.lines.push(line);
        self.spacings.push(spacing);
    }
}

impl ViewEntity for TextMulti {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn draw(&mut self, parent_transform: &Transform) {
        if !self.visible {
            return;
        }

        let transform = *parent_transform * self.transform;

        for line in &mut self.lines {
            line.draw(&transform);
        }
    }
}
