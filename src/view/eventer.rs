use macroquad::prelude::*;

use crate::view::transform::Transform;

#[derive(Debug)]
pub enum EventerEvent {
    MouseEntered,
    MouseExited,
    LeftMousePressed,
    LeftMouseReleased,
    // Right
    // DragStarted
}

pub struct Eventer {
    pub enabled: bool,
    pub mouse_over: bool,
    pub left_mouse_down: bool,
    // right_
    // dragging: bool, drag_start_pos, drag_pos_now
}

impl Eventer {
    pub fn new() -> Self {
        Self {
            enabled: true,
            mouse_over: false,
            left_mouse_down: false,
        }
    }

    /// Test whether the point lies in the texture rectangle, considering rotation.
    /// Note: Macroquad's mouse_position() gives the physical location of the mouse.
    pub fn contains_point(
        &self,
        point: &Vec2,
        transform: &Transform,
        size: Vec2,
        centered: bool,
    ) -> bool {
        let (pos, rot) = transform.combined_pos_rot();

        // Get the net test point relative to the sprite's position.
        let net_x = point.x - pos.x;
        let net_y = point.y - pos.y;
        // Rotate the point clockwise (the same direction as Macroquad's rotation). This is a
        // little different than the standard rotation formulas.
        let theta = rot;
        let rot_x = net_x * f32::cos(theta) + net_y * f32::sin(theta);
        let rot_y = -net_x * f32::sin(theta) + net_y * f32::cos(theta);
        // See if the rotated point is in the unrotated sprite rectangle.
        if centered {
            f32::abs(rot_x) <= size.x / 2.0 && f32::abs(rot_y) <= size.y / 2.0
        } else {
            rot_x >= 0.0 && rot_x <= size.x && rot_y >= 0.0 && rot_y <= size.y
        }
    }

    pub fn process_events(
        &mut self,
        mouse_pos: &Vec2,
        transform: &Transform,
        size: Vec2,
        centered: bool,
    ) -> Option<EventerEvent> {
        if !self.enabled {
            return None;
        }

        let mouse_over = self.contains_point(mouse_pos, transform, size, centered);

        if mouse_over && !self.mouse_over {
            self.mouse_over = true;
            return Some(EventerEvent::MouseEntered);
        }

        if !mouse_over && self.mouse_over {
            self.mouse_over = false;
            return Some(EventerEvent::MouseExited);
        }

        let left_mouse_down = is_mouse_button_down(MouseButton::Left);

        if mouse_over && left_mouse_down && !self.left_mouse_down {
            self.left_mouse_down = true;
            return Some(EventerEvent::LeftMousePressed);
        }
        self.left_mouse_down = false;

        if mouse_over && is_mouse_button_released(MouseButton::Left) {
            return Some(EventerEvent::LeftMouseReleased);
        }
        None
    }
}
