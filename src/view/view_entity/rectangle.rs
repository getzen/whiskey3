use macroquad::prelude::*;

use crate::view::transform::Transform;

use super::view_entity::ViewEntity;

#[allow(unused)]
pub struct Rectangle {
    pub transform: Transform,
    pub size: Vec2,
    pub anchor: Vec2,
    pub fill_color: Option<Color>,
    pub stroke_color: Option<Color>,
    pub stroke_width: f32,
}

impl Rectangle {
    pub fn new(size: Vec2, fill_color: Option<Color>, stroke_color: Option<Color>, stroke_width: f32) -> Self {
        Self {
            transform: Transform::new(),
            size,
            anchor: Vec2::new(0.5, 0.5),
            fill_color,
            stroke_color,
            stroke_width,
        }
    }
}

impl ViewEntity for Rectangle {
    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn draw(&mut self, parent_transform: &Transform) {
        let transform = *parent_transform * self.transform;
        let (pos, rot, scale) = transform.trans_rot_scale();
        let size = self.size * scale;

        if let Some(color) = self.fill_color {
            let params = DrawRectangleParams {
                rotation: rot,
                color,
                offset: self.anchor,
            };
            draw_rectangle_ex(pos.x, pos.y, size.x, size.y, params);
        }

        if let Some(color) = self.stroke_color {
            let params = DrawRectangleParams {
                rotation: rot,
                color,
                offset: self.anchor,
            };
            draw_rectangle_lines_ex(pos.x, pos.y, size.x, size.y, self.stroke_width, params);
        }
    }
}
