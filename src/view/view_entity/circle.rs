use macroquad::prelude::*;

use crate::view::{transform::Transform, utility_graphics::circle_contains_point};

use super::view_entity::ViewEntity;

pub struct Circle {
    pub transform: Transform,
    pub radius: f32,
    pub fill_color: Option<Color>,
    pub stroke_color: Option<Color>,
    pub stroke_width: f32,
}

#[allow(unused)]
impl Circle {
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
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn contains_point(&mut self, point: &Vec2, parent_transform: &Transform) -> bool {
        let transform = *parent_transform * self.transform;
        let local_pt = transform.convert_point_to_local(point);
        circle_contains_point(Vec2::ZERO, self.radius, local_pt)
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
