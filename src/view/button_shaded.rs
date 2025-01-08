use macroquad::math::Vec2;
use macroquad::prelude::Color;
use macroquad::prelude::Texture2D;

use crate::view::button_state::ButtonState;
use crate::view::eventer::Eventer;
use crate::view::eventer::EventerEvent;
use crate::view::imager::Imager;
use crate::view::transform::Transform;

/// A button that uses a single texture with color shades to show the ButtonState.
pub struct ButtonShaded {
    pub id: u8,
    pub transform: Transform,
    pub image: Imager,
    pub eventer: Eventer,
    pub state: ButtonState,
    pub normal_color: Color,
    pub highlighted_color: Color,
    pub disabled_color: Color,
}

impl ButtonShaded {
    pub fn new(id: u8, pos: Vec2, texture: Texture2D, tex_size_multiplier: f32) -> Self {
        Self {
            id,
            transform: Transform::new(pos, 0.0),
            image: Imager::new(texture, tex_size_multiplier, true),
            eventer: Eventer::new(),
            state: ButtonState::Normal,
            normal_color: Color::from_rgba(230, 230, 230, 255),
            highlighted_color: Color::from_rgba(255, 255, 255, 255),
            disabled_color: Color::from_rgba(100, 100, 100, 255),
        }
    }

    #[allow(dead_code)]
    pub fn contains_point(&self, point: &Vec2) -> bool {
        let size = self.image.draw_size();
        let centered = self.image.centered;
        self.eventer
            .contains_point(point, &self.transform, size, centered)
    }

    /// Check if event occurred, handle internally, and return true if clicked.
    pub fn process_events(&mut self, mouse_pos: &Vec2) -> bool {
        if self.state == ButtonState::Disabled || self.state == ButtonState::Hidden {
            return false;
        }
        let size = self.image.draw_size();
        let centered = self.image.centered;
        let event = self
            .eventer
            .process_events(mouse_pos, &self.transform, size, centered);

        if event.is_none() {
            return false;
        }

        // Handle possible state change before returning event.
        match event.as_ref().unwrap() {
            EventerEvent::LeftMousePressed => {
                self.state = ButtonState::Highlighted;
            }
            EventerEvent::LeftMouseReleased => {
                self.state = ButtonState::Normal;
                return true;
            }
            
            EventerEvent::MouseEntered => {
                self.state = ButtonState::Highlighted;
            },
            EventerEvent::MouseExited => {
                self.state = ButtonState::Normal;
            },
            _ => self.state = ButtonState::Normal,
        };
        false
    }

    pub fn draw(&mut self) {
        if self.state == ButtonState::Hidden {
            return;
        }
        let color = match &self.state {
            ButtonState::Normal => Some(self.normal_color),
            ButtonState::Highlighted => Some(self.highlighted_color),
            ButtonState::Disabled => Some(self.disabled_color),
            _ => None,
        };
        self.image.draw(&self.transform, color);
    }
}
