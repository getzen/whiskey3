use macroquad::prelude::*;

use crate::view::transform_old::Transform;

#[allow(unused)]
#[derive(Clone)]
pub enum AlignH {
    Left,
    Center,
    Right,
}

#[allow(unused)]
#[derive(Clone)]
pub enum AlignV {
    Top,
    Center,
    Bottom,
}

#[derive(Clone)]
pub struct Texter {
    pub visible: bool,
    pub transform: Transform,
    pub text: String,
    pub font: Font,
    pub font_size: u16,
    pub font_scale: f32,
    pub align_h: AlignH,
    pub align_v: AlignV,
    pub color: Color,
}

impl Texter {
    pub fn new(position: Vec2, text: &str, font: Font, font_size: u16, align_h: AlignH, align_v: AlignV) -> Self {
        Self {
            visible: true,
            transform: Transform::from_translation(position),
            text: text.to_string(),
            font,
            font_size,
            font_scale: 1.0,
            align_h,
            align_v,
            color: WHITE,
        }
    }

    pub fn draw(&self, parent_transform: Option<&Transform>) {
        if !self.visible {
            return;
        }
        // Determine the adjustment we need to make for h/v centering.
        let mut adj_pos = Vec2::ZERO;

        // Is this function slow?
        let dimensions = measure_text(&self.text, Some(&self.font), self.font_size, self.font_scale);

        adj_pos.x += match self.align_h {
            AlignH::Left => 0.0,
            AlignH::Center => -dimensions.width * 0.5,
            AlignH::Right => -dimensions.width,
        };

        adj_pos.y += match self.align_v {
            AlignV::Top => dimensions.offset_y,
            AlignV::Center => dimensions.offset_y * 0.5,
            AlignV::Bottom => 0.0,
        };

        let adj_transform = Transform::from_translation(adj_pos);

        let transform = match parent_transform {
            Some(parent) => &Transform::combine(parent, &self.transform),
            None => &self.transform,
        };
        // Combine our adjustment transform.
        let transform = Transform::combine(transform, &adj_transform);
        let (pos, rot) = transform.drawable_position_rotation();

        let params = TextParams {
            font: Some(&self.font),
            font_size: self.font_size,
            font_scale: self.font_scale,
            font_scale_aspect: 1.0,
            rotation: rot,
            color: self.color,
        };
        draw_text_ex(&self.text, pos.x, pos.y, params);
    }
}
