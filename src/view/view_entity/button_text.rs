use macroquad::prelude::*;

use crate::controller::SENDER;
use crate::game::PlayerAction;
use crate::view::button_state::ButtonState;
use crate::view::eventer::{Eventer, HitDetector};
use crate::view::transform::Transform;

use super::rectangle::Rectangle;
use super::text::Text;
use super::view_entity::ViewEntity;

/// A button with drawn text and border. Always centered.
pub struct ButtonText {
    pub visible: bool,
    pub state: ButtonState,
    pub transform: Transform,
    pub text: Text,
    pub rectangle: Rectangle,

    pub normal_color: Color,
    pub highlighted_color: Color,
    pub disabled_color: Color,
    pub eventer: Eventer,
    pub click_action: Option<PlayerAction>,
}

impl ButtonText {
    pub fn new(position: Vec2, text: &str, font: Font, font_size: u16, size: Vec2) -> Self {
        let text = Text::new(Vec2::ZERO, text, font, font_size);

        Self {
            visible: true,
            state: ButtonState::Normal,
            transform: Transform::from_translation(position),
            text,
            rectangle: Rectangle::new(size, None, None, 2.0),
            normal_color: Color::from_rgba(220, 220, 220, 255),
            highlighted_color: Color::from_rgba(255, 255, 255, 255),
            disabled_color: Color::from_rgba(150, 150, 150, 255),
            eventer: Eventer::new(HitDetector::Rect(size, vec2(0.5, 0.5))),
            click_action: None,
        }
    }
}

impl ViewEntity for ButtonText {
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

        let color = match &self.state {
            ButtonState::Highlighted => self.highlighted_color,
            ButtonState::Disabled => self.disabled_color,
            _ => self.normal_color,
        };

        self.rectangle.stroke_color = Some(color);
        self.rectangle.draw(&transform);

        // let (pos, _rot, _scale) = transform.trans_rot_scale();

        // draw_rectangle_lines(pos.x, pos.y, self.size.x, self.size.y, 4.0, color);

        self.text.color = color;
        self.text.draw(&transform);
    }
}
