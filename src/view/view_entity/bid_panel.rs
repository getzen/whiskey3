use macroquad::math::{Vec2, vec2};

use crate::{
    card::Points,
    game::{Bid, PlayerAction},
    view::{transform::Transform, view::FONT},
};

use super::{button_text::ButtonText, text::Text, view_entity::ViewEntity};

pub struct BidPanel {
    pub min_bid: Points,
    pub max_bid: Points,
    current_bid: Points,
    bid_increment: Points,

    pub visible: bool,
    transform: Transform,
    bid_button: ButtonText,
    pass_button: ButtonText,
    plus_button: ButtonText,
    minus_button: ButtonText,
    bid_text: Text,
}

impl BidPanel {
    pub fn new(position: Vec2, min_bid: Points, max_bid: Points) -> Self {
        let font = FONT.get().unwrap();

        let mut bid_button = ButtonText::new(vec2(-5.0, 0.0), "Bid", font.clone(), 18, vec2(80.0, 40.0));
        bid_button.click_action = Some(PlayerAction::Bid(Bid::Points(min_bid)));

        let mut pass_button = ButtonText::new(vec2(100.0, 0.0), "Pass", font.clone(), 18, vec2(80.0, 40.0));
        pass_button.click_action = Some(PlayerAction::Bid(Bid::Pass));

        let plus_button = ButtonText::new(vec2(-70.0, -12.0), "+", font.clone(), 18, vec2(20.0, 20.0));

        let minus_button = ButtonText::new(vec2(-70.0, 12.0), "-", font.clone(), 18, vec2(20.0, 20.0));

        let min_text = min_bid.to_string();
        let bid_text = Text::new(vec2(-105.0, 0.0), &min_text, font.clone(), 18);

        Self {
            min_bid,
            max_bid,
            current_bid: min_bid,
            bid_increment: 5,
            visible: false,
            transform: Transform::from_translation(position),
            bid_button,
            pass_button,
            plus_button,
            minus_button,
            bid_text,
        }
    }

    pub fn update_bid_amount(&mut self, new_amount: Points) {
        self.current_bid = new_amount;
        self.bid_text.text = format!("{}", new_amount);
        self.bid_button.click_action = Some(PlayerAction::Bid(Bid::Points(new_amount)));
    }
}

impl ViewEntity for BidPanel {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn process_mouse(&mut self, point: &Vec2, _parent_transform: &Transform) -> bool {
        if !self.visible {
            return false;
        }

        let mut mouse_over = self.bid_button.process_mouse(point, &self.transform);
        mouse_over = mouse_over || self.pass_button.process_mouse(point, &self.transform);

        mouse_over = mouse_over || self.plus_button.process_mouse(&point, &self.transform);
        if self.plus_button.mouse_state.left_button_released {
            let new_amount = self.max_bid.min(self.current_bid + self.bid_increment);
            self.update_bid_amount(new_amount);
        }

        mouse_over = mouse_over || self.minus_button.process_mouse(point, &self.transform);
        if self.minus_button.mouse_state.left_button_released {
            let new_amount = self.min_bid.max(self.current_bid - self.bid_increment);
            self.update_bid_amount(new_amount);
        }

        mouse_over
    }

    fn draw(&mut self, _parent_transform: &Transform) {
        if !self.visible {
            return;
        }

        self.bid_button.draw(&self.transform);
        self.pass_button.draw(&self.transform);
        self.bid_text.draw(&self.transform);
        self.plus_button.draw(&self.transform);
        self.minus_button.draw(&self.transform);
    }
}
