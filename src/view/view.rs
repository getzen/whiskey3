use std::sync::mpsc::Sender;

use macroquad::prelude::*;

// use foldhash::{HashMap, HashMapExt};

use crate::{
    card::{Card, Id, Suit},
    game::{Game, PlayerAction},
    view::{button_state::ButtonState, card_view::CardView},
};

use super::{
    bid_marker::BidMarker,
    bid_panel::BidPanel,
    button_shaded::ButtonShaded,
    button_text::ButtonText,
    imager::Imager,
    score_table::ScoreTable,
    texter::AlignH,
    texter_multi::TexterMulti,
    trump_chooser::TrumpChooser,
    trump_marker::TrumpMarker,
    view_geom::{
        self, bid_marker_geom, BID_PANEL_POS, CENTER, DONE_EXCHANGING_BUTTON_POS, MESSAGE_POS,
        PLAY_BUTTON_POS, SCORE_TABLE_POS, TRUMP_CHOOSER_POS,
    },
};

// Global variable, created in new() below. To access:
// let font = BODY_FONT.lock().unwrap().clone().unwrap();
use std::sync::Mutex;
pub static BODY_FONT: Mutex<Option<Font>> = Mutex::new(None);

pub struct View {
    card_views: Vec<CardView>,
    turn_marker: Imager,
    score_table: ScoreTable,
    play_button: ButtonShaded,
    bid_markers: Vec<BidMarker>,
    bid_panel: BidPanel,
    done_exchanging_button: ButtonText,
    trump_chooser: TrumpChooser,
    trump_marker: TrumpMarker,
    sender: Sender<PlayerAction>,
    z_order_needs_update: bool,

    message: TexterMulti,
    //trans_animators: HashMap<Id, TranslationAnimator>,
}

impl View {
    pub async fn new(players: usize, sender: Sender<PlayerAction>) -> Self {
        let font = load_ttf_font("./src/assets/Menlo-Bold.ttf").await.unwrap();
        {
            // This is in a block so that body_font goes out of scope (and unlocked) after it's set.
            let mut body_font = BODY_FONT.lock().unwrap();
            *body_font = Some(font.clone());
        }

        let texture = load_texture("src/assets/circle.png").await.unwrap();
        let turn_marker = Imager::new(texture, 0.4, true);

        let play_button_tex = load_texture("src/assets/play_button@2x.png").await.unwrap();
        let mut play_button = ButtonShaded::new(PLAY_BUTTON_POS, play_button_tex, 0.5);
        play_button.visible = false;

        let mut bid_markers = Vec::new();
        for p in 0..players {
            let geom = bid_marker_geom(p, players);
            let marker = BidMarker::new(geom.pos);
            bid_markers.push(marker);
        }

        let mut done_exchanging_button = ButtonText::new(
            DONE_EXCHANGING_BUTTON_POS,
            "Done",
            font.clone(),
            16,
            vec2(80.0, 40.0),
        );
        done_exchanging_button.sender = Some(sender.clone());
        done_exchanging_button.action = Some(PlayerAction::DoneExchanging);
        done_exchanging_button.visible = false;

        Self {
            card_views: Vec::new(),
            turn_marker,
            //message,
            score_table: ScoreTable::new(SCORE_TABLE_POS, font.clone()),
            play_button,
            bid_markers,
            bid_panel: BidPanel::new(0, 0, BID_PANEL_POS, sender.clone()),
            done_exchanging_button,
            trump_chooser: TrumpChooser::new(TRUMP_CHOOSER_POS, sender.clone()).await,
            trump_marker: TrumpMarker::new(CENTER),
            sender,
            z_order_needs_update: false,
            message: TexterMulti::new(MESSAGE_POS),
            //trans_animators: HashMap::new(),
        }
    }

    async fn texture_for(&self, card: &Card) -> Texture2D {
        let path = format!("src/assets/cards/{}.png", card.file_string());
        load_texture(&path).await.unwrap()
    }

    pub async fn create_card_view(&mut self, card: &Card) {
        let back = load_texture("src/assets/cards/back.png").await.unwrap();
        let face = self.texture_for(card).await;

        let mut view = CardView::new(
            card.id,
            face,
            back.clone(),
            card.points,
            self.sender.clone(),
        );
        view.move_to(view_geom::DECK_POS, 100.0);
        self.card_views.push(view);
    }

    // fn find_card_view(&self, card_id: Id) -> Option<&CardView> {
    //     for card_view in &self.card_views {
    //         if card_view.id == card_id {
    //             return Some(card_view);
    //         }
    //     }
    //     None
    // }

    fn find_card_view_mut(&mut self, card_id: Id) -> Option<&mut CardView> {
        self.card_views
            .iter_mut()
            .find(|card_view| card_view.id == card_id)
    }

