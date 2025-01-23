use crate::{
    card::{Card, Points, Rank, Suit},
    scoring::Scoring,
    trick::Trick,
};

pub const DEBUGGING: bool = true;

#[derive(Clone)]
pub enum PlayerAction {
    Bid(Bid),
    Exchange(u8),     // human
    Discard(Vec<u8>), // bot
    DoneExchanging,
    ChooseTrump(Suit),
    PlayCard(u8),
    ShouldExit,
}

pub const ALL_CARDS: [(Suit, Rank, Points); 43] = [
    //(Suit::Club, 4, 0), //
    (Suit::Club, 5, 5),
    (Suit::Club, 6, 0),
    (Suit::Club, 7, 0),
    (Suit::Club, 8, 0),
    (Suit::Club, 9, 0),
    (Suit::Club, 10, 10),
    (Suit::Club, 11, 0),
    (Suit::Club, 12, 0),
    (Suit::Club, 13, 10),
    (Suit::Club, 14, 15),
    //(Suit::Diamond, 3, 0), //
    (Suit::Diamond, 4, 0),
    (Suit::Diamond, 5, 5),
    (Suit::Diamond, 6, 0),
    (Suit::Diamond, 7, 0),
    (Suit::Diamond, 8, 0),
    (Suit::Diamond, 9, 0),
    (Suit::Diamond, 10, 10),
    (Suit::Diamond, 11, 0),
    (Suit::Diamond, 12, 0),
    (Suit::Diamond, 13, 10),
    (Suit::Diamond, 14, 15),
    //(Suit::Heart, 3, 0), //
    (Suit::Heart, 4, 0),
    (Suit::Heart, 5, 5),
    (Suit::Heart, 6, 0),
    (Suit::Heart, 7, 0),
    (Suit::Heart, 8, 0),
    (Suit::Heart, 9, 0),
    (Suit::Heart, 10, 10),
    (Suit::Heart, 11, 0),
    (Suit::Heart, 12, 0),
    (Suit::Heart, 13, 10),
    (Suit::Heart, 14, 15),
    //(Suit::Spade, 4, 0), //
    (Suit::Spade, 5, 5),
    (Suit::Spade, 6, 0),
    (Suit::Spade, 7, 0),
    (Suit::Spade, 8, 0),
    (Suit::Spade, 9, 0),
    (Suit::Spade, 10, 10),
    (Suit::Spade, 11, 0),
    (Suit::Spade, 12, 0),
    (Suit::Spade, 13, 10),
    (Suit::Spade, 14, 15),
    (Suit::Joker, 1, 20),
];

pub const MIN_BID: Points = 60;
pub const LAST_TRICK_BONUS: Points = 20;
pub const SUCCESS_BONUS: Points = 50;
pub const POINTS_TO_WIN: Points = 400;

/// The number of players in the game.
pub const PLAYERS: usize = 4;
/// Number of cards dealt to the nest. The remaining cards are dealt to the players
/// and determine the value returned by self.hand_size().
pub const NEST_SIZE: usize = 3;
/// Number of nest card to deal face up.
pub const NEST_CARDS_UP: u8 = 0;

#[derive(Clone, Debug, PartialEq)]
pub enum Bid {
    Pass,
    Points(Points),
}

#[derive(Clone)]
pub struct Game {
    pub bot_players: [bool; PLAYERS as usize],

    pub scoring: Scoring,
    //pub hand_scores: [Points; 2],
    //pub total_scores: [Points; 2],
    pub deck: Vec<Card>,
    pub nest: Vec<Card>,
    pub hands: Vec<Vec<Card>>,
    pub bids: [Option<Bid>; PLAYERS],
    pub taken: [Vec<Card>; PLAYERS],

    /// The player who is the dealer.
    pub dealer: usize,
    /// The active player.
    pub active: usize,
    nest_face_up_count: u8,

    /// The high bidder.
    pub maker: Option<usize>,
    pub high_bid: Points,

    /// The hand's trump suit.
    pub trump_suit: Option<Suit>,
    /// The current trick
    pub trick: Trick,
    pub tricks_played: u8,
    pub last_trick_winner: usize,

    pub hand_cards_to_deal: u8,
    pub nest_cards_to_deal: u8,
}

