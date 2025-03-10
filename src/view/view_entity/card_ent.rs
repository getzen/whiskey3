use macroquad::prelude::*;

use crate::card::Points;
use crate::controller::SENDER;
use crate::game::PlayerAction;
use crate::view::mouse_state::MouseState;
use crate::view::transform::Transform;
use crate::view::utility_graphics::rect_contains_point;
use crate::view::view::FONT;

use super::sprite::Sprite;
use super::text::Text;
use super::view_entity::ViewEntity;

pub struct CardEnt {
    pub transform: Transform,

    pub sprite: Sprite,
    is_face_up: bool,
    pub face_texture: Texture2D,
    pub back_texture: Texture2D,
    pub dimmed: bool,
    pub dimmed_color: Color,

    pub point_text: Option<Text>,

    pub mouse_state: MouseState,
    pub action: Option<PlayerAction>,
}

impl CardEnt {
    pub fn new(face: Texture2D, back: Texture2D, points: Points) -> Self {
        let size_mult = 0.3333;
        let font = FONT.get().unwrap();

        let mut point_text = None;
        if points > 0 {
            let text = format!("{} pts", points);
            let mut pt = Text::new(vec2(-15.0, 46.0), &text, font.clone(), 12);
            pt.color = GRAY;
            point_text = Some(pt);
        }

        Self {
            transform: Transform::new(),
            sprite: Sprite::new_with_size_mult(face.clone(), size_mult),
            is_face_up: true,
            face_texture: face,
            back_texture: back,
            dimmed: false,
            dimmed_color: Color::from_rgba(200, 200, 200, 255),

            point_text,

            mouse_state: MouseState::new(),
            action: None,
        }
    }

    pub fn set_face_up(&mut self, face_up: bool) {
        self.is_face_up = face_up;
        self.sprite.texture = match face_up {
            true => self.face_texture.clone(),
            false => self.back_texture.clone(),
        }
    }
}

impl ViewEntity for CardEnt {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn set_translation(&mut self, translation: Vec2) {
        self.transform.translation = translation;
    }

    fn set_rotation(&mut self, rotation: f32) {
        self.transform.rotation = rotation;
    }

    fn contains_point(&mut self, point: &Vec2, parent_transform: &Transform) -> bool {
        let transform = *parent_transform * self.transform;
        let mut local_pt = transform.convert_point_to_local(point);
        local_pt += self.sprite.anchor * self.sprite.size;
        rect_contains_point(Vec2::ZERO, self.sprite.size, local_pt)
    }

    fn process_mouse(&mut self, point: &Vec2, parent_transform: &Transform) -> bool {
        let contains_pt = self.contains_point(point, parent_transform);
        self.mouse_state.update(contains_pt, point);

        if self.mouse_state.left_button_released {
            if let Some(sender) = SENDER.get() {
                if let Some(action) = &self.action {
                    sender.send(action.clone()).expect("Send error");
                }
            }
        }
        contains_pt
    }

    fn draw(&mut self, _parent_transform: &Transform) {
        self.sprite.color = match self.dimmed {
            true => self.dimmed_color,
            false => WHITE,
        };
        self.sprite.draw(&self.transform);

        if self.is_face_up {
            if let Some(point_text) = &mut self.point_text {
                point_text.draw(&self.transform);
            }
        }
    }
}
