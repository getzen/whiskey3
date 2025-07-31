use crate::{card::{Card, Points, Rank, Suit}, game_options::JokerKind};

#[derive(Clone)]
pub struct Trick {
    pub cards: Vec<Option<Card>>,
    pub lead_suit: Option<Suit>,
    pub winner: Option<usize>,
    pub winning_suit: Option<Suit>,
    pub winning_rank: Rank,
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
            lead_suit: None,
            winner: None,
            winning_suit: None,
            winning_rank: 0,
            points: 0,
        }
    }

    pub fn reset(&mut self) {
        for card in &mut self.cards {
            *card = None;
        }
        self.lead_suit = None;
        self.winner = None;
        self.winning_suit = None;
        self.winning_rank = 0;
        self.points = 0;
    }

    pub fn add(&mut self, player: usize, card: Card, trump_suit: &Option<Suit>, joker_kind: JokerKind) {
        self.points += card.points;
        let card_suit = card.suit;
        let card_rank = card.rank;
        self.cards[player] = Some(card);

        if self.lead_suit.is_none() {
            self.winner = Some(player);
            if joker_kind == JokerKind::Phoenix && card_suit == Suit::Joker {
                // Nothing left to do.
                return;
            }
            self.lead_suit = Some(card_suit); // This should never be Suit::Joker.
            self.winning_suit = Some(card_suit); // This should never be Suit::Joker.
            self.winning_rank = card_rank;
            // Nothing left to do.
            return;
        }
        
        // A winning suit has been set.
        let winning_suit = self.winning_suit.unwrap();

        // Special handling
        if joker_kind == JokerKind::Phoenix && card_suit == Suit::Joker {
            // Joker automatically takes the lead, but winning suit and rank do not change.
            self.winner = Some(player);
            return;
        }

        let mut new_winner = false;

        if let Some(trump_suit) = trump_suit {
            if winning_suit == *trump_suit && card_suit == *trump_suit {
                // Note the >= sign below.
                if card_rank >= self.winning_rank {
                    new_winner = true;
                }
            } else {
                // winning card is not trump
                if card_suit == *trump_suit {
                    new_winner = true;
                } else {
                    if card_suit == winning_suit && card_rank > self.winning_rank {
                        new_winner = true;
                    }
                }
            }
        } else {
            // Hand does not have a trump suit.
            if card_suit == winning_suit && card_rank > self.winning_rank {
               new_winner = true;
            }
        }

        if new_winner {
            self.winner = Some(player);
            self.winning_suit = Some(card_suit);
            self.winning_rank = card_rank;
        }
    }

    pub fn completed(&self) -> bool {
        !self.cards.contains(&None)
    }
}
