use crate::{
    card::{Card, Id, Points, Rank, Suit},
    game_options::{
        BiddersLose, BiddersWin, DefendersLose, DefendersWin, DiscardedPointCards, FirstPlayer, GameOptions, JokerKind, MajorityTricksTie, NestAwarded
    },
    scoring::Scoring,
    trick::Trick,
};

pub const DEBUGGING: bool = false;

#[derive(Clone)]
pub enum PlayerAction {
    Bid(Bid),
    Exchange(Id),     // human
    Discard(Vec<Id>), // bot
    DoneExchanging,
    ChooseTrump(Suit),
    PlayCard(Id),
    NextHand,
    ShouldExit,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Bid {
    Pass,
    Points(Points),
}

#[derive(Clone)]
pub struct Game {
    pub options: GameOptions,
    pub bot_players: Vec<bool>,

    pub scoring: Scoring,
    pub deck: Vec<Card>,
    pub exchange: Vec<Card>,
    pub nest: Vec<Card>,
    pub hands: Vec<Vec<Card>>,
    pub bids: Vec<Option<Bid>>,
    pub taken: Vec<Vec<Card>>,

    pub dealer: usize,
    /// The active player.
    pub active: usize,
    exchange_face_up_count: usize,
    nest_face_up_count: usize,

    /// The high bidder.
    pub maker: Option<usize>,
    pub high_bid: Points,

    /// The hand's trump suit.
    pub trump_suit: Option<Suit>,
    /// The current trick
    pub trick: Trick,
    pub last_trick_winner: usize,

    pub hand_cards_to_deal: usize,
    pub exchange_cards_to_deal: usize,
    pub nest_cards_to_deal: usize,
}

impl Game {
    pub fn new() -> Self {
        // Write over the defaults, if needed.
        //let options = GameOptions::whiskey_4();
        let options = GameOptions::whiskey_4_plus();
        //let options = GameOptions::dixie();
        // let options = GameOptions::whiskey_3_7();
        // let options = GameOptions::kentucky_discard();
        //let options = GameOptions::two_hundred();

        options.write_to_yaml("default.txt");

        // Read as normal.
        let options = GameOptions::read_from_yaml("default.txt");
        let players = options.players;

        let mut bot_players = Vec::new();
        let mut hands = Vec::new();
        let mut bids = Vec::new();
        let mut taken = Vec::new();

        for p in 0..players {
            if p == 0 {
                bot_players.push(false);
            } else {
                bot_players.push(true);
            }
            bids.push(None);
            hands.push(Vec::new());
            taken.push(Vec::new());
        }

        let dealer = fastrand::usize(0..players);

        Self {
            options,
            bot_players,
            scoring: Scoring::new(),
            deck: Vec::new(),
            exchange: Vec::new(),
            nest: Vec::new(),
            hands,
            bids,
            taken,
            dealer,
            active: dealer,
            exchange_face_up_count: 0,
            nest_face_up_count: 0,
            maker: None,
            high_bid: 0,
            trump_suit: None,
            trick: Trick::new(players),
            last_trick_winner: 0,

            hand_cards_to_deal: 0,
            exchange_cards_to_deal: 0,
            nest_cards_to_deal: 0,
        }
    }

    pub fn team_index(&self, player: usize) -> usize {
        player % 2
    }

    pub fn opponent_index(&self, player: usize) -> usize {
        (player + 1) % 2
    }

    pub fn bot_is_active(&self) -> bool {
        self.bot_players[self.active]
    }

    pub fn active_hand(&self) -> &[Card] {
        &self.hands[self.active]
    }

    pub fn active_hand_mut(&mut self) -> &mut Vec<Card> {
        &mut self.hands[self.active]
    }

    #[allow(dead_code)]
    pub fn print_hands(&self) {
        for (p, hand) in self.hands.iter().enumerate() {
            print!("{p}: ");
            for card in hand {
                print!("{} ", card);
            }
            println!();
        }
    }

