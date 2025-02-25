use macroquad::prelude::*;

use super::transform::Transform;

pub struct Eventer {
    /// Will check for events. If false, process_events() always returns false.
    pub enabled: bool,
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
    pub fn new() -> Self {
        Self {
            enabled: true,
            mouse_entered: false,
            mouse_over: false,
            mouse_exited: false,
            left_mouse_pressed: false,
            left_mouse_down: false,
            left_mouse_released: false,
            //dragging: false,
        }
    }

    pub fn process_events(&mut self, transform: &Transform, mouse_pos: Vec2) -> bool {
        if !self.enabled {
            return false;
        }

        // These are reset each update:
        self.mouse_entered = false;
        self.mouse_exited = false;
        self.left_mouse_pressed = false;
        self.left_mouse_released = false;

        // Mouse over
        let mouse_over = transform.contains_point(mouse_pos);
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
}
