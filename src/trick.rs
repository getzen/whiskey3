use crate::card::{Card, Points, Suit};

#[derive(Clone)]
pub struct Trick {
    pub cards: Vec<Option<Card>>,
    pub lead_card: Option<Card>,
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
            lead_card: None,
            winner: None,
            points: 0,
        }
    }

    pub fn reset(&mut self) {
        for card in &mut self.cards {
            *card = None;
        }
        self.lead_card = None;
        self.points = 0;
    }

    pub fn is_empty(&self) -> bool {
        self.lead_card.is_none()
    }

    pub fn add(&mut self, player: usize, card: Card, trump_suit: &Option<Suit>) {
        if self.lead_card.is_none() {
            // this is the lead card
            self.lead_card = Some(card.clone());
            self.winner = Some(player);
        } else {
            // this is not the lead card
            let winning_player = self.winner.unwrap();
            let winning_card = self.cards[winning_player].as_ref().unwrap();

            // Hand has a trump suit.
            if trump_suit.is_some() {
                if winning_card.is_trump(trump_suit) && card.is_trump(trump_suit) {
                    if card.rank > winning_card.rank {
                        self.winner = Some(player);
                    }
                } else {
                    // winning card is not trump
                    if card.is_trump(trump_suit) {
                        self.winner = Some(player);
                    } else {
                        if card.suit == winning_card.suit {
                            if card.rank > winning_card.rank {
                                self.winner = Some(player);
                            }
                        }
                    }
                }
            } else {
                // Hand does not have a trump suit.
                if card.suit == winning_card.suit {
                    if card.rank > winning_card.rank {
                        self.winner = Some(player);
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