    fn create_card(&mut self, id: Id, suit: Suit, rank: Rank, points: Points) {
        let mut card = Card::new(id, suit, rank, points);
        card.face_up = false;
        self.deck.push(card);
    }

    pub fn create_deck(&mut self) {
        let mut id = 0;
        let mut hand_points = 0;
        let cards_in_deck = self.options.cards_in_deck.clone();
        for (suit, rank, points) in cards_in_deck {
            self.create_card(id, suit, rank, points);
            id += 1;
            hand_points += points;
        }

        // Hand points hould equal max bid.
        hand_points += self.options.last_trick_pts;
        hand_points += self.options.majority_of_tricks_pts;
        println!(
            "cards created: {}, max_bid: {}, pts_found: {}",
            self.deck.len(),
            self.options.max_bid,
            hand_points
        );
    }

    fn next_player(&mut self) {
        self.active = (self.active + 1) % self.options.players;
    }

    pub fn reset_for_new_hand(&mut self) {
        self.scoring = self.scoring.new_for_next_hand();

        // Gather all the cards
        for p in 0..self.options.players {
            if !self.taken[p].is_empty() {
                self.deck.append(&mut self.taken[p]);
            }
            self.bids[p] = None;
        }
        self.deck.append(&mut self.nest);

        for card in &mut self.deck {
            card.face_up = false;
        }
        fastrand::shuffle(&mut self.deck);

        self.dealer = (self.dealer + 1) % self.options.players;
        self.active = self.dealer;
        self.exchange_face_up_count = 0;
        self.nest_face_up_count = 0;
        self.maker = None;
        self.high_bid = 0;
        self.trick.reset();
        self.set_joker_suit(Suit::Joker);

        self.hand_cards_to_deal = self.options.hand_size * self.options.players;
        self.exchange_cards_to_deal = self.options.exchange_size;
        self.nest_cards_to_deal = self.options.nest_size;
    }

    fn set_joker_suit(&mut self, suit: Suit) {
        for p in 0..self.options.players {
            for card in &mut self.hands[p] {
                if card.is_joker {
                    card.suit = suit;
                }
            }
        }
        for card in &mut self.exchange {
            if card.is_joker {
                card.suit = suit;
            }
        }
        for card in &mut self.nest {
            if card.is_joker {
                card.suit = suit;
            }
        }
        for card in &mut self.deck {
            if card.is_joker {
                card.suit = suit;
            }
        }
    }

    pub fn deal_card_to_hand(&mut self) {
        // normal
        let mut card = self.deck.pop().unwrap();

        card.face_up = !self.bot_players[self.active] || DEBUGGING;
        self.active_hand_mut().push(card);

        if !self.bot_is_active() {
            self.sort_hand(self.active);
        }
        self.next_player();
    }

    pub fn deal_card_to_exchange(&mut self) {
        let mut card = self.deck.pop().unwrap();
        if self.exchange_face_up_count < self.options.exchange_face_up {
            card.face_up = true;
            self.exchange_face_up_count += 1;
        }
        self.exchange.push(card);
    }

    pub fn deal_card_to_nest(&mut self) {
        let mut card = self.deck.pop().unwrap();
        if self.nest_face_up_count < self.options.nest_face_up {
            card.face_up = true;
            self.nest_face_up_count += 1;
        }
        self.nest.push(card);
    }

    pub fn set_active_player_after_deal(&mut self) {
        self.active = self.dealer;
        self.next_player();
    }

    pub fn sort_hand(&mut self, p: usize) {
        self.hands[p].sort();
    }

    pub fn min_current_bid(&self) -> Points {
        if self.high_bid == 0 {
            self.options.min_bid
        } else {
            self.high_bid + self.options.bid_increment
        }
    }

