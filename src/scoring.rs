use crate::card::Points;

#[derive(Clone)]
pub struct Scoring {
    pub bid: [Points; 2],
    pub taken: [Points; 2],
    pub nest: [Points; 2],
    pub bonus: [Points; 2],
    pub hand: [Points; 2],
    pub game: [Points; 2],
}

impl Scoring {
    pub fn new() -> Self {
        Self {
            bid: [0, 0],
            taken: [0, 0],
            nest: [0, 0],
            bonus: [0, 0],
            hand: [0, 0],
            game: [0, 0],
        }
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

    // pub fn update_hands(&mut self) {
    //     for team in 0..2 {
    //         self.hand[team] = self.bid[team] + self.nest[team] + self.bonus[team];
    //     }
    // }

    pub fn update_games(&mut self) {
        for team in 0..2 {
            self.game[team] += self.hand[team];
        }
    }
}
