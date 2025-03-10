use std::sync::mpsc::Sender;

use hashbrown::HashMap;
use macroquad::{
    color::Color,
    input::{KeyCode, is_key_released, mouse_position},
    math::{Vec2, vec2},
    text::{Font, load_ttf_font},
    texture::{Texture2D, load_texture},
    window::{clear_background, next_frame},
};

use crate::{
    card::{Card, Id, Suit},
    game::{Game, PlayerAction},
};

use super::{
    rotation_anim::RotationAnimator,
    transform::Transform,
    translation_anim::TranslationAnimator,
    view_entity::{
        bid_marker::BidMarker, bid_panel::BidPanel, button_state::ButtonState, button_text::ButtonText,
        card_ent::CardEnt, score_table::ScoreTable, text::AlignH, text_multi::TextMulti, trump_chooser::TrumpChooser,
        trump_marker::TrumpMarker, turn_marker::TurnMarker, view_entity::ViewEntity,
    },
    view_geom::{
        self, BID_PANEL_POS, DONE_EXCHANGING_BUTTON_POS, MESSAGE_POS, NEXT_HAND_BUTTON_POS, PLAY_CENTER,
        SCORE_TABLE_POS, TRUMP_CHOOSER_POS, ViewGeom, Z, bid_marker_geom,
    },
};

use std::sync::OnceLock;
pub static FONT: OnceLock<Font> = OnceLock::new();

/// ZOrder can be put into a Vec and sorted by z to provide a drawing
/// and event-checking order for the ids.
struct ZOrder {
    id: Id,
    z: Z,
}

pub struct View {
    /*
    To retrieve concrete struct from view_entities:
    let entity = self.view_entities.get_mut(&self.foo_id).unwrap();
    entity.as_any_mut().downcast_mut::<Foo>().unwrap().radius = 50.;
    -- OR --
    if let Some(foo) = entity.as_any_mut().downcast_mut::<Foo>() {
        foo.radius = 50.;
    }
    */
    view_entities: HashMap<Id, Box<dyn ViewEntity>>,
    z_orders: Vec<ZOrder>,
    z_order_needs_update: bool,

    turn_marker: Id,
    bid_markers: Vec<Id>,
    bid_panel: Id,
    trump_chooser: Id,
    trump_marker: Id,
    done_exchanging_button: Id,
    next_hand_button: Id,
    message: Id,
    score_table: Id,

    translation_anims: HashMap<Id, TranslationAnimator>,
    rotation_anims: HashMap<Id, RotationAnimator>,

    sender: Sender<PlayerAction>,
}

impl View {
    pub async fn new(players: usize, sender: Sender<PlayerAction>) -> Self {
        let font = load_ttf_font("./src/assets/Menlo-Bold.ttf").await.unwrap();
        FONT.set(font.clone()).expect("Error setting FONT.");

        let mut id = 100; // above cards
        let mut view_entities = HashMap::<Id, Box<dyn ViewEntity>>::new();
        let mut z_orders = Vec::new();

        let turn_marker = id;
        let entity = View::create_turn_marker().await;
        view_entities.insert(id, entity);
        z_orders.push(ZOrder { id, z: 255 });
        id += 1;

        let mut bid_markers = Vec::new();
        for p in 0..players {
            let entity = View::create_bid_marker(p, players);
            view_entities.insert(id, entity);
            z_orders.push(ZOrder { id, z: 0 });
            bid_markers.push(id);
            id += 1;
        }

        let bid_panel = id;
        let entity = View::create_bid_panel();
        view_entities.insert(id, entity);
        z_orders.push(ZOrder { id, z: 0 });
        id += 1;

        let trump_chooser = id;
        let entity = View::create_trump_chooser().await;
        view_entities.insert(id, entity);
        z_orders.push(ZOrder { id, z: 0 });
        id += 1;

        let trump_marker = id;
        let entity = View::create_trump_marker();
        view_entities.insert(id, entity);
        z_orders.push(ZOrder { id, z: 0 });
        id += 1;

        let done_exchanging_button = id;
        let entity = View::create_done_exchanging_button();
        view_entities.insert(id, entity);
        z_orders.push(ZOrder { id, z: 0 });
        id += 1;

        let next_hand_button = id;
        let entity = View::create_next_hand_button();
        view_entities.insert(id, entity);
        z_orders.push(ZOrder { id, z: 0 });
        id += 1;

        let message = id;
        let entity = View::create_message();
        view_entities.insert(id, entity);
        z_orders.push(ZOrder { id, z: 0 });
        id += 1;

        let score_table = id;
        let entity = View::create_score_table();
        view_entities.insert(id, entity);
        z_orders.push(ZOrder { id, z: 255 });
        //id += 1;

        Self {
            view_entities,
            z_orders,
            z_order_needs_update: true,

            turn_marker,
            bid_markers,
            bid_panel,
            trump_chooser,
            trump_marker,
            done_exchanging_button,
            next_hand_button,
            message,
            score_table,

            translation_anims: HashMap::new(),
            rotation_anims: HashMap::new(),

            sender,
        }
    }

