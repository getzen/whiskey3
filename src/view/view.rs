use std::{collections::HashSet, sync::mpsc::Sender};

use macroquad::prelude::*;

use crate::{
    card::{Card, SelectState},
    game::{self, Bid, Game, PlayerAction, MIN_BID, NEST_SIZE, PLAYERS},
    state_mgr::State,
    view::{button_state::ButtonState, card_view::CardView},
};

use super::{
    bid_marker::BidMarker, bid_panel::BidPanel, button_shaded::ButtonShaded, button_text::ButtonText, eventer::EventerEvent, score_table::ScoreTable, sprite::Sprite, texter::Texter, view_geom::{self, bid_marker_geom, ViewGeom, BID_PANEL_POS, MESSAGE_POS, PLAY_BUTTON_POS, SCORE_TABLE_POS}
};


pub struct View {
    card_views: Vec<CardView>,
    turn_marker: Sprite,
    message: Texter,
    score_table: ScoreTable,
    play_button: ButtonShaded,
    bid_markers: Vec<BidMarker>,
    bid_panel: BidPanel,
    sender: Sender<PlayerAction>,
    // For human card play:
    playable_card_ids: Vec<u8>,
    hand_table_ids: HashSet<u8>,
    selected_ids: HashSet<u8>,
    z_order_needs_update: bool,
}

impl View {
    pub async fn new(sender: Sender<PlayerAction>) -> Self {
        let turn_tex = load_texture("src/assets/circle.png").await.unwrap();

        let play_button_tex = load_texture("src/assets/play_button@2x.png").await.unwrap();
        let mut play_button = ButtonShaded::new(0, PLAY_BUTTON_POS, play_button_tex, 0.5);
        play_button.state = ButtonState::Hidden;

        let mut message = Texter::new("message", 16, Some("Menlo-Bold.ttf"), false, false).await;
        message.transform.position = MESSAGE_POS;
        message.centered_horiz = true;

        let mut bid_markers = Vec::new();
        for p in 0..PLAYERS {
            let geom = bid_marker_geom(p, PLAYERS);
            let marker = BidMarker::new(geom.pos).await;
            bid_markers.push(marker);
        }

        Self {
            card_views: Vec::new(),
            turn_marker: Sprite::new(turn_tex, 0.4),
            message,
            score_table: ScoreTable::new(SCORE_TABLE_POS).await,
            play_button,
            bid_markers,
            bid_panel: BidPanel::new(65, 120, BID_PANEL_POS).await,
            sender,
            playable_card_ids: Vec::new(),
            hand_table_ids: HashSet::new(),
            selected_ids: HashSet::new(),
            z_order_needs_update: false,
        }
    }

    async fn texture_for(&self, card: &Card) -> Texture2D {
        let path = format!("src/assets/cards/{}.png", card.file_string());
        load_texture(&path).await.unwrap()
    }

    pub async fn create_card_view(&mut self, card: &Card) {
        let back = load_texture("src/assets/cards/back.png").await.unwrap();
        let face = self.texture_for(card).await;

        let mut view = CardView::new(card.id, face, back.clone());
        view.transform.position = view_geom::DECK_POS;
        self.card_views.push(view);
    }

    #[allow(dead_code)]
    fn find_card_view(&self, card_id: u8) -> Option<&CardView> {
        for card_view in &self.card_views {
            if card_view.id == card_id {
                return Some(card_view);
            }
        }
        None
    }

    fn find_card_view_mut(&mut self, card_id: u8) -> Option<&mut CardView> {
        for card_view in &mut self.card_views {
            if card_view.id == card_id {
                return Some(card_view);
            }
        }
        None
    }

    fn sort_card_views_by_z_order(&mut self) {
        self.card_views
            .sort_by(|a, b| a.card_image.z_order.cmp(&b.card_image.z_order));
    }

