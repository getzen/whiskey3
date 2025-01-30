use std::sync::mpsc::Sender;

use macroquad::math::Vec2;
use macroquad::prelude::Color;
use macroquad::shapes::draw_rectangle_lines;
use macroquad::text::Font;

use crate::game::PlayerAction;
use crate::view::button_state::ButtonState;
use crate::view::eventer2::Eventer2;

use super::texter::AlignH;
use super::texter::AlignV;
use super::texter::Texter;
use super::trans::Trans;

/// A button with drawn text and border. Always centered.
pub struct ButtonText {
    pub transform: Trans,
    pub text: Texter,
    pub size: Vec2,
    pub eventer: Eventer2,
    pub state: ButtonState,
    pub normal_color: Color,
    pub highlighted_color: Color,
    pub disabled_color: Color,
    pub sender: Option<Sender<PlayerAction>>,
    pub action: Option<PlayerAction>,
}

impl ButtonText {
    pub fn new(position: Vec2, text: &str, font: Font, font_size: u16, size: Vec2) -> Self {
        let text = Texter::new(position, text, font, font_size, AlignH::Center, AlignV::Center);

        let mut transform = Trans::from_translation(position);
        transform.center_with_size(size);

        Self {
            transform,
            text,
            size,
            eventer: Eventer2::new(),
            state: ButtonState::Normal,
            normal_color: Color::from_rgba(220, 220, 220, 255),
            highlighted_color: Color::from_rgba(255, 255, 255, 255),
            disabled_color: Color::from_rgba(150, 150, 150, 255),
            sender: None,
            action: None,
        }
    }

    /// Returns true if the sprite is visible and transform contains the mouse_pos.
    pub fn process_events(&mut self, parent_transform: Option<&Trans>, mouse_pos: Vec2) -> bool {
        let transform = match parent_transform {
            Some(parent) => &Trans::combine(parent, &self.transform),
            None => &self.transform,
        };

        let mouse_over = self.eventer.process_events(transform, mouse_pos);

        if self.eventer.mouse_entered {
            self.state = ButtonState::Highlighted;
        }

        if self.eventer.mouse_exited {
            self.state = ButtonState::Normal;
        }

        if self.eventer.left_mouse_pressed {
            self.state = ButtonState::Highlighted;
        }

        if self.eventer.left_mouse_released {
            self.state = ButtonState::Normal;

            // Send action if sender and action exist.
            if let Some(sender) = &self.sender {
                if let Some(action) = &self.action {
                    sender.send(action.clone()).expect("Send error");
                }
            }
        }
        mouse_over
    }

    pub fn draw(&mut self, parent_transform: Option<&Trans>) {
        if self.state == ButtonState::Hidden {
            return;
        }

        let transform = match parent_transform {
            Some(parent) => &Trans::combine(parent, &self.transform),
            None => &self.transform,
        };

        let color = match &self.state {
            ButtonState::Highlighted => self.highlighted_color,
            ButtonState::Disabled => self.disabled_color,
            _ => self.normal_color,
        };

        let (pos, _rot) = transform.drawable_position_rotation();

        draw_rectangle_lines(pos.x, pos.y, self.size.x, self.size.y, 4.0, color);

        self.text.color = color;
        self.text.draw(Some(transform));
    }
}