    async fn create_turn_marker() -> Box<dyn ViewEntity> {
        let tex = load_texture("src/assets/circle.png").await.unwrap();
        let mut entity = TurnMarker::new(tex, vec2(20.0, 20.0));
        entity.transform.translation = PLAY_CENTER;
        Box::new(entity)
    }

    fn create_bid_marker(p: usize, players: usize) -> Box<dyn ViewEntity> {
        let geom = bid_marker_geom(p, players);
        let entity = BidMarker::new(geom.pos);
        Box::new(entity)
    }

    fn create_bid_panel() -> Box<dyn ViewEntity> {
        let entity = BidPanel::new(BID_PANEL_POS, 0, 0);
        Box::new(entity)
    }

    async fn create_trump_chooser() -> Box<dyn ViewEntity> {
        let entity = TrumpChooser::new(TRUMP_CHOOSER_POS).await;
        Box::new(entity)
    }

    fn create_trump_marker() -> Box<dyn ViewEntity> {
        let entity = TrumpMarker::new(PLAY_CENTER);
        Box::new(entity)
    }

    fn create_done_exchanging_button() -> Box<dyn ViewEntity> {
        let font = FONT.get().unwrap().clone();
        let mut entity = ButtonText::new(DONE_EXCHANGING_BUTTON_POS, "Done", font, 16, vec2(80.0, 40.0));
        entity.click_action = Some(PlayerAction::DoneExchanging);
        entity.visible = false;
        Box::new(entity)
    }

    fn create_next_hand_button() -> Box<dyn ViewEntity> {
        let font = FONT.get().unwrap().clone();
        let mut entity = ButtonText::new(NEXT_HAND_BUTTON_POS, "Next Hand", font, 16, vec2(120.0, 40.0));
        entity.click_action = Some(PlayerAction::NextHand);
        entity.visible = false;
        Box::new(entity)
    }

    fn create_message() -> Box<dyn ViewEntity> {
        let entity = TextMulti::new(MESSAGE_POS);
        Box::new(entity)
    }

    fn create_score_table() -> Box<dyn ViewEntity> {
        let entity = ScoreTable::new(SCORE_TABLE_POS);
        Box::new(entity)
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
        let entity = Box::new(entity);
        self.view_entities.insert(card.id, entity);
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
        let transform = Transform::new();

        for z_order in self.z_orders.iter().rev() {
            let entity = self.view_entities.get_mut(&z_order.id).unwrap();
            let done = entity.process_mouse(&mouse_pos, &transform);
            if done {
                break;
            }
        }
    }

