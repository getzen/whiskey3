use std::sync::mpsc::Sender;

use hashbrown::HashMap;
use macroquad::prelude::*;

use crate::{
    card::{Card, Id, Suit},
    game::{Game, PlayerAction},
    view::{button_state::ButtonState, card_ent::CardEnt},
};

use super::{
    bid_marker::BidMarker,
    bid_panel::BidPanel,
    button_text::ButtonText,
    score_table::ScoreTable,
    sprite::Sprite,
    texter_multi::TexterMulti,
    trump_chooser::TrumpChooser,
    trump_marker::TrumpMarker,
    turn_marker::TurnMarker,
    view_geom::{
        self, BID_PANEL_POS, DONE_EXCHANGING_BUTTON_POS, MESSAGE_POS, NEXT_HAND_BUTTON_POS,
        PLAY_CENTER, SCORE_TABLE_POS, TRUMP_CHOOSER_POS, ViewGeom, Z, bid_marker_geom,
    },
};

use super::view_entity::ViewEnt;

use std::sync::OnceLock;
pub static FONT: OnceLock<Font> = OnceLock::new();

/// ZOrder can be put into a Vec and sorted by z to provide a drawing
/// and event-checking order for the ids.
struct ZOrder {
    id: Id,
    z: Z,
}

pub struct View {
    id: Id,
    view_entities: HashMap<Id, ViewEnt>,
    z_orders: Vec<ZOrder>,
    turn_marker: Id,
    bid_marker: Id,
    trump_chooser: Id,

    card_views: Vec<CardEnt>,
    score_table: ScoreTable,
    bid_markers: Vec<BidMarker>,
    bid_panel: BidPanel,
    done_exchanging_button: ButtonText,
    trump_marker: TrumpMarker,
    next_hand_button: ButtonText,
    sender: Sender<PlayerAction>,
    z_order_needs_update: bool,

    message: TexterMulti,
}

impl View {
    pub async fn new(players: usize, sender: Sender<PlayerAction>) -> Self {
        let font = load_ttf_font("./src/assets/Menlo-Bold.ttf").await.unwrap();
        FONT.set(font.clone()).expect("Error setting FONT.");

        let texture = load_texture("src/assets/circle.png").await.unwrap();
        let mut turn_marker = Sprite::new(texture);
        turn_marker.set_size_from_multiplier(0.4);

        let mut bid_markers = Vec::new();
        for p in 0..players {
            let geom = bid_marker_geom(p, players);
            let marker = BidMarker::new(geom.pos);
            bid_markers.push(marker);
        }

        let mut done_exchanging_button =
            ButtonText::new(DONE_EXCHANGING_BUTTON_POS, "Done", font.clone(), 16, vec2(80.0, 40.0));
        done_exchanging_button.click_action = Some(PlayerAction::DoneExchanging);
        done_exchanging_button.visible = false;

        let mut next_hand_button =
            ButtonText::new(NEXT_HAND_BUTTON_POS, "Next Hand", font.clone(), 16, vec2(120.0, 40.0));
        next_hand_button.click_action = Some(PlayerAction::NextHand);
        next_hand_button.visible = false;

        Self {
            // Set id high enough so that we don't step on the card ids.
            id: 100,
            view_entities: HashMap::<Id, ViewEnt>::new(),
            z_orders: Vec::new(),
            // These id values are assigned in setup().
            turn_marker: 0,
            bid_marker: 0,
            trump_chooser: 0,

            card_views: Vec::new(),
            score_table: ScoreTable::new(SCORE_TABLE_POS, font.clone()),
            bid_markers,
            bid_panel: BidPanel::new(BID_PANEL_POS, 0, 0),
            done_exchanging_button,
            trump_marker: TrumpMarker::new(PLAY_CENTER),
            next_hand_button,
            sender,
            z_order_needs_update: false,
            message: TexterMulti::new(MESSAGE_POS),
        }
    }

