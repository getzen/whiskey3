use macroquad::prelude::*;

use crate::card::Id;
use crate::card::Points;
use crate::controller::SENDER;
use crate::game::PlayerAction;
use crate::view::sprite::Sprite;

use super::eventer::Eventer;
use super::eventer::HitDetector;
use super::text::Text;
use super::transform::Transform;
use super::view::FONT;

pub struct CardEnt {
    pub id: Id,
    pub transform: Transform,
    pub sprite: Sprite,
    is_face_up: bool,
    pub face_texture: Texture2D,
    pub back_texture: Texture2D,

    pub dimmed_color: Color,
    pub dimmed: bool,
    
    pub eventer: Eventer,
    pub action: Option<PlayerAction>,

    pub point_text_transform: Transform,
    pub point_text: Option<Text>,
}

impl CardEnt {
    pub fn new(id: Id, face: Texture2D, back: Texture2D, points: Points) -> Self {
        let size_mult = 0.3333;
        let size = vec2(face.width() * size_mult, face.height() * size_mult);

        let font = FONT.get().unwrap();
        let point_text_transform = Transform::from_x_y(-15.0, 46.0);

        let mut point_text = None;
        if points > 0 {
            let text = format!("{} pts", points);
            let mut pt = Text::new(&text, font.clone(), 12);
            pt.color = GRAY;
            point_text = Some(pt);
        }

        Self {
            id,
            transform: Transform::new(),
            sprite: Sprite::new_with_size_mult(face.clone(), size_mult),
            is_face_up: true,
            face_texture: face,
            back_texture: back,
            dimmed_color: Color::from_rgba(200, 200, 200, 255),
            dimmed: false,

            point_text_transform,
            point_text,

            eventer: Eventer::new(HitDetector::Rect(size, vec2(0.5, 0.5))),
            action: None,
        }
    }

    pub fn set_translation(&mut self, translation: Vec2) {
        self.transform.translation = translation;
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.transform.rotation = rotation;
    }

    pub fn set_face_up(&mut self, face_up: bool) {
        self.is_face_up = face_up;
        self.sprite.texture = match face_up {
            true => self.face_texture.clone(),
            false => self.back_texture.clone(),
        }
    }

    /// Returns true if the sprite is visible and transform contains the mouse_pos.
    pub fn process_mouse(&mut self, mouse_pos: Vec2) -> bool {
        let mouse_over = self.eventer.process_mouse(&mouse_pos, &self.transform);

        if self.eventer.left_mouse_released {
            if let Some(sender) = SENDER.get() {
                if let Some(action) = &self.action {
                    sender.send(action.clone()).expect("Send error");
                }
            }
        }
        mouse_over
    }

    pub fn draw(&mut self) {
        self.sprite.color = match self.dimmed {
            true => self.dimmed_color,
            false => WHITE,
        };
        self.sprite.draw(&self.transform);

        if self.is_face_up {
            if let Some(point_text) = &self.point_text {
                point_text.draw(&self.transform);
            }
        }
    }
}
