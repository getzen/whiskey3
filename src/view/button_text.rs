use std::sync::mpsc::Sender;

use macroquad::math::Vec2;
use macroquad::prelude::Color;
use macroquad::shapes::draw_rectangle_lines;

use crate::game::PlayerAction;
use crate::view::button_state::ButtonState;
use crate::view::eventer::Eventer;
use crate::view::eventer::EventerEvent;
use crate::view::transform::Transform;

use super::texter::Texter;

/// A button with drawn text and border. Always centered.
pub struct ButtonText {
    pub id: u8,
    pub transform: Transform,
    pub text: Texter,
    pub size: Vec2,
    pub eventer: Eventer,
    pub state: ButtonState,
    pub normal_color: Color,
    pub highlighted_color: Color,
    pub disabled_color: Color,
    pub sender: Option<Sender<PlayerAction>>,
    pub player_action: Option<PlayerAction>,
}

impl ButtonText {
    pub async fn new(
        id: u8,
        pos: Vec2,
        text: &str,
        font_size: u16,
        font_name: Option<&str>,
        size: Vec2,
    ) -> Self {
        let mut text = Texter::new(text, font_size, font_name, true, true).await;
        text.transform.position = pos;

        Self {
            id,
            transform: Transform::new(pos, 0.0),
            text,
            size,
            eventer: Eventer::new(),
            state: ButtonState::Normal,
            normal_color: Color::from_rgba(220, 220, 220, 255),
            highlighted_color: Color::from_rgba(255, 255, 255, 255),
            disabled_color: Color::from_rgba(150, 150, 150, 255),
            sender: None,
            player_action: None,
        }
    }

    #[allow(dead_code)]
    pub fn contains_point(&self, point: &Vec2) -> bool {
        self.eventer
            .contains_point(point, &self.transform, self.size, true)
    }

    /// Check if event occurred, handle internally, and return true if clicked.
    pub fn process_events(&mut self, mouse_pos: &Vec2) -> bool {
        if self.state == ButtonState::Disabled || self.state == ButtonState::Hidden {
            return false;
        }
        let event = self
            .eventer
            .process_events(mouse_pos, &self.transform, self.size, true);
        
        if event.is_none() {
            return false;
        }

        // Handle possible state change before returning event.
        match event.as_ref().unwrap() {
            EventerEvent::MouseEntered => {
                self.state = ButtonState::Highlighted;
            }
            EventerEvent::LeftMousePressed => {
                self.state = ButtonState::Normal;
            }
            EventerEvent::LeftMouseReleased => {
                self.state = ButtonState::Highlighted;
                // Send action if sender and action exist.
                if let Some(sender) = &self.sender {
                    if let Some(action) = &self.player_action {
                        sender.send(action.clone()).expect("Send error");
                    }
                }
                return true;
            }
            _ => {
                self.state = ButtonState::Normal;
            }
        }
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

        if color.is_none() {
            return;
        }

        let pos = self.transform.centered_position(self.size);

        draw_rectangle_lines(pos.x, pos.y, self.size.x, self.size.y, 4.0, color.unwrap());

        // Text is already centered vert and horiz.
        self.text.color = color.unwrap();
        self.text.draw();
    }
}
