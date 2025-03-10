use macroquad::prelude::*;

use crate::controller::SENDER;
use crate::game::PlayerAction;
use crate::view::mouse_state::MouseState;
use crate::view::transform::Transform;
use crate::view::utility_graphics::rect_contains_point;

use super::button_state::ButtonState;
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
    pub mouse_state: MouseState,
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
            mouse_state: MouseState::new(),
            click_action: None,
        }
    }
}

impl ViewEntity for ButtonText {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn contains_point(&mut self, point: &Vec2, parent_transform: &Transform) -> bool {
        let transform = *parent_transform * self.transform;
        let mut local_pt = transform.convert_point_to_local(point);
        local_pt += self.rectangle.anchor * self.rectangle.size;
        rect_contains_point(Vec2::ZERO, self.rectangle.size, local_pt)
    }

    fn process_mouse(&mut self, point: &Vec2, parent_transform: &Transform) -> bool {
        if !self.visible {
            return false;
        }

        let contains_pt = self.contains_point(point, parent_transform);
        self.mouse_state.update(contains_pt, point);

        if self.state == ButtonState::Disabled {
            return contains_pt;
        }

        if self.mouse_state.mouse_entered {
            self.state = ButtonState::Highlighted;
        }

        if self.mouse_state.mouse_exited {
            self.state = ButtonState::Normal;
        }

        if self.mouse_state.left_button_pressed {
            self.state = ButtonState::Highlighted;
        }

        if self.mouse_state.left_button_released {
            self.state = ButtonState::Normal;
            if let Some(sender) = SENDER.get() {
                if let Some(action) = &self.click_action {
                    sender.send(action.clone()).expect("Send error");
                }
            }
        }
        contains_pt
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
