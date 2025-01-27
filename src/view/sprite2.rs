use std::sync::mpsc::Sender;

use macroquad::color::{Color, WHITE};
use macroquad::prelude::Texture2D;
use macroquad::texture::draw_texture_ex;
use macroquad::{math::Vec2, texture::DrawTextureParams};

use crate::game::PlayerAction;
use crate::view::trans::Trans;

use super::animators::{RotationAnimator, TranslationAnimator};
use super::eventer2::Eventer2;

pub struct Sprite2 {
    pub visible: bool,
    pub transform: Trans,
    pub texture: Texture2D,
    pub color: Color,
    pub trans_anim: Option<TranslationAnimator>,
    pub angle_anim: Option<RotationAnimator>,
    pub eventer: Eventer2,
    pub sender: Option<Sender<PlayerAction>>,
    pub action: Option<PlayerAction>,
}

impl Sprite2 {
    pub fn new(texture: Texture2D, size_mult: f32) -> Self {
        let size = Vec2::new(texture.width() * size_mult, texture.height() * size_mult);
        let mut transform = Trans::new();
        transform.center_with_size(size);

        Self {
            visible: true,
            transform,
            texture,
            color: WHITE,
            trans_anim: None,
            angle_anim: None,
            eventer: Eventer2::new(),
            sender: None,
            action: None,
        }
    }

    pub fn move_to(&mut self, end_position: Vec2, velocity: f32) {
        self.trans_anim = Some(TranslationAnimator::new(
            self.transform.translation,
            end_position,
            velocity,
        ));
    }

    #[allow(unused)]
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

    /// Returns true if the sprite's transform contains the mouse_pos.
    pub fn process_events(&mut self, parent_transform: Option<&Trans>, mouse_pos: Vec2) -> bool {
        let transform = match parent_transform {
            Some(parent) => &Trans::combine(parent, &self.transform),
            None => &self.transform,
        };

        let mouse_over = self.eventer.process_events(transform, mouse_pos);

        if self.eventer.left_mouse_released {
            println!("left mouse released");
            // Send action if one exists. Dimmed cards should not have actions.
            if let Some(sender) = &self.sender {
                if let Some(action) = &self.action {
                    sender.send(action.clone()).expect("Send error");
                }
            }
        }
        mouse_over
    }

    pub fn draw(&self, parent_transform: Option<&Trans>) {
        if !self.visible {
            return;
        }

        let transform = match parent_transform {
            Some(parent) => &Trans::combine(parent, &self.transform),
            None => &self.transform,
        };

        let (pos, rot) = transform.drawable_position_rotation();

        let params = DrawTextureParams {
            dest_size: Some(transform.size),
            rotation: rot,
            ..Default::default()
        };

        draw_texture_ex(&self.texture, pos.x, pos.y, self.color, params);
    }
}
