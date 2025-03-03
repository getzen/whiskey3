use std::{cell::RefCell, rc::Rc, sync::mpsc::Sender};

use hashbrown::HashMap;
use macroquad::{
    color::{BLUE, Color},
    input::{KeyCode, is_key_released, mouse_position},
    math::{Vec2, vec2},
    text::{Font, load_ttf_font},
    texture::{Texture2D, load_texture},
    window::{clear_background, next_frame},
};

use crate::{
    card::{Card, Id, Suit},
    game::{Game, PlayerAction},
    view::{button_state::ButtonState, card_ent::CardEnt},
};

use super::{
    bid_panel::BidPanel, rotation_anim::RotationAnimator, score_table::ScoreTable, text_multi::TextMulti, transform::Transform, translation_anim::TranslationAnimator, trump_chooser::TrumpChooser, trump_marker::TrumpMarker, view_entity::{bid_marker::BidMarker, button_text::ButtonText, circle::Circle, text::AlignH, turn_marker::TurnMarker, view_entity::ViewEntity}, view_geom::{
        self, bid_marker_geom, ViewGeom, BID_PANEL_POS, DONE_EXCHANGING_BUTTON_POS, MESSAGE_POS, NEXT_HAND_BUTTON_POS, PLAY_CENTER, SCORE_TABLE_POS, TRUMP_CHOOSER_POS, Z
    }
};

use super::view_enum::ViewEnum;

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
    view_enums: HashMap<Id, ViewEnum>,
    z_orders: Vec<ZOrder>,
    z_order_needs_update: bool,
    translation_anims: HashMap<Id, TranslationAnimator>,
    rotation_anims: HashMap<Id, RotationAnimator>,
   
    bid_panel: Id,
    trump_chooser: Id,
    trump_marker: Id,
    done_exchanging_button: Id,
    next_hand_button: Id,
    message: Id,
    score_table: Id,

    sender: Sender<PlayerAction>,

    view_entities: HashMap<Id, Rc<RefCell<dyn ViewEntity>>>,
    turn_marker: Rc<RefCell<TurnMarker>>,
    bid_markers: Vec<Rc<RefCell<BidMarker>>>,
    test_circle: Rc<RefCell<Circle>>,
}

impl View {
    pub async fn new(sender: Sender<PlayerAction>) -> Self {
        let font = load_ttf_font("./src/assets/Menlo-Bold.ttf").await.unwrap();
        FONT.set(font.clone()).expect("Error setting FONT.");

        let players = 4;

        ///////////
        let id = 0;
        let mut view_entities = HashMap::<Id, Rc<RefCell<dyn ViewEntity>>>::new();
        let mut z_orders = Vec::new();

        let (id, turn_marker) = View::create_turn_marker(id).await;
        view_entities.insert(id, turn_marker.clone());
        z_orders.push(ZOrder { id, z: 255 });

        let mut bid_markers = Vec::new();
        for p in 0..players {
            let (id, marker) = View::create_bid_marker(id, p, players);
            view_entities.insert(id, marker.clone());
            z_orders.push(ZOrder { id, z: 0 });
            bid_markers.push(marker);
        }



        let (id, test_circle) = View::create_test_circle(id);
        view_entities.insert(id, test_circle.clone());
        

        println!("id: {}", id);
        ///////////

        Self {
            // Set id high enough so that we don't step on the card ids.
            id: 100,
            view_enums: HashMap::<Id, ViewEnum>::new(),
            z_orders: Vec::new(),
            z_order_needs_update: false,
            translation_anims: HashMap::new(),
            rotation_anims: HashMap::new(),

            // These id values are assigned in setup().
            
            bid_panel: 0,
            trump_chooser: 0,
            trump_marker: 0,
            done_exchanging_button: 0,
            next_hand_button: 0,
            message: 0,
            score_table: 0,

            sender,

            view_entities,
            turn_marker,
            bid_markers,
            test_circle,
        }
    }

    fn create_test_circle(mut id: Id) -> (Id, Rc<RefCell<Circle>>) {
        id += 1;
        let mut ent = Circle::new(90.0, None, Some(BLUE), 2.0);
        ent.transform.translation = vec2(100., 100.);
        let ent = Rc::new(RefCell::new(ent));
        (id, ent)
    }

    fn next_id(&mut self) -> Id {
        self.id += 1;
        self.id
    }

