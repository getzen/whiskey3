use std::sync::mpsc::Sender;

use macroquad::color::WHITE;
use macroquad::math::Vec2;
use macroquad::prelude::Color;
use macroquad::prelude::Texture2D;

use crate::game::PlayerAction;
use crate::view::imager::Imager;

use crate::view::animators::TranslationAnimator;

use super::animators::RotationAnimator;
use super::eventer::Eventer;
use super::transform::Transform;

pub struct CardView {
    pub id: u8, // must match Card id
    pub transform: Transform,
    pub card_image: Imager,
    pub eventer: Eventer,

    pub face_texture: Texture2D,
    pub back_texture: Texture2D,

    pub dimmed_color: Color,
    pub dimmed: bool,

    pub trans_anim: Option<TranslationAnimator>,
    pub angle_anim: Option<RotationAnimator>,

    sender: Sender<PlayerAction>,
    pub action: Option<PlayerAction>,
}

impl CardView {
    pub fn new(id: u8, face: Texture2D, back: Texture2D, sender: Sender<PlayerAction>) -> Self {
        Self {
            id,
            transform: Transform::new(),
            card_image: Imager::new(face.clone(), 0.3333, true),
            eventer: Eventer::new(),
            face_texture: face,
            back_texture: back,
            dimmed_color: Color::from_rgba(200, 200, 200, 255),
            dimmed: false,
            trans_anim: None,
            angle_anim: None,
            sender,
            action: None,
        }
    }

    pub fn set_face_up(&mut self, face_up: bool) {
        self.card_image.texture = match face_up {
            true => self.face_texture.clone(),
            false => self.back_texture.clone(),
        }
    }

    pub fn move_to(&mut self, end_position: Vec2, velocity: f32) {
        self.trans_anim = Some(TranslationAnimator::new(
            self.transform.translation,
            end_position,
            velocity,
        ));
    }

    pub fn rotate_to(&mut self, end_radian: f32, velocity: f32) {
        self.angle_anim = Some(RotationAnimator::new(
            self.transform.rotation,
            end_radian,
            velocity,
        ));
    }

    pub fn update(&mut self, time_delta: f32) {
        if let Some(translator) = &mut self.trans_anim {
            self.transform.translation = translator.update(time_delta);
            if translator.completed {
                self.trans_anim = None;
            }
        }

        if let Some(rotator) = &mut self.angle_anim {
            self.transform.rotation = rotator.update(time_delta);
            if rotator.completed {
                self.angle_anim = None;
            }
        }
    }

    /// Returns true if the sprite is visible and transform contains the mouse_pos.
    pub fn process_events(&mut self, parent_transform: Option<&Transform>, mouse_pos: Vec2) -> bool {
        let transform = match parent_transform {
            Some(parent) => &Transform::combine(parent, &self.transform),
            None => &self.transform,
        };

        let mouse_over = self.eventer.process_events(transform, mouse_pos);

        if self.eventer.left_mouse_released {
            // Send action if sender and action exist.
            if let Some(action) = &self.action {
                self.sender.send(action.clone()).expect("Send error");
            }
        }
        mouse_over
    }

    pub fn draw(&mut self) {
        self.card_image.color = match self.dimmed {
            true => self.dimmed_color,
            false => WHITE,
        };
        self.card_image.draw(Some(&self.transform));
    }
}