    fn next_id(&mut self) -> Id {
        self.id += 1;
        self.id
    }

    pub async fn setup(&mut self) {
        // Cards are created from Controller call in GameAction::Setup.
        self.turn_marker = self.create_turn_marker().await;
        self.trump_chooser = self.create_trump_chooser().await;
    }

    async fn create_turn_marker(&mut self) -> Id {
        let id = self.next_id();
        let tex = load_texture("src/assets/circle.png").await.unwrap();
        let mut entity = TurnMarker::new(tex);
        entity.set_translation(vec2(100.0, 100.0));

        self.view_entities.insert(id, ViewEnt::TurnMarker(entity));
        self.z_orders.push(ZOrder { id, z: 0 });
        id
    }

    async fn create_trump_chooser(&mut self) -> Id {
        let id = self.next_id();
        let entity = TrumpChooser::new(TRUMP_CHOOSER_POS).await;
        self.view_entities.insert(id, ViewEnt::TrumpChooser(entity));
        self.z_orders.push(ZOrder { id, z: 0 });
        id
    }

    async fn texture_for(&self, card: &Card) -> Texture2D {
        let path = format!("src/assets/cards/{}.png", card.file_string());
        load_texture(&path).await.unwrap()
    }

    pub async fn create_card_view(&mut self, card: &Card) {
        let back = load_texture("src/assets/cards/back.png").await.unwrap();
        let face = self.texture_for(card).await;

        let mut entity = CardEnt::new(card.id, face, back.clone(), card.points);
        entity.transform.translation = view_geom::PLAY_CENTER;
        self.view_entities.insert(card.id, ViewEnt::CardEnt(entity));
        self.z_orders.push(ZOrder { id: card.id, z: 0 });
    }

    fn set_z_order(&mut self, id: Id, z: Z) {
        if let Some(z_order) = self.z_orders.iter_mut().find(|z| z.id == id) {
            z_order.z = z;
        }
        self.z_order_needs_update = true;
    }

    fn sort_entities_by_z_order(&mut self) {
        self.z_orders.sort_by_key(|f| f.z);
        self.z_order_needs_update = false;
    }

    pub fn check_events(&mut self) {
        // Key presses
        if is_key_released(KeyCode::Escape) {
            self.sender.send(PlayerAction::ShouldExit).expect("Send error");
        }

        let mouse_pos: Vec2 = mouse_position().into();

        for z_order in &self.z_orders {
            let entity = self.view_entities.get_mut(&z_order.id).unwrap();
            let done = entity.process_mouse(mouse_pos);
            if done {
                break;
            }
        }

        let transform = super::transform::Transform::default();

        if self.score_table.process_mouse(&mouse_pos, &transform) {
            return;
        }

        if self.bid_panel.process_mouse(&mouse_pos) {
            return;
        }

        if self.done_exchanging_button.process_mouse(&mouse_pos, &transform) {
            return;
        }

        if self.next_hand_button.process_mouse(&mouse_pos, &transform) {
            return;
        }
    }

    pub fn update(&mut self, time_delta: f32) {
        for z_order in &self.z_orders {
            let entity = self.view_entities.get_mut(&z_order.id).unwrap();
            entity.update(time_delta);
        }

        if self.z_order_needs_update {
            self.sort_entities_by_z_order();
        }

        for bid_marker in &mut self.bid_markers {
            bid_marker.update(time_delta);
        }

        self.score_table.update(time_delta);
    }

    pub fn update_info(&mut self, game: &Game) {
        self.score_table.visible = true;
        self.score_table.update_scoring(&game);

        let entity = self.view_entities.get_mut(&self.turn_marker).unwrap();
        if let ViewEnt::TurnMarker(marker) = entity {
            marker.visible = true;
            let geom = view_geom::turn_marker_geom(game.active, game.options.players);
            marker.set_translation(geom.pos);
        }
    }