    pub async fn setup(&mut self, players: usize) {
        // Cards are created from Controller call in GameAction::Setup.
        
        self.bid_panel = self.create_bid_panel();
        self.trump_chooser = self.create_trump_chooser().await;
        self.trump_marker = self.create_trump_marker();
        self.done_exchanging_button = self.create_done_exchanging_button();
        self.next_hand_button = self.create_next_hand_button();
        self.message = self.create_message();
        self.score_table = self.create_score_table();
    }

    async fn create_turn_marker(mut id: Id) -> (Id, Rc<RefCell<TurnMarker>>) {
        id += 1;
        let tex = load_texture("src/assets/circle.png").await.unwrap();
        let mut entity = TurnMarker::new(tex, vec2(20.0, 20.0));
        entity.transform.translation = PLAY_CENTER;
        let entity =  Rc::new(RefCell::new(entity));
        (id, entity)
    }

    fn create_bid_marker(mut id: Id, p: usize, players: usize) -> (Id, Rc<RefCell<BidMarker>>) {
        id += 1;
        let geom = bid_marker_geom(p, players);
        let entity = BidMarker::new(geom.pos);
        let entity = Rc::new(RefCell::new(entity));
        (id, entity)
    }

    fn create_bid_panel(&mut self) -> Id {
        let id = self.next_id();
        let entity = BidPanel::new(BID_PANEL_POS, 0, 0);
        self.view_enums.insert(id, ViewEnum::BidPanel(entity));
        self.z_orders.push(ZOrder { id, z: 0 });
        id
    }

    async fn create_trump_chooser(&mut self) -> Id {
        let id = self.next_id();
        let entity = TrumpChooser::new(TRUMP_CHOOSER_POS).await;
        self.view_enums.insert(id, ViewEnum::TrumpChooser(entity));
        self.z_orders.push(ZOrder { id, z: 0 });
        id
    }

    fn create_trump_marker(&mut self) -> Id {
        let id = self.next_id();
        let entity = TrumpMarker::new(PLAY_CENTER);
        self.view_enums.insert(id, ViewEnum::TrumpMarker(entity));
        self.z_orders.push(ZOrder { id, z: 0 });
        id
    }

    fn create_done_exchanging_button(&mut self) -> Id {
        let id = self.next_id();
        let font = FONT.get().unwrap().clone();
        let mut entity = ButtonText::new(DONE_EXCHANGING_BUTTON_POS, "Done", font, 16, vec2(80.0, 40.0));
        entity.click_action = Some(PlayerAction::DoneExchanging);
        entity.visible = false;
        self.view_enums.insert(id, ViewEnum::ButtonText(entity));
        self.z_orders.push(ZOrder { id, z: 0 });
        id
    }

    fn create_next_hand_button(&mut self) -> Id {
        let id = self.next_id();
        let font = FONT.get().unwrap().clone();
        let mut entity = ButtonText::new(NEXT_HAND_BUTTON_POS, "Next Hand", font, 16, vec2(120.0, 40.0));
        entity.click_action = Some(PlayerAction::NextHand);
        entity.visible = false;
        self.view_enums.insert(id, ViewEnum::ButtonText(entity));
        self.z_orders.push(ZOrder { id, z: 0 });
        id
    }

    fn create_message(&mut self) -> Id {
        let id = self.next_id();
        let entity = TextMulti::new(MESSAGE_POS);
        self.view_enums.insert(id, ViewEnum::TextMulti(entity));
        self.z_orders.push(ZOrder { id, z: 0 });
        id
    }

    fn create_score_table(&mut self) -> Id {
        let id = self.next_id();
        let font = FONT.get().unwrap().clone();
        let entity = ScoreTable::new(SCORE_TABLE_POS, font);
        self.view_enums.insert(id, ViewEnum::ScoreTable(entity));
        self.z_orders.push(ZOrder { id, z: 250 });
        id
    }

    async fn texture_for(&self, card: &Card) -> Texture2D {
        let path = format!("src/assets/cards/{}.png", card.file_string());
        load_texture(&path).await.unwrap()
    }

    pub async fn create_card_view(&mut self, card: &Card) {
        let back = load_texture("src/assets/cards/back.png").await.unwrap();
        let face = self.texture_for(card).await;

        let mut entity = CardEnt::new(face, back.clone(), card.points);
        entity.transform.translation = view_geom::PLAY_CENTER;
        self.view_enums.insert(card.id, ViewEnum::CardEnt(entity));
        self.z_orders.push(ZOrder { id: card.id, z: 0 });
    }