    pub fn make_bid(&mut self, bid: Bid) {
        if let Bid::Points(p) = bid {
            self.high_bid = p;
            self.maker = Some(self.active);

            // If pass-then-bid is allowed.
            if self.options.bid_after_passing {
                for p in 0..self.bids.len()  {
                    if p == self.active {
                        continue;
                    }
                    self.bids[p] = None;
                }
            }

            self.scoring.update_bids(self.team_index(self.active), p);
        }
        self.bids[self.active] = Some(bid);
    }

    pub fn next_bidding_player(&mut self) {
        loop {
            self.next_player();
            if self.options.bid_after_passing {
                break;
            }

            match &self.bids[self.active] {
                None => break,
                Some(bid) => match bid {
                    Bid::Pass => {
                        continue;
                    }
                    Bid::Points(_) => break,
                },
            }
        }
    }

    pub fn bidding_completed(&self) -> bool {
        let mut passes = 0;
        let mut bids = 0;
        for bid in &self.bids {
            match bid {
                Some(bid) => match bid {
                    Bid::Pass => passes += 1,
                    Bid::Points(_) => bids += 1,
                },
                None => return false,
            }
        }
        bids == 1 && (bids + passes) == self.options.players
    }

    pub fn end_bidding(&mut self) {
        self.active = self.maker.unwrap();
    }

    pub fn move_exchange_cards_to_maker(&mut self) {
        let maker = self.maker.unwrap();
        let is_human = !self.bot_players[maker];
        while !self.exchange.is_empty() {
            if let Some(mut card) = self.exchange.pop() {
                card.face_up = is_human || DEBUGGING;
                self.hands[maker].push(card);
            }
        }
        self.sort_hand(maker);
    }

    pub fn mark_eligible_discards(&mut self) {
        let maker = self.maker.unwrap();
        match self.options.discard_point_cards {
            DiscardedPointCards::Allowed(_) => {
                for card in &mut self.hands[maker] {
                    card.eligible = true;
                }
            }
            DiscardedPointCards::OnlyWhenForced(_) => {
                let mut eligible_count = 0;
                // Mark point cards as ineligible.
                for card in &mut self.hands[maker] {
                    if card.points == 0 {
                        card.eligible = true;
                        eligible_count += 1;
                    } else {
                        card.eligible = false;
                    }
                }
                // Do we have enough cards to discard?
                if eligible_count >= self.options.exchange_size {
                    return; // yes
                }
                // Mark 5-point cards as eligible.
                for card in &mut self.hands[maker] {
                    if card.points == 5 {
                        card.eligible = true;
                        eligible_count += 1;
                    }
                }
                // We good now?
                if eligible_count >= self.options.exchange_size {
                    return; // yes
                }
                // Mark 10-point cards as eligible.
                for card in &mut self.hands[maker] {
                    if card.points == 10 {
                        card.eligible = true;
                        eligible_count += 1;
                    }
                }
                // Surely we are good now.
                if eligible_count >= self.options.exchange_size {
                    return; // yes
                } else {
                    panic!();
                }
            }
        }
    }

    pub fn swap_with_exchange(&mut self, id: Id) {
        let maker = self.maker.unwrap();

        // Check if hand card.
        if let Some(idx) = self.hands[maker].iter().position(|c| c.id == id) {
            if !self.exchange_is_full() {
                let card = self.hands[maker].remove(idx);
                self.exchange.push(card);
            }
        }
        // Check if exchange card.
        else if let Some(idx) = self.exchange.iter().position(|c| c.id == id) {
            let card = self.exchange.remove(idx);
            self.hands[maker].push(card);
            self.sort_hand(maker);
        }
    }

    pub fn exchange_is_full(&self) -> bool {
        self.exchange.len() == self.options.exchange_size
    }

    pub fn turn_exchange_cards(&mut self) {
        for card in &mut self.exchange {
            card.face_up = false; // default
            match self.options.discard_point_cards {
                DiscardedPointCards::Allowed(face_up) => {
                    if face_up && card.points > 0 {
                        card.face_up = true;
                    }
                }
                DiscardedPointCards::OnlyWhenForced(face_up) => {
                    if face_up && card.points > 0 {
                        card.face_up = true;
                    }
                }
            }
        }
    }

