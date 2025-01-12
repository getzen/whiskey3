use std::sync::mpsc::Sender;

use crate::{
    card::{Card, Suit, Points},
    game::{Bid, Game, PlayerAction, PLAYERS},
};

#[derive(Clone)]
pub struct BotMonte {}

impl BotMonte {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_bid(&self, game: &Game, min: Points, max: Points, sender: Sender<PlayerAction>) {
        let mut bid = Bid::Pass;
        let mut bid_pts = 50;
        let cards = game.active_hand();
        for card in cards {
            if card.is_joker {
                bid_pts += 20;
            }
            match card.rank {
                14 => bid_pts += 20,
                13 => bid_pts += 15,
                _ => {}
            }
        }

        if bid_pts >= min {
            bid_pts = min;
            //bid_pts = bid_pts.min(max);
            bid = Bid::Bid(bid_pts);
        }

        sender.send(PlayerAction::Bid(bid)).expect("send error");
    }

    fn best_suit(&self, cards: &[Card]) -> Suit {
        const SUITS: [Suit; 4] = [
            Suit::Club,
            Suit::Diamond,
            Suit::Heart,
            Suit::Spade,
        ];
        let mut best_suit_score = 0;
        let mut best_suit = Suit::Club;

        for suit in SUITS.iter() {
            let mut suit_score = 0;

            for card in cards {
                if card.suit == *suit {
                    suit_score += card.rank;
                }
            }
            if suit_score > best_suit_score {
                best_suit_score = suit_score;
                best_suit = *suit;
            }
        }

        best_suit
    }

    fn lowest_non_trump_card(&self, cards: &[Card], trump: &Option<Suit>) -> u8 {
        let mut lowest_rank = 99;
        let mut lowest_id = 0;

        for card in cards {
            if !card.is_trump(&trump) && card.rank < lowest_rank {
                lowest_rank = card.rank;
                lowest_id = card.id;
            }
        }
        lowest_id
    }

    pub fn choose_discards(&self, game: &Game, nest_size: usize, sender: Sender<PlayerAction>) {
        // Super basic: dump the three lowest non-trump cards.

        // Make a copy of the hand cards.
        let mut cards_copy = Vec::new();
        for card in game.active_hand() {
            cards_copy.push(card.clone());
        }
        let mut discards = Vec::new();

        let trump = self.best_suit(&cards_copy);

        while discards.len() < nest_size {
            let lowest_card_id = self.lowest_non_trump_card(&cards_copy, &Some(trump));
            discards.push(lowest_card_id);
            if let Some(idx) = cards_copy.iter().position(|c| c.id == lowest_card_id) {
                cards_copy.swap_remove(idx);
            }
        }
        sender
            .send(PlayerAction::Discard(discards))
            .expect("send error");
    }

    pub fn choose_trump(&self, game: &Game, sender: Sender<PlayerAction>) {
        let cards = game.active_hand();
        let suit = self.best_suit(&cards);
        sender
            .send(PlayerAction::ChooseTrump(suit))
            .expect("send error");
    }

    // Use a MonteCarlo simulation to pick the best card.
    pub fn best_card_play(
        &self,
        game: &mut Game,
        simulations: usize,
        sender: Sender<PlayerAction>,
    ) {
        println!("bot thinking");

        let monte_player = game.active;
        let team = game.team_index(game.active);
        let opp_team = game.opponent_index(game.active);
        let mut best_score = i32::MIN;

        let legal_card_ids = game.get_playable_card_ids();
        let mut best_play = &legal_card_ids[0];
        if legal_card_ids.len() == 1 {
            sender
                .send(PlayerAction::PlayCard(best_play.clone()))
                .expect("send error");
            return;
        }

        // Create a vec with all the cards we don't know about.
        let mut hidden_cards = Vec::new();

        for p in 0..PLAYERS {
            if p == game.active {
                continue;
            }
            // Push hand cards to hidden_cards. We don't want to clear the hand using
            // append() since we need to know the hand len later.
            for card in &game.hands[p] {
                hidden_cards.push(card);
            }
            for card in &game.deck {
                hidden_cards.push(card);
            }
        }

        if legal_card_ids.is_empty() {
            panic!("No legal card ids!");
        }

        for card_id in &legal_card_ids {
            let mut sim_score = 0;

            for _ in 0..simulations {
                let mut sim_game = game.clone();

                // Assign random cards to all players but the active player.
                fastrand::shuffle(&mut hidden_cards);
                let mut hidden_cards_idx = 0;
                for p in 0..PLAYERS {
                    if p == monte_player {
                        continue;
                    }
                    let hand = &mut sim_game.hands[p];
                    let card_count = hand.len();

                    hand.clear();
                    for _ in 0..card_count {
                        let card = hidden_cards[hidden_cards_idx].clone();
                        hidden_cards_idx += 1;
                        hand.push(card);
                    }
                }

                sim_game.play_card_id(*card_id);

                while !sim_game.hand_completed() {
                    if sim_game.trick_completed() {
                        sim_game.award_trick();
                        sim_game.reset_for_next_trick();
                    }

                    let ids = sim_game.get_playable_card_ids();
                    if let Some(id) = fastrand::choice(ids) {
                        sim_game.play_card_id(id);
                    }
                }
                let _ = sim_game.award_nest_cards();

                sim_score += sim_game.scores[team] as i32;
                sim_score -= sim_game.scores[opp_team] as i32;
            }

            if sim_score > best_score {
                best_score = sim_score;
                best_play = card_id;
            }
        }
        sender
            .send(PlayerAction::PlayCard(*best_play))
            .expect("send error");
    }
}
