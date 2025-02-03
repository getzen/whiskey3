use std::cmp::Ordering;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
    Joker,
}

pub type Id = u8;
pub type Rank = u8;
pub type Points = usize;

#[derive(Clone, Debug)]
pub struct Card {
    pub id: Id,
    pub suit: Suit,
    pub rank: Rank,
    // Used to remember a card is a joker when its suit is changed to trump.
    pub is_joker: bool,
    pub points: Points,
    pub face_up: bool,
    pub eligible: bool, // for discarding or playing to a trick
}

impl Card {
    pub fn new(id: Id, suit: Suit, rank: Rank, points: Points) -> Self {
        let is_joker = matches!(suit, Suit::Joker);
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

    pub fn is_trump(&self, trump_suit: &Option<Suit>) -> bool {
        match trump_suit {
            Some(suit) => self.suit == *suit,
            None => false,
        }
    }

    /// Used by PartialOrd to determine sort order.
    pub fn sort_order(&self) -> u8 {
        self.rank
            + match self.suit {
                Suit::Club => 0,
                Suit::Diamond => 20,
                Suit::Heart => 40,
                Suit::Spade => 60,
                Suit::Joker => 80,
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
            Suit::Club => format!("clb{}", rank),
            Suit::Diamond => format!("dia{}", rank),
            Suit::Heart => format!("hrt{}", rank),
            Suit::Spade => format!("spd{}", rank),
            Suit::Joker => "joker".to_string(),
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
            Suit::Spade => write!(f, "{rank}♤"),
            Suit::Club => write!(f, "{rank}♧"),
            Suit::Diamond => write!(f, "{rank}♦️"),
            Suit::Heart => write!(f, "{rank}♥️"),
            Suit::Joker => write!(f, "{rank},Jkr"),
        }
    }
}
