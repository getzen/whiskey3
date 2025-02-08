use std::sync::mpsc::Sender;

use crate::{
    card::{Card, Id, Points, Suit},
    game::{Bid, Game, PlayerAction},
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

    pub fn choose_discards(&self, game: &Game, exchange_size: usize, sender: Sender<PlayerAction>) {
        // Super basic: dump the three lowest non-trump cards.

        // Make a copy of the hand cards.
        let mut cards_copy = Vec::new();
        for card in game.active_hand() {
            cards_copy.push(card.clone());
        }
        let mut discards = Vec::new();

        let trump = self.best_suit(&cards_copy);

        while discards.len() < exchange_size {
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
        let suit = self.best_suit(cards);
        sender
            .send(PlayerAction::ChooseTrump(suit))
            .expect("send error");
    }

    pub fn get_bid(
        &self,
        min: Points,
        max: Points,
        game: &Game,
        simulations: usize,
        sender: Sender<PlayerAction>,
    ) {
        let mut sim_game = game.clone();
        let cards = sim_game.active_hand();
        let suit = self.best_suit(cards);
        sim_game.maker = Some(sim_game.active);
        sim_game.set_trump_suit(suit);

        sim_game.active = game.active;

        let (_id, _best_score, mut all_scores) = self.run_simulations(&mut sim_game, simulations);

        all_scores.sort_unstable();

        // 0.0 means choose the lowest score produced in all simulations.
        // 1.0 means choose the highest score produced in a simulations.
        let aggressiveness = 0.50;

        let index = (all_scores.len() as f64 * aggressiveness) as usize - 1;

        let bid_pts = all_scores[index];
        println!("P: {}: bid_pts: {}", game.active, bid_pts);

        let mut bid = Bid::Pass;

        if bid_pts >= min {
            // bid_pts is the max we should bid. Let's bid half-way between
            // the min and bid_pts to allow room to raise. Add a random factor?
            // let mut adj_bid = ((bid_pts + min) / 2) % 5;
            let mut adj_bid = (bid_pts + min) / 2 / 5 * 5;
            adj_bid = adj_bid.min(max);
            bid = Bid::Points(adj_bid);
        }
        sender.send(PlayerAction::Bid(bid)).expect("send error");
    }

    pub fn get_play(&self, game: &mut Game, simulations: usize, sender: Sender<PlayerAction>) {
        let mut sim_game = game.clone();
        let (best_play_id, _score, _all_scores) = self.run_simulations(&mut sim_game, simulations);

        sender
            .send(PlayerAction::PlayCard(best_play_id))
            .expect("send error");
    }

    // Use a MonteCarlo simulation to pick the best card.
    pub fn run_simulations(
        &self,
        game: &mut Game,
        simulations: usize,
    ) -> (Id, Points, Vec<Points>) {
        let monte_player = game.active;
        let team = game.team_index(game.active);
        //let opp_team = game.opponent_index(game.active);

        let legal_card_ids = game.get_playable_card_ids();
        let mut best_score = 0; //i32::MIN;
        let mut all_scores = Vec::with_capacity(simulations * legal_card_ids.len());

        let legal_card_ids = game.get_playable_card_ids();
        let mut best_card_id = &legal_card_ids[0];

        // Create a vec with all the cards we don't know about.
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

                // Manually calc score to exclude success bonus.
                let this_sim_score = sim_game.scoring.points_taken[team]
                    + sim_game.scoring.nest[team]
                    + sim_game.scoring.last_trick[team];
                all_scores.push(this_sim_score);
                sim_score += this_sim_score;
            }

            if sim_score > best_score {
                best_score = sim_score;
                best_card_id = card_id;
            }
        }
        (*best_card_id, best_score, all_scores)
    }
}
