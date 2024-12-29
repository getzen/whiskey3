use std::sync::mpsc::{self, Receiver, Sender};

use crate::bot_monte::BotMonte;
use crate::game::{Game, PlayerAction};

use crate::state_mgr::{ControllerMsg, State, StateMgr, ViewUpdate};
use crate::view::view::View;

pub struct Controller {
    game: Game,
    view: Option<View>,
    state_mgr: StateMgr,
    sender: Sender<PlayerAction>,
    receiver: Receiver<PlayerAction>,
}

impl Controller {
    pub async fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        let view = View::new(sender.clone());

        Self {
            game: Game::new(),
            view: Some(view.await),
            state_mgr: StateMgr::new(),
            sender,
            receiver,
        }
    }

    pub async fn go(&mut self) {
        let mut last_time = macroquad::time::get_time();

        self.game.create_deck();

        if let Some(view) = &mut self.view {
            for card in &self.game.deck {
                view.create_card_view(card).await;
            }
            view.update_deck(&self.game);
        }

        loop {
            let time_delta = (macroquad::time::get_time() - last_time) as f32;
            last_time = macroquad::time::get_time();

            self.state_mgr.process(time_delta, &mut self.game);

            // Process the events in the view update queue.
            for event in &self.state_mgr.view_queue {
                if let Some(view) = &mut self.view {
                    match event {
                        ViewUpdate::Info(state) => {
                            view.update_info(&mut self.game, state);
                        }
                        ViewUpdate::Deck => {
                            view.update_deck(&self.game);
                        }
                        ViewUpdate::Nest => {
                            view.update_nest(&self.game);
                        }
                        ViewUpdate::Taken => {
                            view.update_taken(&self.game);
                        }
                        ViewUpdate::Hand(player) => {
                            view.update_hand(&self.game, *player);
                        }
                    }
                }
            }
            self.state_mgr.view_queue.clear();

            // Check for ControllerMsg
            if let Some(message) = self.state_mgr.controller_msg.take() {
                match message {
                    ControllerMsg::GetPlay => {
                        println!("GetPlay");
                        if self.game.bot_active() {
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
                        } else if let Some(view) = &mut self.view {
                            view.get_human_play(&mut self.game);
                        }
                    }
                }
            }

            let should_exit = self.check_player_actions();
            if should_exit {
                break;
            }

            // Update and draw the view.
            if let Some(view) = &mut self.view {
                view.process_events();
                view.update(time_delta);
                view.draw().await;
            }
        }
    }

    // Return true to exit app.
    fn check_player_actions(&mut self) -> bool {
        let received = self.receiver.try_recv();
        if received.is_ok() {
            match received.unwrap() {
                PlayerAction::PlayCard(card_id) => {
                    println!("PlayCard id: {}", card_id);
                    // Pass it to the StateMgr for handling.
                    self.state_mgr
                        .state_queue
                        .push_back(State::MakingPlay(card_id));
                }
                PlayerAction::ShouldExit => return false,
            }
        }
        return false;
    }
}
