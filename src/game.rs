use crate::{
    card::{Card, CardSuit, Points, Rank, SelectState},
    trick::Trick,
};

pub enum PlayerAction {
    Bid(Bid),
    IncBid,
    DecBid,
    PlayCard(u8),
    ShouldExit,
}

/// Card ranks to include in the game.
pub const CARD_RANKS: [u8; 9] = [5, 7, 8, 9, 10, 11, 12, 13, 14];
/// The number of players in the game.
pub const PLAYERS: usize = 4;
/// Initial hand size.
pub const HAND_SIZE: usize = 9;
// Initial number of cards dealt to the table.
pub const NEST_SIZE: usize = 1;
// Number of nest card to deal face up.
pub const NEST_CARDS_UP: u8 = 0;
pub const JOKER_RANK: u8 = 15;
pub const JOKER_PTS: u16 = 0;

pub const POINTS_TO_WIN: Points = 200;

#[derive(Clone, Debug)]
pub enum Bid {
    Pass,
    Bid(u8),
}

#[derive(Clone)]
pub struct Game {
    pub bot_players: [bool; PLAYERS as usize],
    pub scores: [Points; 2],

    pub deck: Vec<Card>,
    pub nest: Vec<Card>,
    pub hands: Vec<Vec<Card>>,
    pub bids: [Option<Bid>; PLAYERS],
    pub taken: [Vec<Card>; 2], // We, They