impl Game {
    pub fn new() -> Self {
        let mut hands = Vec::new();
        for _ in 0..PLAYERS {
            hands.push(Vec::new());
        }

        let dealer = fastrand::usize(0..4);

        Self {
            bot_players: [false, true, true, true],
            scoring: Scoring::new(),
            //hand_scores: [0, 0],
            //total_scores: [0, 0],
            deck: Vec::new(),
            nest: Vec::new(),
            hands,
            bids: [None, None, None, None],
            taken: [Vec::new(), Vec::new(), Vec::new(), Vec::new()],
            dealer,
            active: dealer,
            nest_face_up_count: 0,
            maker: None,
            high_bid: 0,
            trump_suit: None,
            trick: Trick::new(PLAYERS),
            tricks_played: 0,
            last_trick_winner: 0,

            hand_cards_to_deal: 0,
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
                print!("{} ", card.to_string());
            }
            println!("");
        }
    }

    pub fn hand_size(&self) -> usize {
        (ALL_CARDS.len() - NEST_SIZE) / PLAYERS
    }

    pub fn max_bid(&self) -> Points {
        let mut bid = 0;
        for (_suit, _rank, points) in &ALL_CARDS {
            bid += points;
        }
        bid
    }

    fn create_card(&mut self, id: u8, suit: Suit, rank: Rank, points: Points) {
        let mut card = Card::new(id, suit, rank, points);
        card.face_up = false;
        self.deck.push(card);
    }

    pub fn create_deck(&mut self) {
        let mut id = 0;

        for (suit, rank, points) in ALL_CARDS {
            self.create_card(id, suit, rank, points);
            id += 1;
        }
        println!("cards created: {}", self.deck.len());
    }

    fn next_player(&mut self) {
        self.active = (self.active + 1) % PLAYERS;
    }

    pub fn reset_for_new_hand(&mut self) {
        // If a game is over, all the cards are now in "taken."
        for p in 0..PLAYERS {
            if self.taken[p].len() > 0 {
                self.deck.append(&mut self.taken[p]);
            }
            self.bids[p] = None;
        }
        for card in &mut self.deck {
            card.face_up = false;
        }
        fastrand::shuffle(&mut self.deck);

        self.dealer = (self.dealer + 1) % PLAYERS;
        self.active = self.dealer;
        self.nest_face_up_count = 0;
        self.maker = None;
        self.high_bid = 0;
        self.tricks_played = 0;
        self.set_joker_suit(Suit::Joker);

        self.hand_cards_to_deal = (self.hand_size() * PLAYERS) as u8;
        self.nest_cards_to_deal = NEST_SIZE as u8;
    }

    fn set_joker_suit(&mut self, suit: Suit) {
        for p in 0..PLAYERS {
            for card in &mut self.hands[p] {
                if card.is_joker {
                    card.suit = suit;
                    return;
                }
            }
        }
        for card in &mut self.nest {
            if card.is_joker {
                card.suit = suit;
                return;
            }
        }
        for card in &mut self.deck {
            if card.is_joker {
                card.suit = suit;
                return;
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

    pub fn deal_card_to_nest(&mut self) {
        let mut card = self.deck.pop().unwrap();
        if self.nest_face_up_count < NEST_CARDS_UP {
            card.face_up = true;
            self.nest_face_up_count += 1;
        }
        self.nest.push(card);
    }

    pub fn sort_hand(&mut self, p: usize) {
        self.hands[p].sort();
    }

    pub fn min_bid(&self) -> Points {
        MIN_BID.max(self.high_bid + 5)
    }

    pub fn make_bid(&mut self, bid: Bid) {
        match bid {
            Bid::Points(p) => {
                self.high_bid = p;
                self.maker = Some(self.active);
                self.scoring.update_bids(self.team_index(self.active), p);
            }
            _ => {}
        }
        self.bids[self.active] = Some(bid);
        self.active = self.next_bidding_player();
    }

    pub fn next_bidding_player(&self) -> usize {
        let mut p = self.active;
        loop {
            p = (p + 1) % PLAYERS;
            match &self.bids[p] {
                None => break,
                Some(bid) => match bid {
                    Bid::Pass => continue,
                    Bid::Points(_) => break,
                },
            }
        }
        p
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
        bids == 1 && (bids + passes) == PLAYERS
    }

    pub fn move_nest_cards_to_maker(&mut self) {
        let maker = self.maker.unwrap();
        let is_human = !self.bot_players[maker];
        while !self.nest.is_empty() {
            if let Some(mut card) = self.nest.pop() {
                card.face_up = is_human || DEBUGGING;
                self.hands[maker].push(card);
            }
        }
        for card in &mut self.hands[maker] {
            card.eligible = true;
        }
        self.sort_hand(maker);
    }

    pub fn exchange_with_nest(&mut self, id: u8) {
        let maker = self.maker.unwrap();

        // Check if hand card.
        if let Some(idx) = self.hands[maker].iter().position(|c| c.id == id) {
            if !self.nest_is_full() {
                let card = self.hands[maker].remove(idx);
                self.nest.push(card);
            }
        }
        // Check if nest card.
        else if let Some(idx) = self.nest.iter().position(|c| c.id == id) {
            let card = self.nest.remove(idx);
            self.hands[maker].push(card);
            self.sort_hand(maker);
        }
    }

    pub fn nest_is_full(&self) -> bool {
        self.nest.len() == NEST_SIZE
    }

    pub fn turn_nest_cards(&mut self, face_up: bool) {
        for card in &mut self.nest {
            card.face_up = face_up || DEBUGGING;
        }
    }

    pub fn set_trump_suit(&mut self, suit: Suit) {
        self.trump_suit = Some(suit);
        self.set_joker_suit(suit);
        self.sort_hand(0);
        // The first trick begins with the player after the winning bidder.
        self.next_player();
    }

    fn has_card_in_lead_suit(&self) -> bool {
        if let Some(lead_card) = &self.trick.lead_card {
            let hand = self.active_hand();
            for card in hand {
                if card.suit == lead_card.suit {
                    return true;
                }
            }
        }
        false
    }

    pub fn get_playable_card_ids(&mut self) -> Vec<u8> {
        let mut eligible_ids = Vec::new();
        let has_card_in_lead_suit = self.has_card_in_lead_suit();

        let trick_is_empty = self.trick.is_empty();

        let lead_card = self.trick.lead_card.clone();

        for card in self.active_hand_mut() {
            let mut eligible = false;

            // Is this the first card to play or there are no matching cards in hand?
            if trick_is_empty || !has_card_in_lead_suit {
                eligible = true;
            }
            // Not the first card in play.
            else if card.suit == lead_card.as_ref().unwrap().suit {
                eligible = true;
            }
            if eligible {
                eligible_ids.push(card.id);
            }
            card.eligible = eligible;
        }
        eligible_ids
    }

    pub fn play_card_id(&mut self, card_id: u8) {
        let mut index = 0;
        let hand = self.active_hand_mut();
        for (idx, card) in hand.iter().enumerate() {
            if card.id == card_id {
                index = idx;
                break;
            }
        }
        let card = hand.remove(index);
        self.trick.add(self.active, card, &self.trump_suit);

        self.next_player();
    }

    pub fn trick_completed(&self) -> bool {
        self.trick.completed()
    }

    pub fn award_trick(&mut self) {
        self.last_trick_winner = self.trick.winner.unwrap();

        let team = self.team_index(self.last_trick_winner);
        self.scoring.taken[team] += self.trick.points;
        self.tricks_played += 1;

        for opt_card in &mut self.trick.cards {
            let card = opt_card.take().unwrap();
            self.taken[self.last_trick_winner].push(card);
        }
    }

    pub fn reset_for_next_trick(&mut self) {
        self.active = self.trick.winner.unwrap();
        self.trick.reset();
    }

    pub fn hand_completed(&self) -> bool {
        self.tricks_played == self.hand_size() as u8
    }

    pub fn award_nest_cards(&mut self) -> Points {
        // let winner = self.trick.winner.unwrap();
        // let team = self.team_index(winner);
        let mut points = 0;

        let team = 1 - self.team_index(self.maker.unwrap());

        for card in &mut self.nest {
            card.face_up = true;
            self.scoring.nest[team] += card.points;
            points += card.points;
        }
        points
    }

    pub fn complete_hand(&mut self) {
        let maker = self.maker.unwrap();
        let team = self.team_index(maker);
        let opp = 1 - team;

        // Last trick bonus
        let last_trick_team = self.team_index(self.last_trick_winner);
        self.scoring.last_trick[last_trick_team] = LAST_TRICK_BONUS;

        let maker_total = self.scoring.taken[team] + self.scoring.nest[team] + self.scoring.last_trick[team];
        let maker_bid_total = self.scoring.bid[team] + self.scoring.nest[team] + self.scoring.last_trick[team];
        let opp_total = self.scoring.taken[opp] + self.scoring.nest[opp] + self.scoring.last_trick[opp];

        if maker_total >= self.high_bid {
            self.scoring.bonus[team] = SUCCESS_BONUS;
            // Award bid points, not taken points.
            self.scoring.hand[team] = maker_bid_total + self.scoring.bonus[team];
            self.scoring.hand[opp] = opp_total;
        } else {
            // Defenders win.
            self.scoring.bonus[opp] = SUCCESS_BONUS;
            self.scoring.hand[opp] = opp_total + self.scoring.bonus[opp];
        }
        self.scoring.update_game_scores();
    }

    pub fn complete_game(&mut self) {
        // If both scores exceed the requirement, the maker's team wins.
    }
}
