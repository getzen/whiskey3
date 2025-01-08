use macroquad::{color::WHITE, math::Vec2, shapes::draw_circle_lines};

use crate::game::Bid;

use super::{texter::Texter, transform::Transform};

pub struct BidMarker {
    pub visible: bool,
    transform: Transform,
    radius: f32,
    text: Texter,
}

impl BidMarker {
    pub async fn new(position: Vec2) -> Self {
        let mut text = Texter::new("--", 18, Some("Menlo-Bold.ttf"), true, true).await;
        text.transform.position = position;

        Self {
            visible: false,
            transform: Transform::new(position, 0.0),
            radius: 30.0,
            text,
        }
    }

    pub fn update_with_bid(&mut self, opt_bid: Option<Bid>) {
        match opt_bid {
            Some(bid) => {
                self.visible = true;
                match bid {
                    Bid::Pass => self.text.text = "Pass".to_string(),
                    Bid::Bid(bid) => {
                        self.text.text = bid.to_string();
                    }
                }
            }
            None => self.visible = false,
        }
    }

    pub fn draw(&mut self) {
        if !self.visible {
            return;
        }

        let pos = self.transform.position;

        // Circles are already centered.
        draw_circle_lines(pos.x, pos.y, self.radius, 3.0, WHITE);

        // Text is already centered vert and horiz.
        self.text.draw();
    }
}
