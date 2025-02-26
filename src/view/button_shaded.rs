use std::sync::mpsc::Sender;

use macroquad::math::Vec2;
use macroquad::prelude::Color;
use macroquad::prelude::Texture2D;

use crate::game::PlayerAction;
use crate::view::button_state::ButtonState;
use crate::view::eventer::Eventer;
use crate::view::imager::Imager;
use crate::view::transform_old::Transform;

/// A button that uses a single texture with color shades to show the ButtonState.
pub struct ButtonShaded {
    pub visible: bool,
    pub transform: Transform,
    pub image: Imager,
    pub eventer: Eventer,
    pub state: ButtonState,
    pub normal_color: Color,
    pub highlighted_color: Color,
    pub disabled_color: Color,
    pub sender: Option<Sender<PlayerAction>>,
    pub action: Option<PlayerAction>,
}

impl ButtonShaded {
    pub fn new(position: Vec2, texture: Texture2D, size_mult: f32) -> Self {
        let size = Vec2::new(texture.width() * size_mult, texture.height() * size_mult);
        Self {
            visible: true,
            transform: Transform::from_translation_size_centered(position, size, true),
            image: Imager::new(texture, size_mult, true),
            eventer: Eventer::new(),
            state: ButtonState::Normal,
            normal_color: Color::from_rgba(230, 230, 230, 255),
            highlighted_color: Color::from_rgba(255, 255, 255, 255),
            disabled_color: Color::from_rgba(100, 100, 100, 255),
            sender: None,
            action: None,
        }
    }

    /// Returns true if the sprite is visible and transform contains the mouse_pos.
    pub fn process_events(&mut self, parent_transform: Option<&Transform>, mouse_pos: Vec2) -> bool {
        let transform = match parent_transform {
            Some(parent) => &Transform::combine(parent, &self.transform),
            None => &self.transform,
        };

        let mouse_over = self.eventer.process_events(transform, mouse_pos);

        if self.state == ButtonState::Disabled {
            return mouse_over;
        }

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

    pub fn draw(&mut self, parent_transform: Option<&Transform>) {
        if !self.visible {
            return;
        }

        let transform = match parent_transform {
            Some(parent) => &Transform::combine(parent, &self.transform),
            None => &self.transform,
        };

        self.image.color = match &self.state {
            ButtonState::Highlighted => self.highlighted_color,
            ButtonState::Disabled => self.disabled_color,
            _ => self.normal_color,
        };
        self.image.draw(Some(transform));
    }
}