    pub fn update(&mut self, time_delta: f32) {
        for z_order in &self.z_orders {
            let entity = self.view_entities.get_mut(&z_order.id).unwrap();
            entity.update(time_delta);
        }

        if self.z_order_needs_update {
            self.sort_z_orders();
        }

        // Update translation_anims and transforms.
        for (id, anim) in &mut self.translation_anims {
            // println!("anim id: {}", id);
            anim.update(time_delta);
            if let Some(entity) = self.view_entities.get_mut(id) {
                entity.set_translation(anim.current);
            }
        }
        self.translation_anims.retain(|_k, v| !v.completed);

        // Update rotation_anims and transforms.
        for (id, anim) in &mut self.rotation_anims {
            anim.update(time_delta);
            if let Some(entity) = self.view_entities.get_mut(id) {
                entity.set_rotation(anim.current);
            }
        }
        self.rotation_anims.retain(|_k, v| !v.completed);
    }

    pub fn update_info(&mut self, game: &Game) {
        let entity = self.view_entities.get_mut(&self.score_table).unwrap();
        if let Some(score_table) = entity.as_any_mut().downcast_mut::<ScoreTable>() {
            score_table.visible = true;
            score_table.update_scoring(&game);
        }

        let entity = self.view_entities.get_mut(&self.turn_marker).unwrap();
        if let Some(turn_marker) = entity.as_any_mut().downcast_mut::<TurnMarker>() {
            turn_marker.visible = true;
            let geom = view_geom::turn_marker_geom(game.active, game.options.players);
            turn_marker.transform.translation = geom.pos;
        }
    }

    pub fn update_message(&mut self, texts: &[&str]) {
        let entity = self.view_entities.get_mut(&self.message).unwrap();
        if let Some(message) = entity.as_any_mut().downcast_mut::<TextMulti>() {
            message.clear_lines();
            let font = FONT.get().unwrap();
            for text in texts {
                message.add_line(text, font.clone(), 18, AlignH::Center, 20.0);
            }
        }
    }

    fn get_card_ent_mut(&mut self, id: Id) -> &mut CardEnt {
        // don't make id: Id --> id: &Id
        let entity = self.view_entities.get_mut(&id).unwrap();
        entity.as_any_mut().downcast_mut::<CardEnt>().unwrap()
    }

    fn update_card_ent(&mut self, id: Id, geom: ViewGeom, face_up: bool) {
        let entity = self.view_entities.get_mut(&id).unwrap();
        if let Some(card_ent) = entity.as_any_mut().downcast_mut::<CardEnt>() {
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
            let id = self.bid_markers[p];
            let entity = self.view_entities.get_mut(&id).unwrap();
            if let Some(bid_marker) = entity.as_any_mut().downcast_mut::<BidMarker>() {
                bid_marker.update_with_bid(opt_bid.clone());
            }
        }
    }

