use macroquad::prelude::*;

pub struct MouseState {
    /// The last mouse position supplied by the update() fn.
    pub mouse_position: Vec2,
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
    /// The mouse position has changed while the left button is down.
    /// Dragging will remain true while the button remains down, even if
    /// mouse_over becomes false. This is so a control like a slider will
    /// work correctly even if the mouse moves off the control.
    pub dragging: bool,
    pub drag_start_position: Option<Vec2>,
    /// The current position less the drag_start_position.
    pub drag_delta: Option<Vec2>
}

impl MouseState {
    pub fn new() -> Self {
        Self {
            mouse_position: Vec2::ZERO,
            mouse_entered: false,
            mouse_over: false,
            mouse_exited: false,
            left_button_pressed: false,
            left_button_down: false,
            left_button_released: false,
            dragging: false,
            drag_start_position: None,
            drag_delta: None,
        }
    }

    /// Keep the point argument for eventual dragging state.
    pub fn update(&mut self, contains_pt: bool, point: &Vec2) {
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
        let left_mouse_down = is_mouse_button_down(MouseButton::Left);
        if contains_pt {
            if left_mouse_down && !self.left_button_down {
                self.left_button_pressed = true;
            } else if !left_mouse_down && self.left_button_down {
                self.left_button_released = true;
            }
            self.left_button_down = left_mouse_down;
        } else {
            self.left_button_down = false;
        }

        // Dragging
        if !self.dragging {
            if self.left_button_down {
                if self.drag_start_position.is_none() {
                    if *point != self.mouse_position {
                        self.drag_start_position = Some(*point);
                        self.drag_delta = Some(Vec2::ZERO);
                    }
                } else { 
                    self.dragging = true;
                    //println!("start drag");
                }
            }
        } else {
            if !left_mouse_down {
                self.dragging = false;
                //println!("end drag");
                self.drag_start_position = None;
            } else {
                self.drag_delta = Some(*point - self.drag_start_position.unwrap());
                //println!("dragging");
            }
        }

        self.mouse_position = *point;
    }
}


// Sample drag handling in a fn like process_mouse(). It requires the object
// to have a field called drag_start_pos. It's needed so that the 'grab' point
// for the object remains the same as its position is updated.
/*
let contains_pt = self.contains_point(point, parent_transform);
self.mouse_state.update(contains_pt, point);

if self.mouse_state.dragging {
    if self.drag_start_pos.is_none() {
        // Remember the position of this obj when the drag starts.
        self.drag_start_pos = Some(self.rect.transform.translation);
    }
    let drag_delta = self.mouse_state.drag_delta.unwrap();
    // Set the current translation as the start position plus the delta.
    self.rect.transform.translation = self.drag_start_pos.unwrap() + drag_delta;
} else {
    self.drag_start_pos = None;
}
*/