    /// The player who is the dealer.
    pub dealer: usize,
    /// The active player.
    pub active: usize,
    nest_face_up_count: u8,
    /// The high bidder.
    pub maker: Option<usize>,
    pub high_bid: Option<Bid>,
    /// The hand's trump suit.
    pub trump_suit: Option<CardSuit>,
    /// The current trick
    pub trick: Trick,
    pub tricks_played: u8,
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
            scores: [0, 0],
            deck: Vec::new(),
            nest: Vec::new(),
            hands,
            bids: [None, None, None, None],
            taken: [Vec::new(), Vec::new()],
            dealer,
            active: dealer,
            nest_face_up_count: 0,
            maker: None,
            high_bid: None,
            trump_suit: None,
            trick: Trick::new(PLAYERS),
            tricks_played: 0,
        }
    }

    pub fn team_index(&self, player: usize) -> usize {
        player % 2
    }

    pub fn opponent_index(&self, player: usize) -> usize {
        (player + 1) % 2
    }

    pub fn bot_active(&self) -> bool {
        self.bot_players[self.active]
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

    #[allow(dead_code)]
    pub fn print_table(&self) {
        print!("table: ");
        for card in &self.nest {
            print!("{} ", card.to_string());
        }
        println!("");
    }

    #[allow(dead_code)]
    pub fn print_captures(&self) {
        for team in 0..2 {
            print!("captured: {}: ", team);
            for card in &self.taken[team] {
                print!("{} ", card.to_string());
            }
            println!("");
        }
    }

    fn create_card(&mut self, id: u8, suit: CardSuit, rank: Rank, points: Points) {
        let mut card = Card::new(id, suit, rank, points);
        card.face_up = false;
        self.deck.push(card);
    }

    pub fn create_deck(&mut self) {
        const SUITS: [CardSuit; 5] = [
            CardSuit::Club,
            CardSuit::Diamond,
            CardSuit::Heart,
            CardSuit::Spade,
            CardSuit::Joker,
        ];
        let mut id = 0;
        for suit in &SUITS {
            if *suit == CardSuit::Joker {
                self.create_card(id, *suit, JOKER_RANK, JOKER_PTS);
                id += 1;
            } else {
                for rank in CARD_RANKS {
                    let points = match rank {
                        5 => 5,
                        10 => 10,
                        14 => 10,
                        _ => 0,
                    };

                    self.create_card(id, *suit, rank, points);
                    id += 1;
                }
            }
        }
        println!("cards created: {}", self.deck.len());
    }

    fn next_player(&mut self) {
        self.active = (self.active + 1) % PLAYERS;
    }

    pub fn reset_for_new_hand(&mut self) {
        // If a game is over, all the cards are now in "taken."
        for team in 0..2 {
            if self.taken[team].len() > 0 {
                self.deck.append(&mut self.taken[team]);
            }
        }
        for card in &mut self.deck {
            card.face_up = false;
        }
        fastrand::shuffle(&mut self.deck);

        self.dealer = (self.dealer + 1) % PLAYERS;
        self.active = self.dealer;
        self.nest_face_up_count = 0;
        self.maker = None;
        self.high_bid = None;
        self.bids.fill(None);
        self.tricks_played = 0;
    }

    pub fn deal_card_to_hand(&mut self) {
        let mut card = self.deck.pop().unwrap();
        card.face_up = !self.bot_players[self.active];
        self.hands[self.active].push(card);
        if !self.bot_active() {
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

    pub fn make_bid(&mut self, bid: Bid) {
        match bid {
            Bid::Bid(_) => {
                self.high_bid = Some(bid.clone());
                self.maker = Some(self.active);
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
                    Bid::Bid(_) => break,
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
                    Bid::Bid(_) => bids += 1,
                },
                None => return false,
            }
        }
        bids == 1 && (bids + passes) == PLAYERS
    }

    pub fn move_nest_cards_to_bid_winner(&mut self) {
        let maker = self.maker.unwrap();
        self.hands[maker].append(&mut self.nest);
        self.sort_hand(maker);
    }

    pub fn discard_to_nest(&mut self, ids: Vec<u8>) {
        let maker = self.maker.unwrap();
        for id in ids {
            let idx = self.hands[maker].iter().position(|c| c.id == id);
            let card = self.hands[maker].remove(idx.unwrap());
            self.nest.push(card);
        }
    }

    pub fn undiscard_from_nest(&mut self, id: u8) {
        let idx = self.nest.iter().position(|c| c.id == id);
        let card = self.nest.remove(idx.unwrap());
        let maker = self.maker.unwrap();
        self.hands[maker].push(card);
        self.sort_hand(maker);
    }

    pub fn mark_hand_as_out_of_scope(&mut self, player: usize) {
        for card in &mut self.hands[player] {
            card.select_state = SelectState::OutOfScope;
        }
    }

    fn has_card_in_lead_suit(&self) -> bool {
        if let Some(lead_suit) = &self.trick.lead_card_suit {
            let hand = &self.hands[self.active];
            for card in hand {
                if card.suit == *lead_suit {
                    return true;
                }
            }
        }
        false
    }

    pub fn get_playable_card_ids(&self) -> Vec<u8> {
        let mut eligible_ids = Vec::new();
        let has_card_in_lead_suit = self.has_card_in_lead_suit();

        for card in &self.hands[self.active] {
            let mut eligible = false;

            // Is this the first card to play or there are no matching cards in hand?
            if self.trick.is_empty() || !has_card_in_lead_suit {
                eligible = true;
            }
            // Not the first card in play.
            else if card.suit == self.trick.lead_card_suit.unwrap() {
                eligible = true;
            }
            if eligible {
                eligible_ids.push(card.id);
            }
        }
        eligible_ids
    }

    // pub fn set_select_state_for_cards(state: SelectState, cards: &mut [Card]) {
    //     for card in cards {
    //         card.select_state = state.clone();
    //     }
    // }

    pub fn play_card_id(&mut self, card_id: u8) {
        let mut index = 0;
        let hand = &mut self.hands[self.active];
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
        let winner = self.trick.winner.unwrap();
        let team = self.team_index(winner);
        self.scores[team] += self.trick.points;
        self.tricks_played += 1;

        for opt_card in &mut self.trick.cards {
            let card = opt_card.take().unwrap();
            self.taken[team].push(card);
        }
    }

    pub fn reset_for_next_trick(&mut self) {
        self.active = self.trick.winner.unwrap();
        self.trick.reset();
    }

    pub fn hand_completed(&self) -> bool {
        self.tricks_played == HAND_SIZE as u8
    }

    pub fn award_nest_cards(&mut self) {
        let winner = self.trick.winner.unwrap();
        let team = self.team_index(winner);
        for card in &self.nest {
            self.scores[team] += card.points;
        }
        self.taken[team].append(&mut self.nest);
    }

    pub fn complete_game(&mut self) {
        // let we_they_totals = hand_score.we_they_totals();
        // self.scores[0] += we_they_totals[0];
        // self.scores[1] += we_they_totals[1];

        // self.scores[0] += self.sweeps[0];
        // self.scores[1] += self.sweeps[1];
    }
}
