use std::sync::mpsc::Sender;

use macroquad::color::GRAY;
use macroquad::color::WHITE;
use macroquad::math::vec2;
use macroquad::math::Vec2;
use macroquad::prelude::Color;
use macroquad::prelude::Texture2D;

use crate::card::Points;
use crate::game::PlayerAction;
use crate::view::imager::Imager;

use crate::view::animators::TranslationAnimator;

use super::animators::RotationAnimator;
use super::eventer::Eventer;
use super::texter::AlignH;
use super::texter::AlignV;
use super::texter::Texter;
use super::transform::Transform;
use super::view::BODY_FONT;

pub struct CardView {
    pub id: u8, // must match Card id
    pub transform: Transform,
    pub card_image: Imager,
    pub point_text: Option<Texter>,
    pub eventer: Eventer,

    is_face_up: bool,
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
    pub fn new(
        id: u8,
        face: Texture2D,
        back: Texture2D,
        points: Points,
        sender: Sender<PlayerAction>,
    ) -> Self {
        let size_mult = 0.3333;
        let size = vec2(face.width() * size_mult, face.height() * size_mult);

        let font = BODY_FONT.lock().unwrap().clone().unwrap();
        let text_pos = vec2(-22.0, 44.0);

        let mut point_text = None;
        if points > 0 {
            let text = format!("{} pts", points);
            let mut pt = Texter::new(text_pos, &text, font, 12, AlignH::Center, AlignV::Bottom);
            pt.color = GRAY;
            pt.transform.rotation = -std::f32::consts::PI * 0.5;
            point_text = Some(pt);
        }

        Self {
            id,
            transform: Transform::from_translation_size_centered(Vec2::ZERO, size, true),
            card_image: Imager::new(face.clone(), size_mult, true),
            point_text,
            eventer: Eventer::new(),
            is_face_up: true,
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
        self.is_face_up = face_up;
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
    pub fn process_events(
        &mut self,
        parent_transform: Option<&Transform>,
        mouse_pos: Vec2,
    ) -> bool {
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

        if self.is_face_up {
            if let Some(point_text) = &self.point_text {
                point_text.draw(Some(&self.transform));

                // let transform = Transform::combine(&self.transform, &point_text.transform);
                // let (pos, _rot) = transform.drawable_position_rotation();
                // draw_circle(pos.x, pos.y, 3., RED);
            }
        }
    }
}
