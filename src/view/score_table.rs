use macroquad::math::Vec2;

use crate::{card::Points, game::Bid};

use super::texter::Texter;

pub struct ScoreTable {
    heading: Texter,
    score: Texter,
    bid: Texter,
    hand: Texter,
}

impl ScoreTable {
    pub async fn new(mut position: Vec2) -> Self {
        let font_size = 14;
        let y_spacing = 18.0;
        let mut heading = Texter::new(
            "         We   They",
            14,
            Some("Menlo-Bold.ttf"),
            false,
            false,
        )
        .await;
        heading.transform.position = position;
        position.y += y_spacing;

        let mut score = Texter::new("", font_size, Some("Menlo-Bold.ttf"), false, false).await;
        score.transform.position = position;
        position.y += y_spacing;

        let mut bid = Texter::new("", font_size, Some("Menlo-Bold.ttf"), false, false).await;
        bid.transform.position = position;
        position.y += y_spacing;

        let mut hand = Texter::new("", font_size, Some("Menlo-Bold.ttf"), false, false).await;
        hand.transform.position = position;
        position.y += y_spacing;

        Self {
            heading,
            score,
            bid,
            hand,
        }
    }

    pub fn update(
        &mut self,
        scores: &[Points; 2],
        maker: &Option<usize>,
        bid: &Option<Bid>,
        hand: &[Points; 2],
    ) {
        self.score.text = format!("Score    {:>2}    {:>2}", scores[0], scores[1]);

        let mut bid0 = "-".to_string();
        let mut bid1 = "-".to_string();
        match maker {
            Some(m) => {
                let points = match bid {
                    Some(bid) => match bid {
                        Bid::Pass => panic!(),
                        Bid::Bid(p) => p,
                    },
                    None => panic!(),
                };
                match m {
                    0 | 2 => bid0 = format!("{:>2}", points),
                    1 | 3 => bid1 = format!("{:>2}", points),
                    _ => panic!(),
                }
            }
            _ => {}
        }
        self.bid.text = format!("Bid      {}     {}", bid0, bid1);

        self.hand.text = format!("Hand     {:>2}    {:>2}", hand[0], hand[1]);
    }

    pub fn draw(&mut self) {
        self.heading.draw();
        self.score.draw();
        self.bid.draw();
        self.hand.draw();
    }
}
