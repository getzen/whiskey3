use std::sync::mpsc::{self, Receiver, Sender};

use crate::bot_monte::BotMonte;
use crate::card::CardSuit;
use crate::game::{Bid, Game, PlayerAction};

use crate::view::view::View;

#[derive(Clone, Debug, PartialEq)]
pub enum GameAction {
    Setup,
    ResetForNewHand,
    DealToHands,
    DealToNest,
    GetBid,
    WaitForBid,
    MakeBid(Bid),
    EndBidding,
    MoveNestToMaker,
    GetExchanges,
    WaitForExchanges,
    Exchange(u8),
    EndExchanging,
    GetTrump,
    WaitForTrump,
    SelectTrump(CardSuit),
    GetCardPlay,
    WaitForCardPlay,
    PlayCard(u8),
    AwardTrick,
    EndHand,
    AwardNest,
    PresentScore,
    Exit,
}

pub struct Controller {
    game: Game,
    view: View,

    sender: Sender<PlayerAction>,
    receiver: Receiver<PlayerAction>,

    game_action: Option<GameAction>,
    delay_before_game_action: f32,
}

impl Controller {
    pub async fn new() -> Self {
        let (sender, receiver) = mpsc::channel();

        Self {
            game: Game::new(),
            view: View::new(sender.clone()).await,
            sender,
            receiver,
            game_action: None,
            delay_before_game_action: 0.0,
        }
    }

    pub async fn go(&mut self) {
        let mut last_time = macroquad::time::get_time();
        self.game_action = Some(GameAction::Setup);
        self.delay_before_game_action = 1.0;

        loop {
            let time_delta = (macroquad::time::get_time() - last_time) as f32;
            last_time = macroquad::time::get_time();

            // Update view animations and such.
            self.view.update(time_delta);

            // View will check events from UI elements. May result in PlayerAction
            // message being sent.
            self.view.check_events();

            // Check for PlayerAction message. Bots send messages to convey results.
            let received = self.receiver.try_recv();
            if received.is_ok() {
                // Don't handle actions here directly. Forward as GameActions to keep
                // all the logic in one place.
                match received.unwrap() {
                    PlayerAction::Bid(bid) => {
                        self.game_action = Some(GameAction::MakeBid(bid));
                    }
                    PlayerAction::Exchange(id) => {
                        self.game_action = Some(GameAction::Exchange(id));
                    }
                    PlayerAction::DoneExchanging => {
                        self.game_action = Some(GameAction::EndExchanging);
                    }
                    PlayerAction::PlayCard(card_id) => {
                        self.game_action = Some(GameAction::PlayCard(card_id));
                    }
                    PlayerAction::ShouldExit => todo!(),
                    // _ => {} // Remaining actions were handled directly by view.
                }
            }

            if self.delay_before_game_action > 0.0 {
                self.delay_before_game_action =
                    (self.delay_before_game_action - time_delta).max(0.0);
            } else {
                /*
                view.update_info(&mut self.game, state);
                view.update_deck(&self.game);
                view.update_nest(&self.game);
                view.update_bids(&self.game);
                view.update_taken(&self.game);
                view.update_hand(&self.game, *player);

                view.get_human_bid(&mut self.game);
                view.end_human_bid(&self.game);
                view.get_human_play(&mut self.game);
                */

                // Here is where the sausage is made.
                if let Some(action) = &self.game_action {
                    println!("{:?}", action);
                    match action {
                        GameAction::Setup => {
                            self.game.create_deck();

                            for card in &self.game.deck {
                                self.view.create_card_view(card).await;
                            }
                            self.view.update_deck(&self.game);
                            self.view.update_message("Welcome to Whiskey");

                            self.game_action = Some(GameAction::ResetForNewHand);
                        }
                        GameAction::ResetForNewHand => {
                            self.game.reset_for_new_hand();
                            self.view.update_info(&self.game);
                            self.view.update_deck(&self.game);
                            self.view.update_message("");
                            self.game_action = Some(GameAction::DealToHands);
                        }
                        GameAction::DealToHands => {
                            if self.game.hand_cards_to_deal > 0 {
                                self.game.hand_cards_to_deal -= 1;
                                let player = self.game.active;
                                self.game.deal_card_to_hand();
                                self.view.update_hand(&self.game, player);
                                self.delay_before_game_action = 0.1;
                            } else {
                                self.game_action = Some(GameAction::DealToNest);
                            }
                        }
                        GameAction::DealToNest => {
                            if self.game.nest_cards_to_deal > 0 {
                                self.game.nest_cards_to_deal -= 1;
                                self.game.deal_card_to_nest();
                                self.view.update_nest(&self.game, false);
                                self.delay_before_game_action = 0.5;
                            } else {
                                self.game_action = Some(GameAction::GetBid);
                            }
                        }
                        GameAction::GetBid => {
                            self.view.update_info(&self.game);
                            self.view.update_bids(&self.game);

                            if self.game.bot_is_active() {
                                self.spawn_bid_bot();
                            } else {
                                self.view.update_message("Your bid");
                                self.view.get_human_bid(&mut self.game);
                            }
                            self.delay_before_game_action = 1.0;
                            self.game_action = Some(GameAction::WaitForBid);
                        }
                        GameAction::WaitForBid => self.game_action = None,
                        GameAction::MakeBid(bid) => {
                            println!("MakeBid!");
                            self.game.make_bid(bid.clone());

                            self.view.update_info(&self.game);
                            self.view.update_message("");
                            self.view.update_bids(&self.game);
                            self.view.end_human_bid(&self.game);

                            if self.game.bidding_completed() {
                                self.delay_before_game_action = 2.0;
                                self.game_action = Some(GameAction::EndBidding);
                            } else {
                                self.game_action = Some(GameAction::GetBid);
                            }
                        }
                        GameAction::EndBidding => {
                            self.view.hide_bids_except_maker(&self.game);
                            self.game_action = Some(GameAction::MoveNestToMaker);
                        }
                        GameAction::MoveNestToMaker => {
                            self.game.move_nest_cards_to_maker();
                            let maker = self.game.maker.unwrap();
                            self.view.update_hand(&self.game, maker);
                            self.view.set_discardable_hand_cards(&self.game);
                            self.game_action = Some(GameAction::GetExchanges);
                        }
                        GameAction::GetExchanges => {
                            self.view.update_message("Discard 3 cards.");
                            self.view.show_done_exchanging_button(false);
                            self.game_action = Some(GameAction::WaitForExchanges);
                        }
                        GameAction::WaitForExchanges => self.game_action = None,
                        GameAction::Exchange(id) => {
                            self.game.exchange_with_nest(*id);

                            // Disable Done button if nest is full.
                            self.view
                                .show_done_exchanging_button(self.game.nest_is_full());

                            let maker = self.game.maker.unwrap();
                            self.view.update_hand(&self.game, maker);
                            self.view.update_nest(&self.game, false);

                            self.game_action = Some(GameAction::WaitForExchanges);
                        }
                        GameAction::EndExchanging => {
                            let maker = self.game.maker.unwrap();
                            self.game.turn_nest_cards(false);
                            
                            self.view.reset_eligibility(&self.game.hands[maker]);
                            self.view.reset_eligibility(&self.game.nest);
                            self.view.hide_done_exchanging_button();
                            self.view.update_nest(&self.game, true); // true == aside

                            self.game_action = Some(GameAction::GetTrump);
                        }
                        GameAction::GetTrump => {
                            self.game_action = None;
                        }
                        GameAction::WaitForTrump => todo!(),
                        GameAction::SelectTrump(_) => todo!(),
                        GameAction::GetCardPlay => todo!(),
                        GameAction::WaitForCardPlay => todo!(),
                        GameAction::PlayCard(card_id) => {
                            self.game.play_card_id(*card_id);

                            // view ...

                            if self.game.trick_completed() {
                                self.delay_before_game_action = 1.0;
                                self.game_action = Some(GameAction::AwardTrick);
                            } else {
                                self.game_action = Some(GameAction::GetCardPlay);
                            }
                        }
                        GameAction::AwardTrick => todo!(),
                        GameAction::EndHand => todo!(),
                        GameAction::AwardNest => todo!(),
                        GameAction::PresentScore => todo!(),
                        GameAction::Exit => todo!(),
                    }
                }
            }

            self.view.draw().await;
        } // end of loop
    }

