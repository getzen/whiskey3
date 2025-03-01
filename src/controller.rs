use std::sync::mpsc::{self, Receiver, Sender};

use crate::bot_monte::BotMonte;
use crate::card::{Id, Suit};
use crate::game::{Bid, Game, PlayerAction};

use crate::view::view::View;

use std::sync::OnceLock;
pub static SENDER: OnceLock<Sender<PlayerAction>> = OnceLock::new();

#[derive(Clone, Debug, PartialEq)]
pub enum GameAction {
    Setup,
    ResetForNewHand,
    DealToHands,
    DealToExchange,
    DealToNest,
    GetBid,
    MakeBid(Bid),
    EndBidding,
    MoveNestToMaker,
    GetExchanges,
    Exchange(Id),
    Discard(Vec<Id>),
    EndExchanging,
    GetTrump,
    ChooseTrump(Suit),
    GetCardPlay,
    PlayCard(Id),
    AwardTrick,
    AwardNest,
    PresentScore,
    //Exit,
}

pub struct Controller {
    game: Game,
    view: View,

    receiver: Receiver<PlayerAction>,

    game_action: Option<GameAction>,
    delay_before_game_action: f32,
}

impl Controller {
    pub async fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        SENDER.set(sender.clone()).expect("Error setting SENDER.");

        let game = Game::new();
        let players = game.options.players;
        let mut view = View::new(sender.clone()).await;
        view.setup(players).await;

