use macroquad::prelude::*;

pub struct MouseState {
    /// The mouse just entered.
    pub mouse_entered: bool,
    /// The mouse is currently over.
    pub mouse_over: bool,
    /// The mouse just exited.
    pub mouse_exited: bool,
    /// The left button was just pressed and mouse_over is true.
    pub left_button_pressed: bool,
    /// The left button is currently down and mouse_over is true.
    pub left_button_down: bool,
    /// The left button was just released and mouse_over is true.
    pub left_button_released: bool,
    // right_
    // dragging: bool, drag_start_pos, drag_pos_now
}

impl MouseState {
    pub fn new() -> Self {
        Self {
            mouse_entered: false,
            mouse_over: false,
            mouse_exited: false,
            left_button_pressed: false,
            left_button_down: false,
            left_button_released: false,
            //dragging: false,
        }
    }

    /// Keep the point argument for eventual dragging state.
    pub fn update(&mut self, contains_pt: bool, _point: &Vec2) {
        // These are reset each update:
        self.mouse_entered = false;
        self.mouse_exited = false;
        self.left_button_pressed = false;
        self.left_button_released = false;

        if contains_pt && !self.mouse_over {
            self.mouse_entered = true;
        }
        if !contains_pt && self.mouse_over {
            self.mouse_exited = true;
        }
        self.mouse_over = contains_pt;

        // Left button
        if contains_pt {
            let left_mouse_down = is_mouse_button_down(MouseButton::Left);
            if left_mouse_down && !self.left_button_down {
                self.left_button_pressed = true;
            } else if !left_mouse_down && self.left_button_down {
                self.left_button_released = true;
            }
            self.left_button_down = left_mouse_down;
        } else {
            self.left_button_down = false;
        }
    }
}
