use macroquad::prelude::*;

use super::transform::Transform;

#[allow(unused)]
pub enum HitDetector {
    Rect(Vec2, Vec2),   // size, anchor
    Circle(f32),        // radius
    Polygon(Vec<Vec2>), // points
}

pub struct Eventer {
    /// Will check for events. If false, process_events() always returns false.
    pub enabled: bool,
    /// The type of hit detection to use.
    pub hit_detector: HitDetector,
    /// The mouse just entered.
    pub mouse_entered: bool,
    /// The mouse is currently over.
    pub mouse_over: bool,
    /// The mouse just exited.
    pub mouse_exited: bool,
    /// The left mouse was just pressed and mouse_over is true.
    pub left_mouse_pressed: bool,
    /// The left mouse is currently down and mouse_over is true.
    pub left_mouse_down: bool,
    /// The left mouse was just released and mouse_over is true.
    pub left_mouse_released: bool,
    // right_
    // dragging: bool, drag_start_pos, drag_pos_now
}

impl Eventer {
    pub fn new(hit_detector: HitDetector) -> Self {
        Self {
            enabled: true,
            hit_detector,
            mouse_entered: false,
            mouse_over: false,
            mouse_exited: false,
            left_mouse_pressed: false,
            left_mouse_down: false,
            left_mouse_released: false,
            //dragging: false,
        }
    }

    pub fn process_mouse(&mut self, position: &Vec2, transform: &Transform) -> bool {
        if !self.enabled {
            return false;
        }

        // These are reset each update:
        self.mouse_entered = false;
        self.mouse_exited = false;
        self.left_mouse_pressed = false;
        self.left_mouse_released = false;

        let mouse_over = self.contains_point(position, transform);

        if mouse_over && !self.mouse_over {
            self.mouse_entered = true;
        }
        if !mouse_over && self.mouse_over {
            self.mouse_exited = true;
        }
        self.mouse_over = mouse_over;

        // Left button
        if mouse_over {
            let left_mouse_down = is_mouse_button_down(MouseButton::Left);
            if left_mouse_down && !self.left_mouse_down {
                self.left_mouse_pressed = true;
            } else if !left_mouse_down && self.left_mouse_down {
                self.left_mouse_released = true;
            }
            self.left_mouse_down = left_mouse_down;
        } else {
            self.left_mouse_down = false;
        }
        mouse_over
    }

    /// Returns true if the given point is contained in HitDetector.
    pub fn contains_point(&self, point: &Vec2, transform: &Transform) -> bool {
        // Get the adjusted test point relative to the translation.
        let mut adj_pt = vec2(point.x - transform.translation.x, point.y - transform.translation.y);

        // Rotate the point clockwise. (Notan and Macroquad both rotate clockwise).
        let theta = transform.rotation;
        adj_pt = vec2(
            adj_pt.x * f32::cos(theta) + adj_pt.y * f32::sin(theta),
            -adj_pt.x * f32::sin(theta) + adj_pt.y * f32::cos(theta),
        );

        match &self.hit_detector {
            HitDetector::Rect(size, anchor) => {
                let adj_size = *size * transform.scale;
                adj_pt.x += adj_size.x * anchor.x;
                adj_pt.y += adj_size.y * anchor.y;
                adj_pt.x >= 0.0 && adj_pt.x <= adj_size.x && adj_pt.y >= 0.0 && adj_pt.y <= adj_size.y
            }
            HitDetector::Circle(radius) => {
                let adj_radius = radius * transform.scale.x;
                // adj_pt.x -= radius * anchor.x;
                // adj_pt.y -= radius * anchor.y;
                Vec2::ZERO.distance(adj_pt) <= adj_radius
            }
            HitDetector::Polygon(_points) => todo!(),
        }
    }
}
