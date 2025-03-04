use macroquad::math::Vec2;
use macroquad::math::vec2;
use macroquad::prelude::Color;
use macroquad::prelude::Texture2D;

use crate::controller::SENDER;
use crate::game::PlayerAction;
use crate::view::eventer::Eventer;
use crate::view::eventer::HitDetector;
use crate::view::transform::Transform;

use super::button_state::ButtonState;
use super::sprite::Sprite;
use super::view_entity::ViewEntity;

/// A button that uses a single texture with color shades to show the ButtonState.
pub struct ButtonShaded {
    pub visible: bool,
    pub state: ButtonState,
    pub transform: Transform,
    pub sprite: Sprite,
    pub normal_color: Color,
    pub highlighted_color: Color,
    pub disabled_color: Color,
    pub eventer: Eventer,
    pub click_action: Option<PlayerAction>,
}

impl ButtonShaded {
    pub fn new(position: Vec2, texture: Texture2D, size_mult: f32, click_action: Option<PlayerAction>) -> Self {
        let size = texture.size() * size_mult;
        Self {
            visible: true,
            state: ButtonState::Normal,
            transform: Transform::from_translation(position),
            sprite: Sprite::new_with_size_mult(texture, size_mult),
            normal_color: Color::from_rgba(230, 230, 230, 255),
            highlighted_color: Color::from_rgba(255, 255, 255, 255),
            disabled_color: Color::from_rgba(100, 100, 100, 255),
            eventer: Eventer::new(HitDetector::Rect(size, vec2(0.5, 0.5))),
            click_action,
        }
    }
}

impl ViewEntity for ButtonShaded {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn process_mouse(&mut self, mouse_pos: &Vec2, parent_transform: &Transform) -> bool {
        if !self.visible {
            return false;
        }

        let transform = *parent_transform * self.transform;
        let mouse_over = self.eventer.process_mouse(mouse_pos, &transform);

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
            if let Some(sender) = SENDER.get() {
                if let Some(action) = &self.click_action {
                    sender.send(action.clone()).expect("Send error");
                }
            }
        }
        mouse_over
    }

    fn draw(&mut self, parent_transform: &Transform) {
        if !self.visible {
            return;
        }

        let transform = *parent_transform * self.transform;
        self.sprite.color = match &self.state {
            ButtonState::Highlighted => self.highlighted_color,
            ButtonState::Disabled => self.disabled_color,
            _ => self.normal_color,
        };
        self.sprite.draw(&transform);
    }
}