    fn set_z_order(&mut self, id: Id, z: Z) {
        if let Some(z_order) = self.z_orders.iter_mut().find(|z| z.id == id) {
            z_order.z = z;
        }
        self.z_order_needs_update = true;
    }

    fn sort_z_orders(&mut self) {
        self.z_orders.sort_by_key(|f| f.z);
        self.z_order_needs_update = false;
    }

    pub fn check_events(&mut self) {
        // Key presses
        if is_key_released(KeyCode::Escape) {
            self.sender.send(PlayerAction::ShouldExit).expect("Send error");
        }

        let mouse_pos: Vec2 = mouse_position().into();

        /////
        let transform = Transform::new();
        for (_id, ent) in &self.view_entities {
            ent.borrow_mut().process_mouse(&mouse_pos, &transform);
        }
        /////

        for z_order in self.z_orders.iter().rev() {
            let entity = self.view_enums.get_mut(&z_order.id).unwrap();
            let done = entity.process_mouse(mouse_pos);
            if done {
                break;
            }
        }
    }

    pub fn update(&mut self, time_delta: f32) {
        /////
        for (_id, ent) in &self.view_entities {
            ent.borrow_mut().update(time_delta);
        }
        self.test_circle.borrow_mut().stroke_width = 1.0;
        /////

        for z_order in &self.z_orders {
            let entity = self.view_enums.get_mut(&z_order.id).unwrap();
            entity.update(time_delta);
        }

        if self.z_order_needs_update {
            self.sort_z_orders();
        }

        // Update translation_anims and transforms.
        for (id, anim) in &mut self.translation_anims {
            anim.update(time_delta);
            if let Some(ent) = self.view_enums.get_mut(id) {
                ent.set_translation(anim.current);
            }
        }
        self.translation_anims.retain(|_k, v| !v.completed);

        // Update rotation_anims and transforms.
        for (id, anim) in &mut self.rotation_anims {
            anim.update(time_delta);
            if let Some(ent) = self.view_enums.get_mut(id) {
                ent.set_rotation(anim.current);
            }
        }
        self.rotation_anims.retain(|_k, v| !v.completed);
    }

    pub fn update_info(&mut self, game: &Game) {
        let entity = self.view_enums.get_mut(&self.score_table).unwrap();
        if let ViewEnum::ScoreTable(table) = entity {
            table.visible = true;
            table.update_scoring(&game);
        }

        let entity = self.view_enums.get_mut(&self.turn_marker).unwrap();
        if let ViewEnum::TurnMarker(marker) = entity {
            marker.visible = true;
            let geom = view_geom::turn_marker_geom(game.active, game.options.players);
            marker.transform.translation = geom.pos;
        }
    }

    pub fn update_message(&mut self, texts: &[&str]) {
        let entity = self.view_enums.get_mut(&self.message).unwrap();
        if let ViewEnum::TextMulti(text_multi) = entity {
            text_multi.clear_lines();
            let font = FONT.get().unwrap();
            for text in texts {
                text_multi.add_line(text, font.clone(), 18, AlignH::Center, 20.0);
            }
        }
    }

    fn update_card_ent(&mut self, id: Id, geom: ViewGeom, face_up: bool) {
        let entity = self.view_enums.get_mut(&id).unwrap();
        if let ViewEnum::CardEnt(card_ent) = entity {
            let start = card_ent.transform.translation;
            let end = geom.pos;
            let trans_anim = TranslationAnimator::new(start, end, view_geom::CARD_SPEED);
            self.translation_anims.insert(id, trans_anim);

            let start = card_ent.transform.rotation;
            let end = geom.rot;
            let rot_anim = RotationAnimator::new(start, end, view_geom::ROT_SPEED);
            self.rotation_anims.insert(id, rot_anim);

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
            let id = self.bid_marker_0 + p as Id;
            let entity = self.view_enums.get_mut(&id).unwrap();
            if let ViewEnum::BidMarker(marker) = entity {
                marker.update_with_bid(opt_bid.clone());
            }
        }
    }

