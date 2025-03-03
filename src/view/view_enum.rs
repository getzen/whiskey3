use macroquad::math::Vec2;

use super::{
    card_ent::CardEnt, score_table::ScoreTable,
    text_multi::TextMulti, transform::Transform, trump_marker::TrumpMarker,
};

// To get a concrete entity when stored as an enum:
// let entity = self.view_entities.get_mut(&id).unwrap();
// if let ViewEnum::TurnMarker(marker) = entity {
//     marker.foo()
// }
pub enum ViewEnum {
    TrumpMarker(TrumpMarker),
    TextMulti(TextMulti),
    ScoreTable(ScoreTable),
    CardEnt(CardEnt),
}

impl ViewEnum {
    pub fn set_translation(&mut self, translation: Vec2) {
        match self {
            ViewEnum::CardEnt(card) => card.set_translation(translation),
            _ => {
                panic!()
            }
        }
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        match self {
            ViewEnum::CardEnt(card) => card.set_rotation(rotation),
            _ => {
                panic!()
            }
        }
    }

    pub fn update(&mut self, time_delta: f32) {
        match self {
            _ => {}
        }
    }

    pub fn process_mouse(&mut self, mouse_pos: Vec2) -> bool {
        let transform = Transform::default();
        match self {
            ViewEnum::CardEnt(card) => card.process_mouse(mouse_pos),
            _ => false,
        }
    }

    pub fn draw(&mut self) {
        let transform = Transform::default();
        match self {
            ViewEnum::TrumpMarker(marker) => marker.draw(),
            ViewEnum::TextMulti(text_multi) => text_multi.draw(&transform),
            ViewEnum::ScoreTable(table) => table.draw(&transform),
            ViewEnum::CardEnt(card) => card.draw(),
        }
    }
}