    pub fn check_events(&mut self) {
        // Key presses
        if is_key_released(KeyCode::Escape) {
            self.sender.send(PlayerAction::ShouldExit).expect("Send error");
        }

        let mouse_pos: Vec2 = mouse_position().into();

        // Buttons
        if self.play_button.process_events(&mouse_pos) {
            println!("play clicked");
            self.play_button.state = ButtonState::Hidden;
            return;
        }

        if let Some(action) = self.bid_panel.process_events(&mouse_pos) {
            // Some actions are handled internally.
            match &action {
                PlayerAction::Bid(bid) => {
                    println!("bid: {:?}", bid);
                    // hide bid panel
                },
                PlayerAction::IncBid => todo!(),
                PlayerAction::DecBid => todo!(),
                _ => {}, // not produced by the bid panel
            }
            self.sender.send(action).expect("Send error");
            return;
        }

        // Cards
        let mut card_id_clicked = None;

        for view in self.card_views.iter_mut().rev() {
            if let Some(event) = view.process_events(&mouse_pos) {
                match event {
                    EventerEvent::LeftMouseReleased => {
                        let state = view.select_state.clone();
                        match state {
                            SelectState::Eligible | SelectState::Selected => {
                                card_id_clicked = Some(view.id);
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
                break; // We are over a card, so we're done regardless.
            }
        }
        if let Some(id) = card_id_clicked {
            self.process_card_click_id(id);
        }
    }

    pub fn update(&mut self, time_delta: f32) {
        for view in &mut self.card_views {
            view.update(time_delta);
        }
        if self.z_order_needs_update {
            self.sort_card_views_by_z_order();
            self.z_order_needs_update = false;
        }
        self.turn_marker.update(time_delta);
    }

    pub fn update_info(&mut self, game: &mut Game, state: &State) {
        self.score_table
            .update(&game.scores, &game.maker, &game.high_bid, &[0, 0]);

        self.turn_marker.imager.visible = true;
        let geom = view_geom::turn_marker_geom(game.active, game::PLAYERS);
        self.turn_marker.move_to(geom.pos, 300.0);

        // Message / buttons
        match state {
            State::Init => {
                self.message.text = "Welcome to Whiskey".to_owned();
            }
            //     State::Dealing { to_deal } => todo!(),
            //     State::DealingToTable { to_deal } => todo!(),
            //     State::PreparingForNewTurn => todo!(),
            State::GettingBid => {
            }
            State::CompletingBidding => {
                self.message.text = format!("Player {} wins the bidding.", game.maker.unwrap());
            }
            State::GettingPlay => {
                if game.bot_is_active() {
                    self.message.text = "Bot Thinking".to_owned();
                } else {
                    self.message.text = "Your Turn".to_owned();
                }
            }
            //     State::WaitingForPlay => todo!(),
            //     State::MakingPlay(card_play) => todo!(),
            //     State::CompletingPlay => todo!(),
            State::CompletingGame(we, they) => {
                self.message.text = format!("Hand Over. We: {}, They: {}", we, they);
            }
            _ => {}
        }
    }

    pub fn update_deck(&mut self, game: &Game) {
        for (idx, card) in game.deck.iter().enumerate() {
            if let Some(view) = self.find_card_view_mut(card.id) {
                let geom = view_geom::deck_geom(idx);
                view.move_to(geom.pos, view_geom::CARD_SPEED);
                view.rotate_to(geom.rot, view_geom::ROT_SPEED);
                view.card_image.z_order = geom.z;
                view.set_face_up(false)
            }
        }
        self.z_order_needs_update = true;
    }

    pub fn update_nest(&mut self, game: &Game) {
        for (idx, card) in game.nest.iter().enumerate() {
            if let Some(view) = self.find_card_view_mut(card.id) {
                let geom = view_geom::nest_geom(idx, NEST_SIZE);
                view.move_to(geom.pos, view_geom::CARD_SPEED);
                view.rotate_to(geom.rot, view_geom::ROT_SPEED);
                view.card_image.z_order = geom.z;
                view.set_face_up(card.face_up);
            }
        }
        self.z_order_needs_update = true;
    }

    pub fn update_bids(&mut self, game: &Game) {
        for (idx, opt_bid) in game.bids.iter().enumerate() {
            self.bid_markers[idx].update_with_bid(opt_bid.clone());
        }
    }

    pub fn update_taken(&mut self, game: &Game) {
        for team in 0..2 {
            for (idx, card) in game.taken[team].iter().rev().enumerate() {
                let geom = view_geom::taken_geom(team, idx);
                if let Some(view) = self.find_card_view_mut(card.id) {
                    view.move_to(geom.pos, view_geom::CARD_SPEED_TAKE);
                    view.rotate_to(geom.rot, view_geom::ROT_SPEED);
                    view.card_image.z_order = 200 - idx;
                    view.set_face_up(card.face_up);
                }
            }
        }
        self.z_order_needs_update = true;
    }

    pub fn update_hand(&mut self, game: &Game, player: usize) {
        let hand = &game.hands[player];
        let is_bot = game.bot_players[player];

        for (idx, card) in hand.iter().enumerate() {
            if let Some(view) = self.card_views.iter_mut().find(|view| view.id == card.id) {
                let geom =
                    view_geom::hand_card_geom(player, idx, hand.len(), game::PLAYERS, is_bot);
                view.move_to(geom.pos, view_geom::CARD_SPEED);
                view.rotate_to(geom.rot, view_geom::ROT_SPEED);
                view.card_image.z_order = geom.z;
                view.set_face_up(card.face_up);
                view.set_select_state(card.select_state.clone());
            }
        }
        self.z_order_needs_update = true;
    }

    pub async fn draw(&mut self) {
        clear_background(Color::from_rgba(100, 100, 100, 255));
        

        let gl = unsafe { get_internal_gl().quad_gl };
        let matrix = glam::Mat4::from_translation(vec3(100.0, 0.0, 0.0));
        gl.push_model_matrix(matrix);
        gl.pop_model_matrix();


        self.turn_marker.draw();
        for marker in &mut self.bid_markers {
            marker.draw();
        }

        for view in &mut self.card_views {
            view.draw();
        }

        self.message.draw();
        self.score_table.draw();
        self.play_button.draw();
        self.bid_panel.draw();
        //draw_multiline_text_ex("Hello, \nWorld.", 300.0, 300.0, Some(1.0), TextParams::default());

        next_frame().await;
    }

    pub fn get_human_bid(&mut self, game: &Game) {
        // Hide bid marker for human.
        self.bid_markers[game.active].visible = false;

        match &game.high_bid {
            Some(bid) => {
                match bid {
                    Bid::Pass => todo!(),
                    Bid::Bid(b) => {
                        self.bid_panel.min_bid = b + 5;
                        self.bid_panel.update_bid_amount(b + 5);
                    }
                }
            },
            None => {
                self.bid_panel.min_bid = MIN_BID;
                self.bid_panel.update_bid_amount(MIN_BID);
            },
        }
        println!("getting human bid");
        self.bid_panel.visible = true;
    }

    pub fn end_human_bid(&mut self, game: &Game) {
        self.bid_panel.visible = false;
        self.bid_markers[game.active].visible = true;
    }

    pub fn get_human_play(&mut self, game: &mut Game) {
        self.playable_card_ids = game.get_playable_card_ids();

        self.hand_table_ids.clear();
        for card in &game.hands[game.active] {
            self.hand_table_ids.insert(card.id);
        }
        for card in &game.nest {
            self.hand_table_ids.insert(card.id);
        }

        for view in &mut self.card_views {
            if self.hand_table_ids.contains(&view.id) {
                view.set_select_state(SelectState::Eligible);
            } else {
                view.set_select_state(SelectState::OutOfScope);
            }
        }

        self.selected_ids.clear();
    }

    fn process_card_click_id(&mut self, id: u8) {
        if self.selected_ids.contains(&id) {
            self.selected_ids.remove(&id);
            if let Some(view) = self.card_views.iter_mut().find(|view| view.id == id) {
                view.set_select_state(SelectState::Eligible);
            }
        } else {
            self.selected_ids.insert(id);
            if let Some(view) = self.card_views.iter_mut().find(|view| view.id == id) {
                view.set_select_state(SelectState::Selected);
            }
        }
        
    }
}
