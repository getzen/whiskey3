use crate::card::{Card, CardSuit, Points};

#[derive(Clone)]
pub struct Trick {
    pub cards: Vec<Option<Card>>,
    pub lead_card_suit: Option<CardSuit>,
    pub winner: Option<usize>,
    pub points: Points,
}

impl Trick {
    pub fn new(player_count: usize) -> Self {
        let mut cards = Vec::with_capacity(player_count);
        for _ in 0..player_count {
            cards.push(None);
        }

        Self {
            cards,
            lead_card_suit: None,
            winner: None,
            points: 0,
        }
    }

    pub fn reset(&mut self) {
        for card in &mut self.cards {
            *card = None;
        }
        self.lead_card_suit = None;
        self.winner = None;
        self.points = 0;
    }

    pub fn is_empty(&self) -> bool {
        self.lead_card_suit.is_none()
    }

    pub fn add(&mut self, player: usize, card: Card, trump_suit: &Option<CardSuit>) {
        if self.lead_card_suit.is_none() {
            // this is the lead card
            self.lead_card_suit = Some(card.suit);
            self.winner = Some(player);
        } else {
            // this is not the lead card
            let winning_player = self.winner.unwrap();
            let winning_card = self.cards[winning_player].as_ref().unwrap();
            if card.suit == winning_card.suit {
                if card.rank > winning_card.rank {
                    self.winner == Some(player);
                }
            } else {
                // Not the same suit as winning card
                if let Some(trump_suit) = trump_suit {
                    if card.suit == *trump_suit {
                        self.winner == Some(player);
                    }
                }
            }
        }
        self.points += card.points;
        self.cards[player] = Some(card);
    }

    pub fn completed(&self) -> bool {
        !self.cards.contains(&None)
    }
}