    pub fn add_exchange_cards_to_nest(&mut self) {
        while !self.exchange.is_empty() {
            self.nest.push(self.exchange.pop().unwrap());
        }
        self.nest.sort_by(|a, b| a.face_up.cmp(&b.face_up));
    }

    // pub fn turn_nest_cards(&mut self, face_up: bool) {
    //     for card in &mut self.nest {
    //         card.face_up = face_up || DEBUGGING;
    //     }
    // }

    pub fn set_trump_suit(&mut self, suit: Suit) {
        self.trump_suit = Some(suit);

        if self.options.joker_kind == JokerKind::Trump {
            self.set_joker_suit(suit);
        }
        
        self.sort_hand(0);
    }

    pub fn set_first_player(&mut self) {
        // Who starts the first trick?
        match self.options.first_player {
            FirstPlayer::Bidder => {
                self.active = self.maker.unwrap();
            }
            FirstPlayer::LeftOfBidder => {
                self.active = self.maker.unwrap();
                self.next_player();
            }
            FirstPlayer::LeftOfDealer => {
                self.active = self.dealer;
                self.next_player();
            }
        }
    }

    fn player_has_card_in_lead_suit(&self) -> bool {
        if let Some(lead_suit) = &self.trick.lead_suit {
            let hand = self.active_hand();
            for card in hand {
                if card.suit == *lead_suit {
                    return true;
                }
            }
        }
        false
    }

    pub fn get_playable_card_ids(&mut self) -> Vec<Id> {
        let mut eligible_ids = Vec::new();

        // There is some code duplication here, but it's in the interest of speed.

        if self.trick.lead_suit.is_none() || !self.player_has_card_in_lead_suit() {
            for card in self.active_hand_mut() {
                card.eligible = true;
                eligible_ids.push(card.id);
            }
            return eligible_ids;
        }

        let lead_suit = self.trick.lead_suit.unwrap();

        for card in self.active_hand_mut() {
            if card.suit == lead_suit {
                 card.eligible = true;
                 eligible_ids.push(card.id);
            } else {
                card.eligible = false;
            }
        }
        eligible_ids
    }

    pub fn play_card_id(&mut self, card_id: Id) {
        let mut index = 0;
        let hand = self.active_hand_mut();
        for (idx, card) in hand.iter().enumerate() {
            if card.id == card_id {
                index = idx;
                break;
            }
        }
        let mut card = hand.remove(index);
        card.face_up = true;
        self.trick.add(self.active, card, &self.trump_suit, self.options.joker_kind);

        self.next_player();
    }

    pub fn trick_completed(&self) -> bool {
        self.trick.completed()
    }

    // pub fn tricks_played(&self) -> u8 {
    //     self.scoring.trick_count[0] + self.scoring.trick_count[1]
    // }

    pub fn award_trick(&mut self) {
        self.last_trick_winner = self.trick.winner.unwrap();

        let team = self.team_index(self.last_trick_winner);
        self.scoring.points_taken[team] += self.trick.points;
        self.scoring.trick_count[team] += 1;
        self.scoring.update_hand_subtotals();

        for opt_card in &mut self.trick.cards {
            let mut card = opt_card.take().unwrap();
            card.face_up = false;
            self.taken[self.last_trick_winner].push(card);
        }
    }

    pub fn reset_for_next_trick(&mut self) {
        self.active = self.trick.winner.unwrap();
        self.trick.reset();
    }

    pub fn hand_completed(&self) -> bool {
        self.active_hand().is_empty()
    }

    pub fn award_nest_cards(&mut self) -> Points {
        let mut points = 0;

        for card in &mut self.nest {
            card.face_up = true;
            points += card.points;
        }
        // Who gets the nest points?
        match self.options.nest_awarded {
            NestAwarded::ToLastTrickWinner => {
                let winner = self.trick.winner.unwrap();
                let team = self.team_index(winner);
                self.scoring.nest[team] = points;
            }
            NestAwarded::ToDefenders => {
                let opp = self.opponent_index(self.maker.unwrap());
                self.scoring.nest[opp] = points;
            }
        }
        self.scoring.update_hand_subtotals();
        points
    }

