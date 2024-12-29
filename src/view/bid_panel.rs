use macroquad::{
    color::{BLACK, GRAY, GREEN, WHITE},
    math::{vec2, Vec2},
    shapes::{draw_rectangle, draw_rectangle_lines},
};

use super::{button_text::ButtonText, eventer::EventerEvent, texter::Texter, transform::Transform};

pub struct BidPanel {
    pub visible: bool,
    size: Vec2,
    transform: Transform,
    bid_button: ButtonText,
    pass_button: ButtonText,
    // plus_button: ButtonText,
    // minus_button: ButtonText,
    // bid_text: Texter,
}

impl BidPanel {
    pub async fn new(position: Vec2) -> Self {
        let bid_button = ButtonText::new(
            0,
            position + vec2(0.0, 0.0),
            "Bid",
            18,
            Some("Menlo-Bold.ttf"),
            vec2(80.0, 40.0),
        )
        .await;

        let pass_button = ButtonText::new(
            0,
            position + vec2(100.0, 0.0),
            "Pass",
            18,
            Some("Menlo-Bold.ttf"),
            vec2(80.0, 40.0),
        )
        .await;

        Self {
            visible: true,
            size: vec2(250.0, 100.0),
            transform: Transform::new(position, 0.0),
            bid_button,
            pass_button,
        }
    }

    pub fn process_events(&mut self, mouse_pos: &Vec2) -> Option<EventerEvent> {
        if !self.visible {
            return None;
        }

        let event = self.bid_button.process_events(mouse_pos);
        self.pass_button.process_events(mouse_pos);

        None
    }

    pub fn draw(&mut self) {
        if !self.visible {
            return;
        }

        let (mut pos, _rot) = self.transform.combined_pos_rot();
        pos.x -= self.size.x / 2.0;
        pos.y -= self.size.y / 2.0;

        //draw_rectangle(pos.x, pos.y, self.size.x, self.size.y, );

        draw_rectangle_lines(pos.x, pos.y, self.size.x, self.size.y, 4.0, GREEN);

        self.bid_button.draw();
        self.pass_button.draw();
    }
}