    fn sort_card_views_by_z_order(&mut self) {
        self.card_views
            .sort_by(|a, b| a.card_image.z_order.cmp(&b.card_image.z_order));
    }

    pub fn check_events(&mut self) {
        // Key presses
        if is_key_released(KeyCode::Escape) {
            self.sender
                .send(PlayerAction::ShouldExit)
                .expect("Send error");
        }

        let mouse_pos: Vec2 = mouse_position().into();

        // if self.play_button.process_events(None, mouse_pos) {
        //     self.play_button.visible = false;
        //     return;
        // }

        self.score_table.process_events(None, mouse_pos);

        if self.bid_panel.process_events(None, mouse_pos) {
            return;
        }

        if self.done_exchanging_button.process_events(None, mouse_pos) {
            return;
        }

        if self.trump_chooser.process_events(None, mouse_pos) {
            return;
        }

        // Cards
        for card_view in self.card_views.iter_mut().rev() {
            if card_view.process_events(None, mouse_pos) {
                return;
            }
        }
    }

    pub fn update(&mut self, time_delta: f32) {
        // // Translation animators
        // let mut completed_ids = Vec::new();
        // for (card_id, anim) in &mut self.trans_animators {
        //     if let Some(card_view) = self.card_views.iter_mut().find(|c| c.id == *card_id) {
        //         card_view.transform.translation = anim.update(time_delta);
        //     }
        //     if anim.completed {
        //         completed_ids.push(*card_id);
        //     }
        // }
        // // Remove the completed animators.
        // for card_id in completed_ids {
        //     self.trans_animators.remove(&card_id);
        // }

        for view in &mut self.card_views {
            view.update(time_delta);
        }
        if self.z_order_needs_update {
            self.sort_card_views_by_z_order();
            self.z_order_needs_update = false;
        }

        for bid_marker in &mut self.bid_markers {
            bid_marker.update(time_delta);
        }

        self.score_table.update(time_delta);
    }

    pub fn update_info(&mut self, game: &Game) {
        self.score_table.visible = true;
        self.score_table.update_scoring(&game);

        self.turn_marker.visible = true;
        let geom = view_geom::turn_marker_geom(game.active, game.options.players);
        self.turn_marker.transform.translation = geom.pos;
    }

