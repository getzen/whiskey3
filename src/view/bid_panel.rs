use std::sync::mpsc::Sender;

use macroquad::{
    math::{vec2, Vec2},
    text::load_ttf_font,
};

use crate::{
    card::Points,
    game::{Bid, PlayerAction},
};

use super::{
    button_text::ButtonText,
    texter::{AlignH, AlignV, Texter},
    trans::Trans,
};

pub struct BidPanel {
    pub min_bid: Points,
    max_bid: Points,
    current_bid: Points,
    bid_increment: Points,

    pub visible: bool,
    size: Vec2,
    transform: Trans,
    bid_button: ButtonText,
    pass_button: ButtonText,
    plus_button: ButtonText,
    minus_button: ButtonText,
    bid_text: Texter,
    sender: Sender<PlayerAction>,
}

impl BidPanel {
    pub async fn new(
        min_bid: Points,
        max_bid: Points,
        position: Vec2,
        sender: Sender<PlayerAction>,
    ) -> Self {
        let font = load_ttf_font("./src/assets/Menlo-Bold.ttf").await.unwrap();

        let mut bid_button = ButtonText::new(
            position + vec2(-5.0, 0.0),
            "Bid",
            font.clone(),
            18,
            vec2(80.0, 40.0),
        );
        bid_button.sender = Some(sender.clone());
        bid_button.action = Some(PlayerAction::Bid(Bid::Points(min_bid)));

        let mut pass_button = ButtonText::new(
            position + vec2(100.0, 0.0),
            "Pass",
            font.clone(),
            18,
            vec2(80.0, 40.0),
        );
        pass_button.sender = Some(sender.clone());
        pass_button.action = Some(PlayerAction::Bid(Bid::Pass));

        let plus_button = ButtonText::new(
            position + vec2(-70.0, -12.0),
            "+",
            font.clone(),
            18,
            vec2(20.0, 20.0),
        );

        let minus_button = ButtonText::new(
            position + vec2(-70.0, 12.0),
            "-",
            font.clone(),
            18,
            vec2(20.0, 20.0),
        );

        let min_text = min_bid.to_string();
        let pos = position + vec2(-105.0, 0.0);
        let bid_text = Texter::new(
            pos,
            &min_text,
            font.clone(),
            18,
            AlignH::Center,
            AlignV::Center,
        );

        Self {
            min_bid,
            max_bid,
            current_bid: min_bid,
            bid_increment: 5,
            visible: false,
            size: vec2(250.0, 100.0),
            transform: Trans::from_translation(position),
            bid_button,
            pass_button,
            plus_button,
            minus_button,
            bid_text,
            sender,
        }
    }

    pub fn update_bid_amount(&mut self, new_amount: Points) {
        self.current_bid = new_amount;
        self.bid_text.text = format!("{}", new_amount);
        self.bid_button.action = Some(PlayerAction::Bid(Bid::Points(new_amount)));
    }

    /// Returns true if event found.
    pub fn process_events(&mut self, mouse_pos: Vec2) -> bool {  // ADD PARENT TRANS HERE AND FOR OTHER STRUCTS
        if !self.visible {
            return false;
        }

        let mouse_over0 = self.bid_button.process_events(Some(&self.transform), mouse_pos);
        let mouse_over1 = self.pass_button.process_events(Some(&self.transform), mouse_pos);

        let mouse_over2 = self.plus_button.process_events(Some(&self.transform), mouse_pos);
        if self.plus_button.eventer.left_mouse_released {
            let new_amount = self.max_bid.min(self.current_bid + self.bid_increment);
            self.update_bid_amount(new_amount);
        }
        
        let mouse_over3 = self.minus_button.process_events(Some(&self.transform), mouse_pos);
        if self.minus_button.eventer.left_mouse_released {
            let new_amount = self.min_bid.max(self.current_bid - self.bid_increment);
            self.update_bid_amount(new_amount);
        }
 
        mouse_over0 || mouse_over1 || mouse_over2 || mouse_over3
    }

    pub fn draw(&mut self, parent_transform: Option<&Trans>) {
        if !self.visible {
            return;
        }

        let transform = match parent_transform {
            Some(parent) => &Trans::combine(parent, &self.transform),
            None => &self.transform,
        };

        self.bid_button.draw(Some(transform));
        self.pass_button.draw(Some(transform));
        self.bid_text.draw(Some(transform));
        self.plus_button.draw(Some(transform));
        self.minus_button.draw(Some(transform));
    }
}
