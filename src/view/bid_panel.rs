use macroquad::{
    color::{BLACK, GRAY, GREEN, WHITE},
    math::{vec2, Vec2},
    shapes::{draw_rectangle, draw_rectangle_lines},
};

use crate::game::{Bid, PlayerAction};

use super::{button_text::ButtonText, eventer::EventerEvent, texter::Texter, transform::Transform};

pub struct BidPanel {
    pub min_bid: u8,
    max_bid: u8,
    current_bid: u8,
    bid_increment: u8,

    pub visible: bool,
    size: Vec2,
    transform: Transform,
    bid_button: ButtonText,
    pass_button: ButtonText,
    plus_button: ButtonText,
    minus_button: ButtonText,
    bid_text: Texter,
}

impl BidPanel {
    pub async fn new(min_bid: u8, max_bid: u8, position: Vec2) -> Self {
        let bid_button = ButtonText::new(
            0,
            position + vec2(0.0, 0.0),
            "Bid",
            18,
            Some("Menlo-Bold.ttf"),
            vec2(80.0, 40.0),
        )
        .await;

        let pass_button = ButtonText::new(
            0,
            position + vec2(100.0, 0.0),
            "Pass",
            18,
            Some("Menlo-Bold.ttf"),
            vec2(80.0, 40.0),
        )
        .await;

        let plus_button = ButtonText::new(
            0,
            position + vec2(-65.0, -12.0),
            "+",
            18,
            Some("Menlo-Bold.ttf"),
            vec2(20.0, 20.0),
        )
        .await;

        let minus_button = ButtonText::new(
            0,
            position + vec2(-65.0, 12.0),
            "-",
            18,
            Some("Menlo-Bold.ttf"),
            vec2(20.0, 20.0),
        )
        .await;

        let mut bid_text = Texter::new("---", 18, Some("Menlo-Bold.ttf"), true, true).await;
        bid_text.transform.position = position + vec2(-100.0, 0.0);
        bid_text.color = WHITE;

        Self {
            min_bid,
            max_bid,
            current_bid: min_bid,
            bid_increment: 5,
            visible: true,
            size: vec2(250.0, 100.0),
            transform: Transform::new(position, 0.0),
            bid_button,
            pass_button,
            plus_button,
            minus_button,
            bid_text,
        }
    }

    pub fn update_bid_amount(&mut self, new_amount: u8) {
        self.current_bid = new_amount;
        self.bid_text.text = format!("{}", new_amount);
    }

    pub fn process_events(&mut self, mouse_pos: &Vec2) -> Option<PlayerAction> {
        if !self.visible {
            return None;
        }

        if self.bid_button.process_events(mouse_pos) {
            return Some(PlayerAction::Bid(Bid::Bid(self.current_bid)));
        }

        if self.pass_button.process_events(mouse_pos) {
            return Some(PlayerAction::Bid(Bid::Pass));
        }

        if self.plus_button.process_events(mouse_pos) {
            let new_amount = self.max_bid.min(self.current_bid + self.bid_increment);
            self.update_bid_amount(new_amount);
        }

        if self.minus_button.process_events(mouse_pos) {
            let new_amount = self.min_bid.max(self.current_bid - self.bid_increment);
            self.update_bid_amount(new_amount);
        }

        None
    }

    pub fn draw(&mut self) {
        if !self.visible {
            return;
        }

        let (mut pos, _rot) = self.transform.combined_pos_rot();
        pos.x -= self.size.x / 2.0;
        pos.y -= self.size.y / 2.0;

        //draw_rectangle(pos.x, pos.y, self.size.x, self.size.y, );

        draw_rectangle_lines(pos.x, pos.y, self.size.x, self.size.y, 4.0, GREEN);

        self.bid_button.draw();
        self.pass_button.draw();
        self.bid_text.draw();
        self.plus_button.draw();
        self.minus_button.draw();
    }
}
