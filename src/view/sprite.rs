use macroquad::math::Vec2;
use macroquad::prelude::Texture2D;

use crate::view::imager::Imager;
use crate::view::transform::Transform;

use super::animators::{RotationAnimator, TranslationAnimator};

pub struct Sprite {
    pub visible: bool,
    pub transform: Transform,
    pub imager: Imager,
    pub trans_anim: Option<TranslationAnimator>,
    pub angle_anim: Option<RotationAnimator>,
}

impl Sprite {
    pub fn new(texture: Texture2D, tex_size_multiplier: f32) -> Self {
        Self {
            visible: true,
            transform: Transform::new(Vec2::ZERO, 0.0),
            imager: Imager::new(texture, tex_size_multiplier, true),
            trans_anim: None,
            angle_anim: None,
        }
    }

    pub fn move_to(&mut self, end_position: Vec2, velocity: f32) {
        self.trans_anim = Some(TranslationAnimator::new(
            self.transform.position,
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
            self.transform.position = translator.update(time_delta);
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

    pub fn draw(&mut self) {
        if !self.visible { return }
        self.imager.draw(&self.transform, None);
    }
}
