use macroquad::{
    color::{Color, WHITE},
    math::{Vec2, vec4},
    shapes::draw_circle_lines,
};

use crate::{
    game::Bid,
    view::{transform::Transform, view::FONT},
};

use super::{text::Text, view_entity::ViewEntity};

pub struct BidMarker {
    pub visible: bool,
    transform: Transform,
    radius: f32,
    text: Text,

    current_bid: Option<Bid>,
    color: Color,
    color_change_dur: f32,
}

impl BidMarker {
    pub fn new(position: Vec2) -> Self {
        let font = FONT.get().unwrap();
        let text = Text::new(Vec2::ZERO, "?", font.clone(), 18);

        Self {
            visible: false,
            transform: Transform::from_translation(position),
            radius: 30.0,
            text,

            current_bid: None,
            color: WHITE,
            color_change_dur: 0.0,
        }
    }

    pub fn update_with_bid(&mut self, opt_bid: Option<Bid>) {
        if let Some(bid_new) = &opt_bid {
            if let Some(bid_old) = &self.current_bid {
                if bid_new == bid_old {
                    return;
                }
            }
        }

        match &opt_bid {
            Some(bid) => {
                self.visible = true;
                self.color = macroquad::color::GREEN;
                self.color_change_dur = 1.0;
                match bid {
                    Bid::Pass => self.text.text = "Pass".to_string(),
                    Bid::Points(bid) => {
                        self.text.text = bid.to_string();
                    }
                }
            }
            None => self.visible = false,
        }
        self.current_bid = opt_bid;
    }
}

impl ViewEntity for BidMarker {
    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn update(&mut self, time_delta: f32) {
        if self.color_change_dur > 0.0 {
            let b = 1.0 - self.color_change_dur;
            let r = 1.0 - self.color_change_dur;
            self.color = Color::from_vec(vec4(r, 1.0, b, 1.0));
            self.color_change_dur -= time_delta;
            return;
        }
        self.color = WHITE;
    }

    fn draw(&mut self, _parent_transform: &Transform) {
        if !self.visible {
            return;
        }

        let (pos, _rot, _scale) = self.transform.trans_rot_scale();

        // Circles are already centered.
        draw_circle_lines(pos.x, pos.y, self.radius, 3.0, self.color);

        // Text is already centered vert and horiz.
        self.text.draw(&self.transform);
    }
}
