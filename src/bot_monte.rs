use std::isize;

use combination::combine;

use crate::{
    card::{Card, Id, Points, Suit},
    controller::SENDER,
    game::{Bid, Game, PlayerAction},
    game_options::BiddersWin, view::view::{ViewMessage, VIEW_MESSAGE_SENDER},
};

#[derive(Clone)]
pub struct BotMonte {}

impl BotMonte {
    pub fn new() -> Self {
        Self {}
    }

    fn best_suit(&self, cards: &[Card]) -> Suit {
        const SUITS: [Suit; 4] = [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade];
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

    #[allow(unused)]
    fn lowest_non_trump_card(&self, cards: &[Card], trump: &Option<Suit>) -> Id {
        let mut lowest_rank = 99;
        // Just in case all cards are point cards and skipped below...
        let mut lowest_id = cards.first().unwrap().id;

        for card in cards {
            // Note: this will exclude point cards.
            if !card.is_trump(trump) && card.rank < lowest_rank && card.points == 0 {
                lowest_rank = card.rank;
                lowest_id = card.id;
            }
        }
        lowest_id
    }

    #[allow(unused)]
    pub fn choose_discards_simple(&self, game: &Game, exchange_size: usize) {
        // Super basic: dump the three lowest non-trump cards.

        let trump = self.best_suit(&game.active_hand());

        // Get the eligible cards.
        let mut eligible_cards = Vec::new();
        for card in game.active_hand() {
            if card.eligible {
                eligible_cards.push(card.clone());
            }
        }
        if eligible_cards.is_empty() {
            panic!("player: {}", game.active);
        }
        let mut discards = Vec::new();

        while discards.len() < exchange_size {
            let lowest_card_id = self.lowest_non_trump_card(&eligible_cards, &Some(trump));
            discards.push(lowest_card_id);
            if let Some(idx) = eligible_cards.iter().position(|c| c.id == lowest_card_id) {
                eligible_cards.swap_remove(idx);
            }
        }
        // sender.send(PlayerAction::Discard(discards)).expect("send error");
        SENDER
            .get()
            .unwrap()
            .send(PlayerAction::Discard(discards))
            .expect("send error");
    }

    pub fn choose_discards(&self, game: &Game, exchange_size: usize) {
        // Get the eligible cards.
        let mut eligible_cards = Vec::new();
        for card in game.active_hand() {
            if card.eligible {
                eligible_cards.push(card.clone());
            }
        }
        if eligible_cards.len() < exchange_size {
            panic!("Not enough eligible cards to exchange! p:{}", game.active);
        }

        // Make a simple vec of 0,1,2,3... as indicies to the eligible cards.
        let mut indicies = Vec::new();
        for i in 0..eligible_cards.len() {
            indicies.push(i);
        }

        // Get all the combos of the indicies.
        let idx_combos = combine::from_vec_at(&indicies, exchange_size);

        let mut best_mean = isize::MIN;
        let mut best_combo = idx_combos[0].clone();

        println!("Searching {} discard combinations", idx_combos.len());

        // Remove the card associated with each combo and find the best score
        // and combo to remove.
        for (i, combo) in idx_combos.iter().enumerate() {

            // Send thinking progress message.
            let progress = i as f32 / idx_combos.len() as f32;
            VIEW_MESSAGE_SENDER
            .get()
            .unwrap()
            .send(ViewMessage::BotThinkingProgress(progress))
            .expect("send error");

            let mut sim_game = game.clone();
            // Must iterate the combo indices in reverse, otherwise removing an index
            // will foul up the correctness of the others.
            for idx in combo.iter().rev() {
                sim_game.active_hand_mut().swap_remove(*idx);
            }
            let (_id, _best_score, all_scores) = self.run_simulations(&mut sim_game, 100);
            let mean: isize = all_scores.iter().sum::<isize>() / all_scores.len() as isize;
            if mean > best_mean {
                println!("new best found: {}", mean);
                best_mean = mean;
                best_combo = combo.clone();
            }
        }

        let mut discards = Vec::new();
        for idx in best_combo {
            discards.push(eligible_cards[idx].id);
        }

        // sender.send(PlayerAction::Discard(discards)).expect("send error");
        SENDER
            .get()
            .unwrap()
            .send(PlayerAction::Discard(discards))
            .expect("send error");
    }

    pub fn choose_trump(&self, game: &Game) {
        let cards = game.active_hand();
        let suit = self.best_suit(cards);
        // sender.send(PlayerAction::ChooseTrump(suit)).expect("send error");
        SENDER
            .get()
            .unwrap()
            .send(PlayerAction::ChooseTrump(suit))
            .expect("send error");
    }

    pub fn get_bid(&self, min: Points, _max: Points, game: &Game, simulations: usize) {
        let mut sim_game = game.clone();
        let cards = sim_game.active_hand();
        let suit = self.best_suit(cards);
        sim_game.maker = Some(sim_game.active);
        sim_game.high_bid = 0;
        sim_game.set_trump_suit(suit);

        sim_game.active = game.active;

        let (_id, _best_score, mut all_scores) = self.run_simulations(&mut sim_game, simulations);

        all_scores.sort_unstable();
        let low = all_scores.first().unwrap();
        let high = all_scores.last().unwrap();

        // 0.0 means choose the lowest score produced in all simulations.
        // 1.0 means choose the highest score produced in a simulations.
        let aggressiveness = 0.70;

        let index = (all_scores.len() as f64 * aggressiveness) as usize - 1;

        let bid_pts = all_scores[index];
        println!("P:{}, low:{}, high:{}, bid:{}", game.active, low, high, bid_pts);

        let mut bid = Bid::Pass;

        if bid_pts >= min {
            match game.options.bidders_win {
                BiddersWin::PointsBid => {
                    bid = Bid::Points(bid_pts);
                }
                BiddersWin::PointsTaken => {
                    bid = Bid::Points(min);
                }
            }
        }
        // sender.send(PlayerAction::Bid(bid)).expect("send error");
        SENDER.get().unwrap().send(PlayerAction::Bid(bid)).expect("send error");
    }

    pub fn get_play(&self, game: &mut Game, simulations: usize) {
        let mut sim_game = game.clone();
        let (best_play_id, _score, _all_scores) = self.run_simulations(&mut sim_game, simulations);

        // sender.send(PlayerAction::PlayCard(best_play_id)).expect("send error");
        SENDER
            .get()
            .unwrap()
            .send(PlayerAction::PlayCard(best_play_id))
            .expect("send error");
    }

    // Use a MonteCarlo simulation to pick the best card.
    pub fn run_simulations(&self, game: &mut Game, simulations: usize) -> (Id, Points, Vec<Points>) {
        

        let monte_player = game.active;
        let team = game.team_index(game.active);

        let legal_card_ids = game.get_playable_card_ids();
        if legal_card_ids.is_empty() {
            panic!("No legal card ids!");
        }

        let mut best_score = isize::MIN;
        let mut all_scores = Vec::with_capacity(simulations * legal_card_ids.len());
        let mut best_card_id = &legal_card_ids[0];

        // Create a vec with a ref to all the cards we don't know about.
        let mut hidden_cards = Vec::new();

        for p in 0..game.options.players {
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

        let start_time = web_time::Instant::now();

        for card_id in &legal_card_ids {
            let mut sim_score = 0;

            for _ in 0..simulations {
                let mut sim_game = game.clone();

                // Assign random cards to all players but the active player.
                fastrand::shuffle(&mut hidden_cards);
                let mut hidden_cards_idx = 0;
                for p in 0..game.options.players {
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
                sim_game.complete_hand();

                //let this_sim_score = sim_game.scoring.hand_final[team];

                // Manually calc score to exclude success bonus.
                let this_sim_score = sim_game.scoring.points_taken[team]
                    + sim_game.scoring.nest[team]
                    + sim_game.scoring.last_trick[team]
                    + sim_game.scoring.majority_bonus[team];

                all_scores.push(this_sim_score);
                sim_score += this_sim_score;
            }

            if sim_score > best_score {
                best_score = sim_score;
                best_card_id = card_id;
            }
        }

        let delta = web_time::Instant::now() - start_time;
        println!("sims: {}, ms: {}", simulations, delta.as_millis());

        (*best_card_id, best_score, all_scores)
    }
}