        Self {
            game,
            view,
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
            if let Ok(received) = self.receiver.try_recv() {
                // Don't handle actions here directly. Forward as GameActions to keep
                // all the logic in one place.
                match received {
                    PlayerAction::Bid(bid) => {
                        self.game_action = Some(GameAction::MakeBid(bid));
                    }
                    PlayerAction::Exchange(id) => {
                        // human
                        self.game_action = Some(GameAction::Exchange(id));
                    }
                    PlayerAction::Discard(ids) => {
                        // bot
                        self.game_action = Some(GameAction::Discard(ids));
                    }
                    PlayerAction::DoneExchanging => {
                        self.game_action = Some(GameAction::EndExchanging);
                    }
                    PlayerAction::ChooseTrump(suit) => {
                        self.game_action = Some(GameAction::ChooseTrump(suit));
                    }
                    PlayerAction::PlayCard(card_id) => {
                        self.game_action = Some(GameAction::PlayCard(card_id));
                    }
                    PlayerAction::NextHand => {
                        self.game_action = Some(GameAction::ResetForNewHand);
                    }
                    PlayerAction::ShouldExit => todo!(),
                    // _ => {} // Remaining actions were handled directly by view.
                }
            }

            if self.delay_before_game_action > 0.0 {
                self.delay_before_game_action = (self.delay_before_game_action - time_delta).max(0.0);
            } else {
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
                            self.view.update_message(&["Welcome to Whiskey"]);

                            self.game_action = Some(GameAction::ResetForNewHand);
                        }
                        GameAction::ResetForNewHand => {
                            self.game.reset_for_new_hand();
                            self.view.update_message(&[""]);
                            self.view.show_next_hand_button(false);
                            self.view.show_trump_marker(false);
                            self.view.update_info(&self.game);
                            self.view.update_deck(&self.game);
                            self.game_action = Some(GameAction::DealToHands);
                        }
                        GameAction::DealToHands => {
                            if self.game.hand_cards_to_deal > 0 {
                                self.game.hand_cards_to_deal -= 1;
                                let player = self.game.active;
                                self.game.deal_card_to_hand();
                                self.view.update_hand(&self.game, player);
                            } else {
                                self.game_action = Some(GameAction::DealToExchange);
                            }
                        }
                        GameAction::DealToExchange => {
                            if self.game.exchange_cards_to_deal > 0 {
                                self.game.exchange_cards_to_deal -= 1;
                                self.game.deal_card_to_exchange();
                                self.view.update_exchange(&self.game);
                                self.delay_before_game_action = 0.5;
                            } else {
                                self.delay_before_game_action = 0.5;
                                self.game_action = Some(GameAction::DealToNest);
                            }
                        }
                        GameAction::DealToNest => {
                            if self.game.nest_cards_to_deal > 0 {
                                self.game.nest_cards_to_deal -= 1;
                                self.game.deal_card_to_nest();
                                self.view.update_nest(&self.game, true);
                                self.delay_before_game_action = 0.5;
                            } else {
                                self.game.set_active_player_after_deal();
                                self.delay_before_game_action = 1.0;
                                self.game_action = Some(GameAction::GetBid);
                            }
                        }
                        GameAction::GetBid => {
                            self.view.update_info(&self.game);
                            self.view.update_bids(&self.game);

                            if self.game.bot_is_active() {
                                self.spawn_bid_bot();
                            } else {
                                self.view.get_human_bid(&self.game);
                            }
                            self.game_action = None;
                        }
                        GameAction::MakeBid(bid) => {
                            let is_bot = self.game.bot_is_active();
                            self.game.make_bid(bid.clone());

                            self.view.update_info(&self.game);
                            self.view.update_bids(&self.game);
                            if !is_bot {
                                self.view.end_human_bid(&self.game);
                            }

                            if self.game.bidding_completed() {
                                self.game.end_bidding();
                                self.delay_before_game_action = 2.0;
                                self.game_action = Some(GameAction::EndBidding);
                            } else {
                                self.game.next_bidding_player();
                                self.delay_before_game_action = 1.0;
                                self.game_action = Some(GameAction::GetBid);
                            }
                        }
                        GameAction::EndBidding => {
                            self.view.hide_bids_except_maker(&self.game);
                            self.game_action = Some(GameAction::MoveNestToMaker);
                        }
                        GameAction::MoveNestToMaker => {
                            self.game.move_exchange_cards_to_maker();
                            self.game.mark_eligible_discards();
                            let maker = self.game.maker.unwrap();
                            self.view.update_hand(&self.game, maker);
                            if !self.game.bot_is_active() {
                                self.view.set_discardable_hand_cards(&self.game);
                            }
                            self.view.update_info(&self.game);
                            self.game_action = Some(GameAction::GetExchanges);
                        }
                        GameAction::GetExchanges => {
                            if self.game.bot_is_active() {
                                self.view.get_bot_discards(&self.game);
                                self.spawn_discard_bot();
                            } else {
                                self.view.get_human_exchanges(&self.game);
                            }
                            self.delay_before_game_action = 1.0;
                            self.game_action = None;
                        }
                        GameAction::Exchange(id) => {
                            // human
                            self.game.swap_with_exchange(*id);

                            // Disable Done button if nest is full.
                            self.view.show_done_exchanging_button(self.game.exchange_is_full());

                            let maker = self.game.maker.unwrap();
                            self.view.update_hand(&self.game, maker);
                            self.view.update_exchange(&self.game);

                            self.game_action = None;
                        }
                        GameAction::Discard(ids) => {
                            // bot
                            for id in ids {
                                // The cards will already be face down.
                                self.game.swap_with_exchange(*id);

                                let maker = self.game.maker.unwrap();
                                self.view.update_hand(&self.game, maker);
                                self.view.update_exchange(&self.game);
                            }
                            self.delay_before_game_action = 1.5;
                            self.game_action = Some(GameAction::EndExchanging);
                        }
                        GameAction::EndExchanging => {
                            let maker = self.game.maker.unwrap();
                            self.game.turn_exchange_cards();
                            self.game.add_exchange_cards_to_nest();

                            self.view.reset_eligibility(&self.game.hands[maker]);
                            self.view.reset_eligibility(&self.game.nest);
                            self.view.hide_done_exchanging_button();
                            self.view.update_exchange(&self.game);
                            self.view.update_nest(&self.game, true); // true == aside

                            self.game_action = Some(GameAction::GetTrump);
                        }
                        GameAction::GetTrump => {
                            if self.game.bot_is_active() {
                                self.view.get_bot_trump(&self.game);
                                self.spawn_trump_bot();
                            } else {
                                self.view.show_trump_chooser(true);
                            }
                            self.delay_before_game_action = 1.0;
                            self.game_action = None;
                        }
                        GameAction::ChooseTrump(suit) => {
                            self.game.set_trump_suit(*suit);
                            self.game.set_first_player();

                            self.view.update_info(&self.game);
                            self.view.update_hand(&self.game, 0);
                            self.view.show_trump_chooser(false);
                            self.view.show_trump_marker(true);
                            self.view.set_trump_suit(Some(*suit)).await;

                            self.delay_before_game_action = 1.0;
                            self.game_action = Some(GameAction::GetCardPlay);
                        }
                        GameAction::GetCardPlay => {
                            if self.game.bot_is_active() {
                                self.view.get_bot_card_play(&self.game);
                                self.spawn_play_bot();
                            } else {
                                self.game.get_playable_card_ids();
                                self.view.get_human_card_play(&self.game);
                            }
                            self.delay_before_game_action = 1.0;
                            self.game_action = None;
                        }
                        GameAction::PlayCard(card_id) => {
                            let player = self.game.active;
                            self.game.play_card_id(*card_id);

                            self.view.reset_eligibility(&self.game.hands[player]);
                            self.view.update_hand(&self.game, player);
                            self.view.update_trick(&self.game);
                            self.view.update_message(&[""]);
                            self.view.update_info(&self.game);

                            if self.game.trick_completed() {
                                self.delay_before_game_action = 1.0;
                                self.game_action = Some(GameAction::AwardTrick);
                            } else {
                                self.game_action = Some(GameAction::GetCardPlay);
                            }
                        }
                        GameAction::AwardTrick => {
                            self.game.award_trick();

                            self.view.update_taken(&self.game);
                            self.view.update_info(&self.game);

                            if self.game.hand_completed() {
                                self.game_action = Some(GameAction::AwardNest);
                            } else {
                                self.game.reset_for_next_trick();
                                self.game_action = Some(GameAction::GetCardPlay);
                            }
                        }

                        GameAction::AwardNest => {
                            let points = self.game.award_nest_cards();

                            let message = format!("There were {} points in the nest.", points);
                            self.view.show_trump_marker(false);
                            self.view.update_message(&[&message]);
                            self.view.update_info(&self.game);
                            self.view.update_nest(&self.game, false);
                            self.view.hide_bid_marker(self.game.maker.unwrap());

                            self.delay_before_game_action = 3.0;
                            self.game_action = Some(GameAction::PresentScore);
                        }
                        GameAction::PresentScore => {
                            self.game_action = None;
                            self.game.complete_hand();
                            self.view.show_next_hand_button(true);
                            self.view.update_info(&self.game);
                        } //GameAction::Exit => todo!(),
                    }
                }
            }

            self.view.draw().await;
        } // end of loop
    }

    fn spawn_bid_bot(&self) {
        let game_clone = self.game.clone();
        let min_bid = self.game.min_current_bid();
        let max_bid = self.game.options.max_bid;

        if cfg!(target_family = "wasm") {
            let bot = BotMonte::new();
            bot.get_bid(min_bid, max_bid, &game_clone, 100);
        } else {
            std::thread::spawn(move || {
                let bot = BotMonte::new();
                bot.get_bid(min_bid, max_bid, &game_clone, 100);
            });
        }
    }

    fn spawn_discard_bot(&self) {
        let game_clone = self.game.clone();
        let exchange_size = self.game.options.exchange_size;

        if cfg!(target_family = "wasm") {
            let bot = BotMonte::new();
            bot.choose_discards(&game_clone, exchange_size);
        } else {
            std::thread::spawn(move || {
                let bot = BotMonte::new();
                bot.choose_discards(&game_clone, exchange_size);
            });
        }
    }

    fn spawn_trump_bot(&self) {
        let game_clone = self.game.clone();

        if cfg!(target_family = "wasm") {
            let bot = BotMonte::new();
            bot.choose_trump(&game_clone);
        } else {
            std::thread::spawn(move || {
                let bot = BotMonte::new();
                bot.choose_trump(&game_clone);
            });
        }
    }

    fn spawn_play_bot(&self) {
        let mut game_clone = self.game.clone();

        if cfg!(target_family = "wasm") {
            let bot = BotMonte::new();
            bot.get_play(&mut game_clone, 500);
        } else {
            std::thread::spawn(move || {
                let bot = BotMonte::new();
                bot.get_play(&mut game_clone, 500);
            });
        }
    }
}
