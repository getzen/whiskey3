use macroquad::prelude::*;

use super::view_entity::ViewEntity;
use crate::view::transform::Transform;

#[allow(unused)]
pub struct Circle {
    pub transform: Transform,
    pub radius: f32,
    pub fill_color: Option<Color>,
    pub stroke_color: Option<Color>,
    pub stroke_width: f32,
}

impl Circle {
    #[allow(unused)]
    pub fn new(radius: f32, fill_color: Option<Color>, stroke_color: Option<Color>, stroke_width: f32) -> Self {
        Self {
            transform: Transform::new(),
            radius,
            fill_color,
            stroke_color,
            stroke_width,
        }
    }
}

impl ViewEntity for Circle {
    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn draw(&mut self, parent_transform: &Transform) {
        let transform = *parent_transform * self.transform;
        let (pos, _rot, scale) = transform.trans_rot_scale();

        let adj_radius = self.radius * scale.x;

        if let Some(color) = self.fill_color {
            draw_circle(pos.x, pos.y, adj_radius, color);
        }

        if let Some(color) = self.stroke_color {
            draw_circle_lines(pos.x, pos.y, adj_radius, self.stroke_width, color);
        }
    }
}
