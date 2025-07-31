use macroquad::prelude::*;

use crate::view::transform::Transform;

use super::view_entity::ViewEntity;


pub struct ProgressBar {
    pub transform: Transform,
    pub size: Vec2,
    pub centered: bool,
    pub outline_color: Color,
    pub fill_color: Color,
    pub progress: f32,
}

#[allow(unused)]
impl ProgressBar {
    pub fn new(position: Vec2, size: Vec2, progress: f32) -> Self {
        Self {
            transform: Transform::from_translation(position),
            size,
            centered: true,
            outline_color: Color::from_rgba(130, 130, 130, 255),
            fill_color: Color::from_rgba(0, 255, 0, 255),
            progress,
        }
    }
}

impl ViewEntity for ProgressBar {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn draw(&mut self, parent_transform: &Transform) {
        let transform = *parent_transform * self.transform;
        let (mut pos, rot, scale) = transform.trans_rot_scale();
        let size = self.size * scale;

        if self.centered {
            pos.x -= size.x * 0.5;
            pos.y -= size.y * 0.5;
        }

        let params = DrawRectangleParams {
            rotation: rot,
            color: self.fill_color,
            offset: Vec2::new(0., 0.),
        };
        let progress_size = Vec2::new(size.x * self.progress, size.y);
        draw_rectangle_ex(pos.x, pos.y, progress_size.x, progress_size.y, params);

        let params = DrawRectangleParams {
            rotation: rot,
            color: self.outline_color,
            offset: Vec2::new(0., 0.),
        };
        draw_rectangle_lines_ex(pos.x, pos.y, size.x, size.y, 1.0, params);
        
    }
}
