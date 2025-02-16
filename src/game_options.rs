use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};

use crate::card::{Points, Rank, Suit};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PartnerKind {
    None,
    Across,
    Called,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DiscardedPointCards {
    /// bool = face up
    Allowed(bool),
    /// bool = face up
    OnlyWhenForced(bool),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NestAwarded {
    ToLastTrickWinner,
    ToDefenders,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MajorityTricksTie {
    ToDefenders,
    SplitBetween,
    NoPoints,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FirstPlayer {
    LeftOfBidder,
    LeftOfDealer,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BiddersWin {
    /// Points = bonus for win
    PointsBid(Points),
    /// Points = bonus for win
    PointsTaken(Points),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BiddersLose {
    Zero,
    MinusBid,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DefendersWin {
    /// Points = bonus for win
    PointsTaken(Points),
    PointsTakenWithCap(Points),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DefendersLose {
    PointsTaken,
    HalfPoints,
    Zero,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameOptions {
    pub players: usize,
    // hand_size, exchange_size, and nest_size determine how many cards are dealt.
    // Any remaining cards in the deck are out of play for the hand.
    pub hand_size: usize,
    /// The number of cards dealt to the exchange. After the bidder exchanges with
    /// the cards in his hand, the exchange cards are added to the nest.
    pub exchange_size: usize,
    /// The number of exchange cards dealt face up as a teaser to the players.
    pub exchange_face_up: usize,
    /// The number of cards dealt directly to the nest and not exchanged. For traditional
    /// Rook games, this will be zero, and the exchange size serves as the effective nest
    /// size.
    pub nest_size: usize,
    /// The number of nest cards dealt face up. Does not include the exchange cards added
    /// later to the nest.
    pub nest_face_up: usize,
    pub min_bid: Points,
    pub max_bid: Points,
    pub bid_after_passing: bool,
    pub discard_point_cards: DiscardedPointCards,
    pub nest_awarded: NestAwarded,
    pub first_player: FirstPlayer,
    pub last_trick_pts: Points,
    pub majority_of_tricks_pts: Points,
    pub majority_tricks_tie: MajorityTricksTie,
    pub bidders_win: BiddersWin,
    pub bidders_lose: BiddersLose,
    pub defenders_win: DefendersWin,
    pub defenders_lose: DefendersLose,
    pub points_to_win_game: Points,

    pub cards_in_deck: Vec<(Suit, Rank, Points)>,
}

impl GameOptions {
    pub fn whiskey_4() -> Self {
        Self {
            players: 4,
            hand_size: 10,
            exchange_size: 3,
            exchange_face_up: 0,
            nest_size: 2,
            nest_face_up: 2,
            min_bid: 80,
            max_bid: 160,
            bid_after_passing: false,
            discard_point_cards: DiscardedPointCards::OnlyWhenForced(true),
            nest_awarded: NestAwarded::ToLastTrickWinner,
            first_player: FirstPlayer::LeftOfBidder,
            last_trick_pts: 0,
            majority_of_tricks_pts: 0,
            majority_tricks_tie: MajorityTricksTie::NoPoints,
            bidders_win: BiddersWin::PointsTaken(40),
            bidders_lose: BiddersLose::Zero,
            defenders_win: DefendersWin::PointsTaken(40),
            defenders_lose: DefendersLose::PointsTaken,
            points_to_win_game: 400,
            // All cards from 4 -> Ace, plus one high Joker worth 0. 45 cards.
            cards_in_deck: vec![
                (Suit::Club, 4, 0),
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
                (Suit::Spade, 4, 0),
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
                (Suit::Joker, 15, 0),
            ],
        }
    }

    /// This is my custom take of the Dixie concept.
    pub fn dixie() -> Self {
        Self {
            players: 4,
            hand_size: 7,
            exchange_size: 2,
            exchange_face_up: 0,
            nest_size: 2,
            nest_face_up: 0,
            min_bid: 60,
            max_bid: 120,
            bid_after_passing: false,
            discard_point_cards: DiscardedPointCards::OnlyWhenForced(true),
            nest_awarded: NestAwarded::ToLastTrickWinner,
            first_player: FirstPlayer::LeftOfBidder,
            last_trick_pts: 0,
            majority_of_tricks_pts: 20,
            majority_tricks_tie: MajorityTricksTie::ToDefenders,
            bidders_win: BiddersWin::PointsTaken(30),
            bidders_lose: BiddersLose::Zero,
            defenders_win: DefendersWin::PointsTaken(30),
            defenders_lose: DefendersLose::PointsTaken,
            points_to_win_game: 300,
            // All cards from 5, 9 -> Ace. 28 cards.
            cards_in_deck: vec![
                (Suit::Club, 5, 5),
                //(Suit::Club, 6, 0),
                //(Suit::Club, 7, 0),
                (Suit::Club, 8, 0),
                (Suit::Club, 9, 0),
                (Suit::Club, 10, 10),
                (Suit::Club, 11, 0),
                (Suit::Club, 12, 0),
                (Suit::Club, 13, 10),
                (Suit::Club, 14, 0),
                (Suit::Diamond, 5, 5),
                //(Suit::Diamond, 6, 0),
                //(Suit::Diamond, 7, 0),
                (Suit::Diamond, 8, 0),
                (Suit::Diamond, 9, 0),
                (Suit::Diamond, 10, 10),
                (Suit::Diamond, 11, 0),
                (Suit::Diamond, 12, 0),
                (Suit::Diamond, 13, 10),
                (Suit::Diamond, 14, 0),
                (Suit::Heart, 5, 5),
                //(Suit::Heart, 6, 0),
                //(Suit::Heart, 7, 0),
                (Suit::Heart, 8, 0),
                (Suit::Heart, 9, 0),
                (Suit::Heart, 10, 10),
                (Suit::Heart, 11, 0),
                (Suit::Heart, 12, 0),
                (Suit::Heart, 13, 10),
                (Suit::Heart, 14, 0),
                (Suit::Spade, 5, 5),
                //(Suit::Spade, 6, 0),
                //(Suit::Spade, 7, 0),
                (Suit::Spade, 8, 0),
                (Suit::Spade, 9, 0),
                (Suit::Spade, 10, 10),
                (Suit::Spade, 11, 0),
                (Suit::Spade, 12, 0),
                (Suit::Spade, 13, 10),
                (Suit::Spade, 14, 0),
                //(Suit::Joker, 15, 0),
            ],
        }
    }

    // pub fn one_high_partnership() -> Self {

    // }

    /// This version has the Joker as the low trump. Having it as the high trump
    /// and worth 20 should be outlawed.
    pub fn kentucky_discard() -> Self {
        Self {
            players: 4,
            hand_size: 9,
            exchange_size: 5,
            exchange_face_up: 0,
            nest_size: 0,
            nest_face_up: 0,
            min_bid: 60,
            max_bid: 120,
            bid_after_passing: false,
            discard_point_cards: DiscardedPointCards::Allowed(false),
            nest_awarded: NestAwarded::ToLastTrickWinner,
            first_player: FirstPlayer::LeftOfBidder,
            last_trick_pts: 0,
            majority_of_tricks_pts: 0,
            majority_tricks_tie: MajorityTricksTie::NoPoints,
            bidders_win: BiddersWin::PointsTaken(0),
            bidders_lose: BiddersLose::MinusBid,
            defenders_win: DefendersWin::PointsTaken(0),
            defenders_lose: DefendersLose::PointsTaken,
            points_to_win_game: 200,
            // All cards from 5 -> Ace, plus one low Joker worth 20. 41 cards.
            cards_in_deck: vec![
                (Suit::Club, 5, 5),
                (Suit::Club, 6, 0),
                (Suit::Club, 7, 0),
                (Suit::Club, 8, 0),
                (Suit::Club, 9, 0),
                (Suit::Club, 10, 10),
                (Suit::Club, 11, 0),
                (Suit::Club, 12, 0),
                (Suit::Club, 13, 0),
                (Suit::Club, 14, 10),
                (Suit::Diamond, 5, 5),
                (Suit::Diamond, 6, 0),
                (Suit::Diamond, 7, 0),
                (Suit::Diamond, 8, 0),
                (Suit::Diamond, 9, 0),
                (Suit::Diamond, 10, 10),
                (Suit::Diamond, 11, 0),
                (Suit::Diamond, 12, 0),
                (Suit::Diamond, 13, 0),
                (Suit::Diamond, 14, 10),
                (Suit::Heart, 5, 5),
                (Suit::Heart, 6, 0),
                (Suit::Heart, 7, 0),
                (Suit::Heart, 8, 0),
                (Suit::Heart, 9, 0),
                (Suit::Heart, 10, 10),
                (Suit::Heart, 11, 0),
                (Suit::Heart, 12, 0),
                (Suit::Heart, 13, 0),
                (Suit::Heart, 14, 10),
                (Suit::Spade, 5, 5),
                (Suit::Spade, 6, 0),
                (Suit::Spade, 7, 0),
                (Suit::Spade, 8, 0),
                (Suit::Spade, 9, 0),
                (Suit::Spade, 10, 10),
                (Suit::Spade, 11, 0),
                (Suit::Spade, 12, 0),
                (Suit::Spade, 13, 0),
                (Suit::Spade, 14, 10),
                (Suit::Joker, 1, 20),
            ],
        }
    }

    fn read_contents_from_file(path: &str) -> String {
        let mut file = File::open(&path).expect("Could not open: {path}");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Could not read to string: {path}");
        contents
    }

    pub fn read_from_yaml(path: &str) -> GameOptions {
        let contents = GameOptions::read_contents_from_file(path);

        match serde_yaml::from_str(&contents) {
            Ok(options) => options,
            Err(e) => panic!("Error creating GameOptions: {}", e),
        }
    }

    pub fn write_to_yaml(&self, path: &str) {
        let serialized = serde_yaml::to_string(self).unwrap();

        let mut file = match File::create(&path) {
            Ok(file) => file,
            Err(e) => panic!("{}", e),
        };
        write!(file, "{}", serialized).expect("File not written: {path}");
    }
}
