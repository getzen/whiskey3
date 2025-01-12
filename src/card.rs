use std::cmp::Ordering;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum CardSuit {
    Club,
    Diamond,
    Heart,
    Spade,
    Joker,
}

pub type Rank = u8;
pub type Points = u16;

#[derive(Clone, Debug)]
pub struct Card {
    pub id: u8,
    pub suit: CardSuit,
    pub rank: Rank,
    // Used to remember a card is a joker when its suit is changed to trump.
    pub is_joker: bool,
    pub points: Points,
    pub face_up: bool,
    pub eligible: bool, // for discarding or playing to a trick
}

impl Card {
    pub fn new(id: u8, suit: CardSuit, rank: Rank, points: Points) -> Self {
        let is_joker = match suit {
            CardSuit::Joker => true,
            _ => false,
        };
        Self {
            id,
            suit,
            rank,
            is_joker,
            points,
            face_up: false,
            eligible: false,
        }
    }

    pub fn is_trump(&self, trump_suit: &Option<CardSuit>) -> bool {
        match trump_suit {
            Some(suit) => self.suit == *suit,
            None => false,
        }
    }

    /// Used by PartialOrd to determine sort order.
    pub fn sort_order(&self) -> u8 {
        self.rank
            + match self.suit {
                CardSuit::Club => 0,
                CardSuit::Diamond => 20,
                CardSuit::Heart => 40,
                CardSuit::Spade => 60,
                CardSuit::Joker => 80,
            }
    }

    fn rank_string(&self) -> String {
        match self.rank {
            11 => "J".to_string(),
            12 => "Q".to_string(),
            13 => "K".to_string(),
            14 => "A".to_string(),
            _ => self.rank.to_string(),
        }
    }

    pub fn file_string(&self) -> String {
        let rank = self.rank_string();
        match self.suit {
            CardSuit::Club => format!("clb{}", rank),
            CardSuit::Diamond => format!("dia{}", rank),
            CardSuit::Heart => format!("hrt{}", rank),
            CardSuit::Spade => format!("spd{}", rank),
            CardSuit::Joker => format!("joker"),
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.sort_order().cmp(&other.sort_order())
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Card {}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.suit == other.suit && self.rank == other.rank
    }
}

impl core::fmt::Display for Card {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let rank = self.rank_string();
        match self.suit {
            CardSuit::Spade => write!(f, "{rank}♤"),
            CardSuit::Club => write!(f, "{rank}♧"),
            CardSuit::Diamond => write!(f, "{rank}♦️"),
            CardSuit::Heart => write!(f, "{rank}♥️"),
            CardSuit::Joker => write!(f, "{rank},Jkr"),
        }
    }
}