    pub fn hide_bids_except_maker(&mut self, game: &Game) {
        for p in 0..game.options.players {
            if game.maker.unwrap() == p {
                continue;
            }
            let id = self.bid_markers[p];
            let entity = self.view_entities.get_mut(&id).unwrap();
            if let Some(bid_marker) = entity.as_any_mut().downcast_mut::<BidMarker>() {
                bid_marker.visible = false;
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
            let card_ent = self.get_card_ent_mut(card.id);
            card_ent.dimmed = false;
            card_ent.action = None;
        }
    }

    pub fn get_human_exchanges(&mut self, game: &Game) {
        let count = game.options.exchange_size - game.exchange.len();
        let text = format!("Discard {} cards", count);
        self.update_message(&[&text]);
        self.show_done_exchanging_button(false);
    }

    pub fn show_done_exchanging_button(&mut self, enabled: bool) {
        let entity = self.view_entities.get_mut(&self.done_exchanging_button).unwrap();
        if let Some(button) = entity.as_any_mut().downcast_mut::<ButtonText>() {
            button.visible = true;
            button.state = match enabled {
                true => ButtonState::Normal,
                false => ButtonState::Disabled,
            }
        }
    }

    pub fn hide_done_exchanging_button(&mut self) {
        let entity = self.view_entities.get_mut(&self.done_exchanging_button).unwrap();
        entity.as_any_mut().downcast_mut::<ButtonText>().unwrap().visible = false;
    }

    pub fn set_discardable_hand_cards(&mut self, game: &Game) {
        let maker = game.maker.unwrap();
        let hand = &game.hands[maker];
        for card in hand {
            let card_ent = self.get_card_ent_mut(card.id);
            card_ent.dimmed = !card.eligible;
            if card.eligible {
                card_ent.action = Some(PlayerAction::Exchange(card.id));
            }
        }
    }

    pub fn show_trump_chooser(&mut self, visible: bool) {
        let entity = self.view_entities.get_mut(&self.trump_chooser).unwrap();
        entity.as_any_mut().downcast_mut::<TrumpChooser>().unwrap().visible = visible;

        if visible {
            self.update_message(&["Select trump suit."]);
        } else {
            self.update_message(&[""]);
        }
    }

    pub fn show_trump_marker(&mut self, visible: bool) {
        let entity = self.view_entities.get_mut(&self.trump_marker).unwrap();
        entity.as_any_mut().downcast_mut::<TrumpMarker>().unwrap().visible = visible;
    }

    pub async fn set_trump_suit(&mut self, suit: Option<Suit>) {
        let entity = self.view_entities.get_mut(&self.trump_marker).unwrap();
        entity
            .as_any_mut()
            .downcast_mut::<TrumpMarker>()
            .unwrap()
            .set_suit(suit)
            .await;
    }

    pub fn set_playable_hand_cards(&mut self, game: &Game) {
        let hand = &game.hands[game.active];

        for card in hand {
            let card_ent = self.get_card_ent_mut(card.id);
            card_ent.dimmed = !card.eligible;
            if card.eligible {
                card_ent.action = Some(PlayerAction::PlayCard(card.id));
            }
        }
    }

    pub fn show_next_hand_button(&mut self, visible: bool) {
        let entity = self.view_entities.get_mut(&self.next_hand_button).unwrap();
        entity.as_any_mut().downcast_mut::<ButtonText>().unwrap().visible = visible;
    }

    pub fn get_human_bid(&mut self, game: &Game) {
        // Hide bid marker for human.
        self.hide_bid_marker(game.active);

        let entity = self.view_entities.get_mut(&self.bid_panel).unwrap();
        if let Some(panel) = entity.as_any_mut().downcast_mut::<BidPanel>() {
            panel.min_bid = game.min_current_bid();
            panel.update_bid_amount(game.min_current_bid());
            panel.max_bid = game.options.max_bid;
            panel.visible = true;
        }
        self.update_message(&["Your bid."]);
    }

    pub fn end_human_bid(&mut self, _game: &Game) {
        let entity = self.view_entities.get_mut(&self.bid_panel).unwrap();
        entity.as_any_mut().downcast_mut::<BidPanel>().unwrap().visible = false;
    }

    pub fn hide_bid_marker(&mut self, bidder: usize) {
        let id = self.bid_markers[bidder];
        let entity = self.view_entities.get_mut(&id).unwrap();
        entity.as_any_mut().downcast_mut::<BidMarker>().unwrap().visible = false;
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

        let transform = Transform::new();
        for z_order in &self.z_orders {
            let entity = self.view_entities.get_mut(&z_order.id).unwrap();
            entity.draw(&transform);
        }

        // let vertices = vec![vec2(100., 100.), vec2(200., 200.), vec2(180., 300.), vec2(140.0, 150.)];
        // let color = match polygon_contains_point(&vertices, mouse_position().into()) {
        //     true => macroquad::color::BLUE,
        //     false => macroquad::color::GREEN,
        // };
        // super::utility_graphics::draw_polygon_lines(&vertices, 1.0, color);

        next_frame().await;
    }
}