    fn spawn_bid_bot(&self) {
        let game_clone = self.game.clone();
        let sender = self.sender.clone();
        let min_bid = self.game.min_bid();
        let max_bid = self.game.max_bid();

        if cfg!(target_family = "wasm") {
            let bot = BotMonte::new();
            bot.get_bid(&game_clone, min_bid, max_bid, sender);
        } else {
            std::thread::spawn(move || {
                let bot = BotMonte::new();
                bot.get_bid(&game_clone, min_bid, max_bid, sender);
            });
        }
    }

    fn spawn_play_bot(&self) {
        let game_clone = self.game.clone();
        let sender = self.sender.clone();

        if cfg!(target_family = "wasm") {
            let bot = BotMonte::new();
            bot.best_card_play(&game_clone, 500, sender);
        } else {
            std::thread::spawn(move || {
                let bot = BotMonte::new();
                bot.best_card_play(&game_clone, 500, sender);
            });
        }
    }

    // Return true to exit app.
    // fn check_player_actions(&mut self) -> bool {
    //     let received = self.receiver.try_recv();
    //     if received.is_ok() {
    //         match received.unwrap() {
    //             PlayerAction::Bid(bid) => todo!(),
    //             PlayerAction::IncBid => todo!(),
    //             PlayerAction::DecBid => todo!(),
    //             PlayerAction::Discard(id) => GameAction::Discard(id),
    //             PlayerAction::PlayCard(_) => todo!(),
    //             PlayerAction::ShouldExit => todo!(),
    //         }
    //     }
    //     return false;
    // }
}