    pub fn update_message(&mut self, texts: &[&str]) {
        self.message.clear_lines();
        let font = FONT.get().unwrap();

        for text in texts {
            self.message.add_line(text, font.clone(), 18, super::text::AlignH::Center, 20.0);
        }
    }

    fn update_card_ent(&mut self, id: Id, geom: ViewGeom, face_up: bool) {
        let entity = self.view_entities.get_mut(&id).unwrap();
        if let ViewEnt::CardEnt(card_ent) = entity {
            //view.move_to(geom.pos, view_geom::CARD_SPEED);
            //view.rotate_to(geom.rot, view_geom::ROT_SPEED);
            card_ent.set_face_up(face_up);
        }
        self.set_z_order(id, geom.z);
    }

    pub fn update_deck(&mut self, game: &Game) {
        for (idx, card) in game.deck.iter().enumerate() {
            let geom = view_geom::deck_geom(Some(game.dealer), game.options.players, idx);
            self.update_card_ent(card.id, geom, card.face_up);
        }
    }

    pub fn update_exchange(&mut self, game: &Game) {
        for (idx, card) in game.exchange.iter().enumerate() {
            let geom = view_geom::nest_exchange_geom(idx, game.exchange.len());
            self.update_card_ent(card.id, geom, card.face_up);
        }
    }

    pub fn update_nest(&mut self, game: &Game, aside: bool) {
        for (idx, card) in game.nest.iter().enumerate() {
            let geom = match aside {
                true => view_geom::nest_aside_geom(idx, game.nest.len()),
                false => view_geom::nest_exchange_geom(idx, game.nest.len()),
            };
            self.update_card_ent(card.id, geom, card.face_up);
        }
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
            let geom = view_geom::hand_card_geom(player, idx, hand.len(), game.options.players, is_bot);
            self.update_card_ent(card.id, geom, card.face_up);
        }
    }

    pub fn update_trick(&mut self, game: &Game) {
        for (idx, opt_card) in game.trick.cards.iter().enumerate() {
            if let Some(card) = opt_card {
                let geom = view_geom::trick_card_geom(idx, game.options.players);
                self.update_card_ent(card.id, geom, card.face_up);
            }
        }
    }

    pub fn update_taken(&mut self, game: &Game) {
        for p in 0..game.options.players {
            for card in &game.taken[p] {
                let geom = view_geom::taken_geom(p, game.options.players);
                self.update_card_ent(card.id, geom, card.face_up);
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

    pub fn get_human_exchanges(&mut self, game: &Game) {
        let count = game.options.exchange_size - game.exchange.len();
        let text = format!("Discard {} cards", count);
        self.update_message(&[&text]);
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

    pub fn show_trump_chooser(&mut self, visible: bool) {
        let entity = self.view_entities.get_mut(&self.trump_chooser).unwrap();
        if let ViewEnt::TrumpChooser(chooser) = entity {
            chooser.visible = visible
        }

        if visible {
            self.update_message(&["Select trump suit."]);
        } else {
            self.update_message(&[""]);
        }
    }

    pub fn show_trump_marker(&mut self, visible: bool) {
        self.trump_marker.visible = visible;
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

    pub fn show_next_hand_button(&mut self, visible: bool) {
        self.next_hand_button.visible = visible;
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

    pub fn hide_bid_marker(&mut self, bidder: usize) {
        self.bid_markers[bidder].visible = false;
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

        for z_order in &self.z_orders {
            let entity = self.view_entities.get_mut(&z_order.id).unwrap();
            entity.draw();
        }

        self.trump_marker.draw();

        for marker in &mut self.bid_markers {
            marker.draw();
        }

        for view in &mut self.card_views {
            view.draw();
        }

        let transform = super::transform::Transform::default();

        self.score_table.draw(&transform);

        self.bid_panel.draw();
        self.done_exchanging_button.draw(&transform);
        self.next_hand_button.draw(&transform);

        self.message.draw(&transform);

        next_frame().await;
    }
}
