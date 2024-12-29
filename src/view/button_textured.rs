use macroquad::math::Vec2;
use macroquad::prelude::Color;
use macroquad::prelude::Texture2D;

use crate::view::eventer::Eventer;
use crate::view::eventer::EventerEvent;
use crate::view::imager::Imager;
use crate::view::transform::Transform;

#[derive(Clone, Copy, PartialEq)]
pub enum ButtonState {
    Normal,
    Highlighted,
    Disabled,
    Hidden,
}

/// A button that uses individual textures to show the ButtonState.
pub struct ButtonTextured {
    pub id: u8,
    pub transform: Transform,
    pub image: Imager,
    pub eventer: Eventer,
    pub state: ButtonState,
    state_internal: ButtonState,

    pub normal_tex: Texture2D,
    pub highlighted_tex: Option<Texture2D>,
    pub disabled_tex: Option<Texture2D>,

    pub alt_color: Option<Color>,
    pub use_alt_color: bool,
}

impl ButtonTextured {
    pub fn new(id: u8, pos: Vec2, normal_tex: Texture2D, highlighted_tex: Option<Texture2D>, disabled_tex: Option<Texture2D>, tex_size_multiplier: f32) -> Self {
        Self {
            id,
            transform: Transform::new(pos, 0.0),
            image: Imager::new(normal_tex.clone(), tex_size_multiplier, true),
            eventer: Eventer::new(),
            state: ButtonState::Normal,
            state_internal: ButtonState::Normal,
            normal_tex,
            highlighted_tex,
            disabled_tex,
            alt_color: Some(Color::from_rgba(200, 255, 200, 255)),
            use_alt_color: false,
        }
    }

    fn check_state(&mut self) {
        if self.state == self.state_internal {
            return;
        }
        self.state_internal = self.state;

        match &self.state {
            ButtonState::Normal => {
                self.image.texture = self.normal_tex.clone()
            },
            ButtonState::Highlighted => {
                if let Some(tex) = &self.highlighted_tex {
                    self.image.texture = tex.clone();
                }
            },
            ButtonState::Disabled => {
                if let Some(tex) = &self.disabled_tex {
                    self.image.texture = tex.clone();
                }
            },
            _ => {},
        }
    }

    #[allow(dead_code)]
    pub fn contains_point(&self, point: &Vec2) -> bool {
        let size = self.image.draw_size();
        let centered = self.image.centered;
        self.eventer
            .contains_point(point, &self.transform, size, centered)
    }

    pub fn process_events(&mut self, mouse_pos: &Vec2) -> Option<EventerEvent> {
        if self.state == ButtonState::Disabled || self.state == ButtonState::Hidden {
            return None;
        }
        let size = self.image.draw_size();
        let centered = self.image.centered;
        let event = self.eventer.process_events(mouse_pos, &self.transform, size, centered);
        if event.is_none() {
            return event;
        }

        // Handle possible state change before returning event.
        self.state = match event.as_ref().unwrap() {
            EventerEvent::LeftMousePressed => ButtonState::Highlighted,
            _ => ButtonState::Normal,
        };
        self.check_state();
        event
    }

    pub fn draw(&mut self) {
        if self.state == ButtonState::Hidden {
            return;
        }
        if self.use_alt_color {
            self.image.draw(&self.transform, self.alt_color);
        } else {
            self.image.draw(&self.transform, None);
        }
    }
}
