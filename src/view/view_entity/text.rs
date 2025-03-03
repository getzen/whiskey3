use macroquad::prelude::*;

use crate::view::transform::Transform;

use super::view_entity::ViewEntity;

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
pub struct Text {
    pub transform: Transform,
    pub text: String,
    pub font: Font,
    pub font_size: u16,
    pub font_scale: f32,
    pub align_h: AlignH,
    pub align_v: AlignV,
    pub color: Color,
}

impl Text {
    pub fn new(position: Vec2, text: &str, font: Font, font_size: u16) -> Self {
        Self {
            transform: Transform::from_translation(position),
            text: text.to_string(),
            font,
            font_size,
            font_scale: 1.0,
            align_h: AlignH::Center,
            align_v: AlignV::Center,
            color: WHITE,
        }
    }
}

impl ViewEntity for Text {
    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
    
    fn draw(&mut self, parent_transform: &Transform) {
        let transform = *parent_transform * self.transform;

        // Determine the adjustment we need to make for h/v centering.
        let mut offset = Vec2::ZERO;

        // Is this function slow?
        let dimensions = measure_text(&self.text, Some(&self.font), self.font_size, self.font_scale);

        offset.x += match self.align_h {
            AlignH::Left => 0.0,
            AlignH::Center => -dimensions.width * 0.5,
            AlignH::Right => -dimensions.width,
        };

        offset.y += match self.align_v {
            AlignV::Top => dimensions.offset_y,
            AlignV::Center => dimensions.offset_y * 0.5,
            AlignV::Bottom => 0.0,
        };

        let offset_trans = Transform::from_translation(offset);
        let adj_transform = Transform::from_multiplying(&transform, &offset_trans);
        let (trans, rot, _scale) = adj_transform.trans_rot_scale();

        let params = TextParams {
            font: Some(&self.font),
            font_size: self.font_size,
            font_scale: self.font_scale,
            font_scale_aspect: 1.0,
            rotation: rot,
            color: self.color,
        };
        draw_text_ex(&self.text, trans.x, trans.y, params);
    }
}