    pub fn hide_bids_except_maker(&mut self, game: &Game) {
        for p in 0..game.options.players {
            if game.maker.unwrap() == p {
                continue;
            }
            let id = self.bid_marker_0 + p as Id;
            let entity = self.view_enums.get_mut(&id).unwrap();
            if let ViewEnum::BidMarker(marker) = entity {
                marker.visible = false;
            }
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
            let entity = self.view_enums.get_mut(&card.id).unwrap();
            if let ViewEnum::CardEnt(card_ent) = entity {
                card_ent.dimmed = false;
                card_ent.action = None;
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
        let entity = self.view_enums.get_mut(&self.done_exchanging_button).unwrap();
        if let ViewEnum::ButtonText(button) = entity {
            button.visible = true;
            button.state = match enabled {
                true => ButtonState::Normal,
                false => ButtonState::Disabled,
            }
        }
    }

    pub fn hide_done_exchanging_button(&mut self) {
        let entity = self.view_enums.get_mut(&self.done_exchanging_button).unwrap();
        if let ViewEnum::ButtonText(button) = entity {
            button.visible = false;
        }
    }

    pub fn set_discardable_hand_cards(&mut self, game: &Game) {
        let maker = game.maker.unwrap();
        let hand = &game.hands[maker];
        for card in hand {
            let entity = self.view_enums.get_mut(&card.id).unwrap();
            if let ViewEnum::CardEnt(card_ent) = entity {
                card_ent.dimmed = !card.eligible;
                if card.eligible {
                    card_ent.action = Some(PlayerAction::Exchange(card.id));
                }
            }
        }
    }

    pub fn show_trump_chooser(&mut self, visible: bool) {
        let entity = self.view_enums.get_mut(&self.trump_chooser).unwrap();
        if let ViewEnum::TrumpChooser(chooser) = entity {
            chooser.visible = visible
        }

        if visible {
            self.update_message(&["Select trump suit."]);
        } else {
            self.update_message(&[""]);
        }
    }

    pub fn show_trump_marker(&mut self, visible: bool) {
        let entity = self.view_enums.get_mut(&self.trump_marker).unwrap();
        if let ViewEnum::TrumpMarker(marker) = entity {
            marker.visible = visible
        }
    }

    pub async fn set_trump_suit(&mut self, suit: Option<Suit>) {
        let entity = self.view_enums.get_mut(&self.trump_marker).unwrap();
        if let ViewEnum::TrumpMarker(marker) = entity {
            marker.set_suit(suit).await
        }
    }

    pub fn set_playable_hand_cards(&mut self, game: &Game) {
        let hand = &game.hands[game.active];

        for card in hand {
            let entity = self.view_enums.get_mut(&card.id).unwrap();
            if let ViewEnum::CardEnt(card_ent) = entity {
                card_ent.dimmed = !card.eligible;
                if card.eligible {
                    card_ent.action = Some(PlayerAction::PlayCard(card.id));
                }
            }
        }
    }

    pub fn show_next_hand_button(&mut self, visible: bool) {
        let entity = self.view_enums.get_mut(&self.next_hand_button).unwrap();
        if let ViewEnum::ButtonText(button) = entity {
            button.visible = visible
        }
    }

    pub fn get_human_bid(&mut self, game: &Game) {
        // Hide bid marker for human.
        self.hide_bid_marker(game.active);

        let entity = self.view_enums.get_mut(&self.bid_panel).unwrap();
        if let ViewEnum::BidPanel(panel) = entity {
            panel.min_bid = game.min_current_bid();
            panel.update_bid_amount(game.min_current_bid());
            panel.max_bid = game.options.max_bid;
            panel.visible = true;
        }
        self.update_message(&["Your bid."]);
    }

    pub fn end_human_bid(&mut self, _game: &Game) {
        let entity = self.view_enums.get_mut(&self.bid_panel).unwrap();
        if let ViewEnum::BidPanel(panel) = entity {
            panel.visible = false;
        }
    }

    pub fn hide_bid_marker(&mut self, bidder: usize) {
        let id = self.bid_marker_0 + bidder as Id;
        let entity = self.view_enums.get_mut(&id).unwrap();
        if let ViewEnum::BidMarker(marker) = entity {
            marker.visible = false;
        }
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

        /////
        let transform = Transform::new();
        for (_id, ent) in &self.view_entities {
            ent.borrow_mut().draw(&transform);
        }
        /////

        for z_order in &self.z_orders {
            let entity = self.view_enums.get_mut(&z_order.id).unwrap();
            entity.draw();
        }
        next_frame().await;
    }
}
