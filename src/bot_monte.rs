use std::sync::mpsc::Sender;

use crate::game::{Game, PlayerAction, PLAYERS};

#[derive(Clone)]
pub struct BotMonte {}

impl BotMonte {
    pub fn new() -> Self {
        Self {}
    }

    // Use a MonteCarlo simulation to pick the best card.
    pub fn best_card_play(&self, game: &Game, simulations: usize, sender: Sender<PlayerAction>) {
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
                sim_game.award_nest_cards();

                sim_score += sim_game.scores[team] as i32;
                sim_score -= sim_game.scores[opp_team] as i32;
            }

            if sim_score > best_score {
                best_score = sim_score;
                best_play = card_id;
            }
        }
        sender
            .send(PlayerAction::PlayCard(best_play.clone()))
            .expect("send error");
    }
}