    pub fn complete_hand(&mut self) {
        let maker = self.maker.unwrap();
        let maker_team = self.team_index(maker);
        let defen_team = self.opponent_index(maker);

        // Award last trick bonus -- EXPERIMENT, SEE BELOW.
        let last_trick_team = self.team_index(self.last_trick_winner);
        // self.scoring.last_trick[last_trick_team] = self.options.last_trick_pts;
        // Award last trick bonus to NEST.
        self.scoring.nest[last_trick_team] += self.options.last_trick_pts;

        // Award points for taking the majority of tricks
        if self.scoring.trick_count[maker_team] > self.scoring.trick_count[defen_team] {
            self.scoring.majority_bonus[maker_team] = self.options.majority_of_tricks_pts;
        } else if self.scoring.trick_count[defen_team] > self.scoring.trick_count[maker_team] {
            self.scoring.majority_bonus[defen_team] = self.options.majority_of_tricks_pts;
        } else {
            // It's a tie
            match self.options.majority_tricks_tie {
                MajorityTricksTie::ToDefenders => {
                    self.scoring.majority_bonus[defen_team] = self.options.majority_of_tricks_pts;
                }
                MajorityTricksTie::SplitBetween => {
                    self.scoring.majority_bonus[maker_team] = self.options.majority_of_tricks_pts / 2;
                    self.scoring.majority_bonus[defen_team] = self.options.majority_of_tricks_pts / 2;
                }
                MajorityTricksTie::NoPoints => {}
            }
        }

        self.scoring.update_hand_subtotals();

        let maker_subtotal = self.scoring.hand_subtotal[maker_team];

        if maker_subtotal >= self.high_bid {
            // Success by makers
            match self.options.bidders_win {
                BiddersWin::PointsBid => {
                    // Note max bid by maker is not required.
                    if self.high_bid == self.options.max_bid && maker_subtotal == self.options.max_bid {
                        self.scoring.bonus[maker_team] = self.options.slam_bonus;
                    }
                    self.scoring.hand_final[maker_team] =
                        self.scoring.bid[maker_team] + self.scoring.bonus[maker_team];
                }
                BiddersWin::PointsTaken => {
                    // Note max bid by maker is not required.
                    if maker_subtotal == self.options.max_bid {
                        self.scoring.bonus[maker_team] = self.options.slam_bonus;
                    }
                    self.scoring.hand_final[maker_team] = maker_subtotal + self.scoring.bonus[maker_team];
                }
            }
            match self.options.defenders_lose {
                DefendersLose::PointsTaken => {
                    self.scoring.hand_final[defen_team] = self.scoring.hand_subtotal[defen_team]
                }
                DefendersLose::HalfPoints => {
                    let mut rounded = self.scoring.hand_subtotal[defen_team];
                    if rounded % 5 == 0 {
                        rounded += 5;
                    }
                    self.scoring.hand_final[defen_team] = rounded / 2;
                }

                DefendersLose::Zero => self.scoring.hand_final[defen_team] = 0,
            }
        } else {
            // Defenders win
            match self.options.bidders_lose {
                BiddersLose::Zero => self.scoring.hand_final[maker_team] = 0,
                BiddersLose::MinusBid => self.scoring.hand_final[maker_team] = -self.scoring.bid[maker_team],
            }
            match self.options.defenders_win {
                DefendersWin::PointsTaken(bonus) => {
                    self.scoring.bonus[defen_team] = bonus;
                    self.scoring.hand_final[defen_team] = self.scoring.hand_subtotal[defen_team] + bonus;
                }
            }
        }
        self.scoring.update_game_scores();
    }

    // pub fn complete_game(&mut self) {
    //     // If both scores exceed the requirement, the maker's team wins.
    // }
}
