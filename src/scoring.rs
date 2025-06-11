use crate::card::Points;

#[derive(Clone)]
pub struct Scoring {
    pub bid: [Points; 2],
    pub points_taken: [Points; 2],
    pub nest: [Points; 2],
    pub last_trick: [Points; 2],
    pub trick_count: [u8; 2],
    pub majority_bonus: [Points; 2],
    pub hand_subtotal: [Points; 2],
    pub bonus: [Points; 2],
    pub hand_final: [Points; 2],
    pub game: [Points; 2],
}

impl Scoring {
    pub fn new() -> Self {
        Self {
            bid: [0, 0],
            points_taken: [0, 0],
            nest: [0, 0],
            last_trick: [0, 0],
            trick_count: [0, 0],
            majority_bonus: [0, 0],
            hand_subtotal: [0, 0],

            bonus: [0, 0],
            hand_final: [0, 0],
            game: [0, 0],
        }
    }

    /// Creates a new Scoring object with self's game scores.
    pub fn new_for_next_hand(&self) -> Self {
        let mut new_scoring = Scoring::new();
        new_scoring.game[0] = self.game[0];
        new_scoring.game[1] = self.game[1];
        new_scoring
    }

    pub fn update_bids(&mut self, team: usize, points: Points) {
        for t in 0..2 {
            if t == team {
                self.bid[t] = points;
            } else {
                self.bid[t] = 0;
            }
        }
    }

    pub fn update_hand_subtotals(&mut self) {
        for t in 0..2 {
            self.hand_subtotal[t] = self.points_taken[t] + self.nest[t] + self.last_trick[t] + self.majority_bonus[t];
        }
    }

    pub fn update_game_scores(&mut self) {
        for team in 0..2 {
            self.game[team] += self.hand_final[team];
        }
    }
}
