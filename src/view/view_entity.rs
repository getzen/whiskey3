use macroquad::math::Vec2;

use super::{bid_marker::BidMarker, card_ent::CardEnt, trump_chooser::TrumpChooser, turn_marker::TurnMarker};

// To get a concrete entity when stored as an enum:
// let entity = self.view_entities.get_mut(&id).unwrap();
// if let ViewEnt::TurnMarker(marker) = entity {
//     marker.foo()
// }
pub enum ViewEnt {
    TurnMarker(TurnMarker),
    //BidMarker(BidMarker),
    TrumpChooser(TrumpChooser),
    CardEnt(CardEnt),
}

impl ViewEnt {
    pub fn set_translation(&mut self, translation: Vec2) {
        match self {
            ViewEnt::TurnMarker(marker) => marker.set_translation(translation),
            ViewEnt::CardEnt(card) => card.set_translation(translation),
            _ => {}
        }
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        match self {
            ViewEnt::CardEnt(card) => card.set_rotation(rotation),
            _ => {}
        }
    }

    pub fn update(&mut self, time_delta: f32) {
        match self {
            _ => {}
        }
    }

    pub fn process_mouse(&mut self, mouse_pos: Vec2) -> bool {
        match self {
            ViewEnt::CardEnt(card) => card.process_mouse(mouse_pos),
            ViewEnt::TrumpChooser(chooser) => chooser.process_mouse(mouse_pos),
            _ => false,
        }
    }

    pub fn draw(&mut self) {
        match self {
            ViewEnt::TurnMarker(marker) => marker.draw(),
            //ViewEnt::BidMarker(marker) => {} //marker.draw(),
            ViewEnt::TrumpChooser(chooser) => chooser.draw(),
            ViewEnt::CardEnt(card) => card.draw(),
        }
    }
}

/*
/// GameEntities are top-level objects. They are stored by Id.
/// They almost always have a Transform component, along with a
/// drawing component such as a Sprite or Text.
pub trait ViewEntity {
    fn as_any(&self) -> &dyn std::any::Any; // simply return 'self'

    fn set_translation(&mut self, _translation: Vec2) {
        //self.transform.translation = _translation;
    }

    fn set_rotation(&mut self, _rotation: f32) {
        //self.transform.rotation = _rotation;
    }

    fn update_eventer(&mut self, _mouse_pos: Vec2) -> bool {
        false
        /* Typical:
        let mouse_over = self.eventer.process_events(&_mouse_pos, &self.transform, &anchor);
        if self.eventer.left_mouse_released {
            if let Some(sender) = SENDER.get() {
                if let Some(action) = &self.action {
                    sender.send(action.clone()).expect("Send error");
                }
            }
        }
        mouse_over
        */
    }

    fn update(&mut self, _time_delta: f32) {}

    fn draw(&mut self) {}
}
*/
