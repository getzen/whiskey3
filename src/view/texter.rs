use macroquad::prelude::*;

use crate::view::transform::Transform;

pub struct Texter {
    pub transform: Transform,
    pub visible: bool,
    pub centered_horiz: bool,
    pub centered_vert: bool,
    pub text: String,
    pub color: Color,
    pub font: Option<Font>,
    pub font_size: u16,
    pub font_scale: f32,
}

impl Texter {
    pub async fn new(
        text: &str,
        font_size: u16,
        font_name: Option<&str>,
        centered_horiz: bool,
        centered_vert: bool,
    ) -> Self {
        let font = match font_name {
            Some(name) => {
                let path = format!("./src/assets/{}", name);
                Some(load_ttf_font(&path).await.unwrap())
            }
            None => None,
        };

        Self {
            transform: Transform::new(Vec2::ZERO, 0.0),
            visible: true,
            centered_horiz,
            centered_vert,
            text: text.to_string(),
            color: BLACK,
            font,
            font_size,
            font_scale: 1.0,
        }
    }

    /// Returns the size of the drawn text: width, height, y offset from baseline.
    pub fn draw_size(&self) -> (f32, f32, f32) {
        let font = match &self.font {
            Some(font) => Some(font),
            None => None,
        };
        let dimensions = measure_text(&self.text, font, self.font_size, self.font_scale);
        (dimensions.width, dimensions.height, dimensions.offset_y)
    }

    pub fn draw(&mut self, parent_trans: Option<Transform>) {
        if !self.visible {
            return;
        }

        let (parent_pos, _parent_rot) = match parent_trans {
            Some(trans) => (trans.position, trans.rotation),
            None => (Vec2::ZERO, 0.0),
        };

        let mut pos = self.transform.position + parent_pos;

        // Is this function slow?
        let (width, _height, offset_y) = self.draw_size();

        //draw_rectangle(pos.x - 1.0, pos.y - 1.0, 2.0, 2.0, RED);

        if self.centered_horiz {
            pos.x -= width * 0.5;
        }

        if self.centered_vert {
            pos.y += offset_y * 0.5;
        }

        let font = match &self.font {
            Some(font) => Some(font),
            None => None,
        };

        let params = TextParams {
            font,
            font_size: self.font_size,
            font_scale: self.font_scale,
            font_scale_aspect: 1.0,
            rotation: self.transform.rotation,
            color: self.color,
        };
        draw_text_ex(&self.text, pos.x, pos.y, params);
    }
}
