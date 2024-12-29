use std::collections::VecDeque;

use crate::game::{Bid, Game, HAND_SIZE, NEST_SIZE, PLAYERS};

#[derive(Debug)]
pub enum State {
    Init,
    Dealing,
    DealingToNest,
    PreparingForNewTurn,
    GettingBid,
    WaitingForBid,
    MakingBid(Bid),
    GettingPlay,
    WaitingForPlay,
    MakingPlay(u8),
    CompletingTrick,
    CompletingGame(u8, u8),
    Delaying { time: f32 },
}

pub enum ViewUpdate {
    Info(State),
    Deck,
    Nest,
    Taken,
    Hand(usize),
}

pub enum ControllerMsg {
    GetPlay,
}

pub struct StateMgr {
    pub state: State,
    pub state_queue: VecDeque<State>,
    pub view_queue: VecDeque<ViewUpdate>,
    pub controller_msg: Option<ControllerMsg>,
}

impl StateMgr {
    pub fn new() -> Self {
        Self {
            state: State::Init,
            state_queue: [State::Init].into(),
            view_queue: VecDeque::new(),
            controller_msg: None,
        }
    }

    pub fn next_state(&mut self, game: &mut Game) {
        if let Some(next_state) = self.state_queue.pop_front() {
            self.on_state_exited(game);
            self.state = next_state;
            self.on_state_entered(game);
        }
    }

    pub fn on_state_entered(&mut self, game: &mut Game) {
        match &self.state {
            State::Init => {
                game.reset_for_new_hand();
                self.view_queue
                    .extend([ViewUpdate::Info(State::Init), ViewUpdate::Deck]);

                let mut deal_actions = Vec::new();
                for _ in 0..HAND_SIZE * PLAYERS {
                    deal_actions.push(State::Dealing);
                    deal_actions.push(State::Delaying { time: 0.2 });
                }
                deal_actions.push(State::DealingToNest);
                self.state_queue.extend(deal_actions);
            }

            State::Dealing => {
                let p = game.active;
                //let is_bot = game.bot_active();
                game.deal_card_to_hand();
                self.view_queue.extend([ViewUpdate::Hand(p)]);
            }

            State::DealingToNest => {
                for _ in 0..NEST_SIZE {
                    game.deal_card_to_nest();
                }
                self.view_queue.extend([ViewUpdate::Nest]);
                self.state_queue.push_back(State::GettingBid);
            }

            State::GettingBid => {
                println!("GettingBid");
            }

            State::WaitingForBid => {}

            State::MakingBid(bid) => {}

            State::PreparingForNewTurn => {
                self.view_queue.extend([ViewUpdate::Taken]);
                self.state_queue.push_back(State::GettingPlay);
            }

            State::GettingPlay => {
                self.view_queue
                    .extend([ViewUpdate::Info(State::GettingPlay)]);
                self.controller_msg = Some(ControllerMsg::GetPlay);
                self.state_queue
                    .extend([State::Delaying { time: 2.0 }, State::WaitingForPlay]);
            }

            State::WaitingForPlay => {}

            State::MakingPlay(card_id) => {
                let player = game.active;
                game.play_card_id(*card_id);
                self.view_queue.extend([
                    ViewUpdate::Info(State::MakingPlay(*card_id)),
                    ViewUpdate::Hand(player),
                    ViewUpdate::Nest,
                    ViewUpdate::Taken,
                ]);

                self.state_queue.extend([State::CompletingTrick]);
            }

            State::CompletingTrick => {
                if game.trick_completed() {
                    if game.hand_completed() {
                        game.complete_game();
                        let we = 0;
                        let they = 0;
                        self.state_queue.extend([State::CompletingGame(we, they)]);
                    } else {
                        game.reset_for_next_trick();
                        self.state_queue.push_back(State::Delaying { time: 1.0 });
                        self.state_queue.push_back(State::PreparingForNewTurn);
                    }
                } else {
                    self.state_queue.push_back(State::PreparingForNewTurn);
                }
            }

            State::CompletingGame(we, they) => {
                //println!("Game over. We: {}, They: {}", totals[0], totals[1]);
                self.view_queue
                    .extend([ViewUpdate::Info(State::CompletingGame(*we, *they))]);
                // self.state_queue
                //     .extend([State::Delaying { time: 3.0 }, State::Init]);
            }

            _ => {}
        }
    }

    pub fn process(&mut self, time_delta: f32, game: &mut Game) {
        match &self.state {
            State::Delaying { time } => {
                if *time > 0.0 {
                    self.state = State::Delaying {
                        time: time - time_delta,
                    };
                } else {
                    self.next_state(game);
                }
            }

            _ => {
                // Default action: go to next state in queue, if any.
                self.next_state(game);
            }
        }
    }

    fn on_state_exited(&mut self, _game: &mut Game) {}
}