    pub fn update_message(&mut self, texts: &[&str]) {
        self.message.clear_lines();
        let font = BODY_FONT.lock().unwrap().clone().unwrap();

        for text in texts {
            self.message
                .add_line(text, font.clone(), 18, AlignH::Center, 20.0);
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

    pub fn update_nest(&mut self, game: &Game, aside: bool) {
        for (idx, card) in game.nest.iter().enumerate() {
            if let Some(view) = self.find_card_view_mut(card.id) {
                let geom = match aside {
                    true => view_geom::nest_aside_geom(idx, game.nest.len()),
                    false => view_geom::nest_geom(idx, game.nest.len()),
                };
                view.move_to(geom.pos, view_geom::CARD_SPEED);
                view.rotate_to(geom.rot, view_geom::ROT_SPEED);
                view.card_image.z_order = geom.z;
                view.set_face_up(card.face_up);
            }
        }
        self.z_order_needs_update = true;
    }

    pub fn update_bids(&mut self, game: &Game) {
        for (p, opt_bid) in game.bids.iter().enumerate() {
            self.bid_markers[p].update_with_bid(opt_bid.clone());
        }
    }

    pub fn hide_bids_except_maker(&mut self, game: &Game) {
        for p in 0..game.options.players {
            if game.maker.unwrap() == p {
                continue;
            }
            self.bid_markers[p].visible = false;
        }
    }

    pub fn update_hand(&mut self, game: &Game, player: usize) {
        let hand = &game.hands[player];
        let is_bot = game.bot_players[player];

        for (idx, card) in hand.iter().enumerate() {
            if let Some(view) = self.card_views.iter_mut().find(|view| view.id == card.id) {
                let geom = view_geom::hand_card_geom(
                    player,
                    idx,
                    hand.len(),
                    game.options.players,
                    is_bot,
                );
                view.move_to(geom.pos, view_geom::CARD_SPEED);
                view.rotate_to(geom.rot, view_geom::ROT_SPEED);
                view.card_image.z_order = geom.z;
                view.set_face_up(card.face_up);
                // card.eligible and card_view.dimmed not handled here
            }
        }
        self.z_order_needs_update = true;
    }

    pub fn update_trick(&mut self, game: &Game) {
        for (idx, opt_card) in game.trick.cards.iter().enumerate() {
            if let Some(card) = opt_card {
                if let Some(view) = self.card_views.iter_mut().find(|view| view.id == card.id) {
                    let geom = view_geom::trick_card_geom(idx, game.options.players);
                    view.move_to(geom.pos, view_geom::CARD_SPEED);
                    view.rotate_to(geom.rot, view_geom::ROT_SPEED);
                    view.card_image.z_order = geom.z;
                    view.set_face_up(true);
                }
            }
        }
        self.z_order_needs_update = true;
    }

    pub fn update_taken(&mut self, game: &Game) {
        for p in 0..game.options.players {
            for card in &game.taken[p] {
                if let Some(view) = self.card_views.iter_mut().find(|view| view.id == card.id) {
                    let geom = view_geom::taken_geom(p, game.options.players);
                    view.move_to(geom.pos, view_geom::CARD_SPEED);
                    view.rotate_to(geom.rot, view_geom::ROT_SPEED);
                    view.card_image.z_order = geom.z;
                    view.set_face_up(false);
                }
            }
        }
    }

    // After player exchanges or plays a card, call this to reset the nest or hand.
    pub fn reset_eligibility(&mut self, cards: &[Card]) {
        for card in cards {
            if let Some(view) = self.card_views.iter_mut().find(|view| view.id == card.id) {
                view.dimmed = false;
                view.action = None;
            }
        }
    }

    pub fn get_human_exchanges(&mut self) {
        self.update_message(&[
            "Discard to the nest.",
            "Any point cards go to your opponents",
            "at the end of the hand",
        ]);
        self.show_done_exchanging_button(false);
    }

    pub fn show_done_exchanging_button(&mut self, enabled: bool) {
        self.done_exchanging_button.visible = true;
        match enabled {
            true => self.done_exchanging_button.state = ButtonState::Normal,
            false => self.done_exchanging_button.state = ButtonState::Disabled,
        }
    }

    pub fn hide_done_exchanging_button(&mut self) {
        self.done_exchanging_button.visible = false;
    }

    pub fn set_discardable_hand_cards(&mut self, game: &Game) {
        let maker = game.maker.unwrap();
        let hand = &game.hands[maker];
        for card in hand {
            if let Some(view) = self.card_views.iter_mut().find(|view| view.id == card.id) {
                view.dimmed = !card.eligible;
                if card.eligible {
                    view.action = Some(PlayerAction::Exchange(card.id));
                }
            }
        }
    }

    pub fn show_trump_chooser(&mut self) {
        self.trump_chooser.visible = true;
        self.update_message(&["Select trump suit."]);
    }

    pub fn hide_trump_chooser(&mut self) {
        self.trump_chooser.visible = false;
        self.update_message(&[""]);
    }

    pub async fn set_trump_suit(&mut self, suit: Option<Suit>) {
        self.trump_marker.set_suit(suit).await;
    }

    pub fn set_playable_hand_cards(&mut self, game: &Game) {
        let hand = &game.hands[game.active];
        for card in hand {
            if let Some(view) = self.card_views.iter_mut().find(|view| view.id == card.id) {
                view.dimmed = !card.eligible;
                if card.eligible {
                    view.action = Some(PlayerAction::PlayCard(card.id));
                }
            }
        }
    }

    pub fn get_human_bid(&mut self, game: &Game) {
        // Hide bid marker for human.
        self.bid_markers[game.active].visible = false;

        self.bid_panel.min_bid = game.min_current_bid();
        self.bid_panel.update_bid_amount(game.min_current_bid());
        self.bid_panel.max_bid = game.options.max_bid;
        self.update_message(&["Your bid."]);
        self.bid_panel.visible = true;
    }

    pub fn end_human_bid(&mut self, _game: &Game) {
        self.bid_panel.visible = false;
    }

    pub fn get_bot_discards(&mut self, _game: &Game) {
        self.update_message(&["Bot thinking."]);
    }

    pub fn get_bot_trump(&mut self, _game: &Game) {
        self.update_message(&[""]);
    }

    pub fn get_human_card_play(&mut self, game: &Game) {
        self.set_playable_hand_cards(game);
        self.update_message(&["Play card."]);
    }

    pub fn get_bot_card_play(&mut self, _game: &Game) {
        self.update_message(&[""]);
    }

    pub async fn draw(&mut self) {
        clear_background(Color::from_rgba(100, 100, 100, 255));

        //self.turn_marker.draw();
        self.trump_marker.draw();

        for marker in &mut self.bid_markers {
            marker.draw(None);
        }

        for view in &mut self.card_views {
            view.draw();
        }

        self.score_table.draw(None);

        self.play_button.draw(None);
        self.bid_panel.draw(None);
        self.done_exchanging_button.draw(None);
        self.trump_chooser.draw(None);

        self.message.draw(None);

        next_frame().await;
    }
}
