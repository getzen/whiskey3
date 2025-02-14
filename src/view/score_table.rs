use array2d::Array2D;
use macroquad::{
    math::{vec2, Vec2},
    shapes::draw_rectangle,
    text::Font,
};

use crate::game::Game;

use super::{
    animators::TranslationAnimator,
    eventer::Eventer,
    texter::{AlignH, AlignV, Texter},
    transform::Transform,
    view_geom::SCORE_TABLE_POS,
};

pub struct ScoreTable {
    pub visible: bool,
    size: Vec2,
    transform: Transform,
    eventer: Eventer,
    trans_anim: Option<TranslationAnimator>,
    texters: Array2D<Texter>,
}

impl ScoreTable {
    pub fn new(position: Vec2, font: Font) -> Self {
        let size = Vec2::new(400.0, 250.0);
        let mut transform = Transform::from_translation(position);
        transform.size = size;

        let default_texter = Texter::new(
            vec2(10.0, 10.0),
            "----",
            font,
            14,
            AlignH::Center,
            AlignV::Center,
        );
        let mut texters = Array2D::filled_with(default_texter, 12, 3);

        // Create the column headings
        let col_headings = ["", "We", "They"];
        for col in 0..col_headings.len() {
            texters[(0, col)].text = col_headings[col].to_string();
        }

        //  Create the row headings
        let row_headings = [
            "",
            "Taken",
            "Last Trick",
            "Nest",
            "Tricks Taken",
            "Majority",
            "",
            "Total/Bid",
            "Bonus",
            "",
            "Hand",
            "Game/Win",
        ];
        for row in 0..row_headings.len() {
            texters[(row, 0)].text = row_headings[row].to_string();
            texters[(row, 0)].align_h = AlignH::Left;
        }

        // Hide certain rows for this game.
        for column in 0..3 {
            texters[(2, column)].visible = false; // Last Trick
                                                  //texters[(4, column)].visible = false; // Tricks Taken
                                                  //texters[(5, column)].visible = false; // Majority
        }

        let mut eventer = Eventer::new();
        eventer.enabled = false;

        Self {
            visible: false,
            size,
            transform,
            eventer,
            trans_anim: None,
            texters,
        }
    }

    pub fn update_scoring(&mut self, game: &Game) {
        let scoring = &game.scoring;
        let mut row = 1;
        self.texters[(row, 1)].text = scoring.points_taken[0].to_string();
        self.texters[(row, 2)].text = scoring.points_taken[1].to_string();
        row += 1;

        self.texters[(row, 1)].text = scoring.last_trick[0].to_string();
        self.texters[(row, 2)].text = scoring.last_trick[1].to_string();
        row += 1;

        self.texters[(row, 1)].text = scoring.nest[0].to_string();
        self.texters[(row, 2)].text = scoring.nest[1].to_string();
        row += 1;

        self.texters[(row, 1)].text = scoring.trick_count[0].to_string();
        self.texters[(row, 2)].text = scoring.trick_count[1].to_string();
        row += 1;

        self.texters[(row, 1)].text = scoring.majority_tricks[0].to_string();
        self.texters[(row, 2)].text = scoring.majority_tricks[1].to_string();
        row += 1;

        // Dividing line.
        row += 1;

        let total0 = scoring.hand_subtotal[0];
        let total1 = scoring.hand_subtotal[1];
        self.texters[(row, 1)].text = format!("{}/{}", total0, scoring.bid[0]);
        self.texters[(row, 2)].text = format!("{}/{}", total1, scoring.bid[1]);
        row += 1;

        self.texters[(row, 1)].text = scoring.bonus[0].to_string();
        self.texters[(row, 2)].text = scoring.bonus[1].to_string();
        row += 1;

        // Dividing line.
        row += 1;

        self.texters[(row, 1)].text = scoring.hand_final[0].to_string();
        self.texters[(row, 2)].text = scoring.hand_final[1].to_string();
        row += 1;

        self.texters[(row, 1)].text =
            format!("{}/{}", scoring.game[0], game.options.points_to_win_game);
        self.texters[(row, 2)].text =
            format!("{}/{}", scoring.game[1], game.options.points_to_win_game);
    }

    pub fn update(&mut self, time_delta: f32) {
        if let Some(translator) = &mut self.trans_anim {
            self.transform.translation = translator.update(time_delta);
            if translator.completed {
                self.trans_anim = None;
            }
        }
    }

    /// Returns true if visible and transform contains the mouse_pos.
    pub fn process_events(
        &mut self,
        parent_transform: Option<&Transform>,
        mouse_pos: Vec2,
    ) -> bool {
        let transform = match parent_transform {
            Some(parent) => &Transform::combine(parent, &self.transform),
            None => &self.transform,
        };

        let _mouse_over = self.eventer.process_events(transform, mouse_pos);

        if self.eventer.mouse_entered {
            let drop_down_pos = SCORE_TABLE_POS + Vec2::new(0.0, -SCORE_TABLE_POS.y);
            self.trans_anim = Some(TranslationAnimator::new(
                self.transform.translation,
                drop_down_pos,
                400.0,
            ));
        }

        if self.eventer.mouse_exited {
            self.trans_anim = Some(TranslationAnimator::new(
                self.transform.translation,
                SCORE_TABLE_POS,
                400.0,
            ));
        }
        false
    }

    pub fn draw(&mut self, parent_transform: Option<&Transform>) {
        if !self.visible {
            return;
        }

        // Position all the texters that are visible.
        let mut position = vec2(0.0, 27.0);
        let line_spacing = vec2(0.0, 19.0);
        let column_x = [0.0, 130.0, 220.0];

        for row in 0..self.texters.num_rows() {
            if self.texters[(row, 0)].visible {
                for col in 0..self.texters.num_columns() {
                    let pos = position + vec2(column_x[col], 0.0);
                    self.texters[(row, col)].transform.translation = pos;
                }
                position += line_spacing;
            }
        }

        // Set the right size for event checking
        self.transform.size = vec2(self.size.x, position.y);

        let transform = match parent_transform {
            Some(parent) => &Transform::combine(parent, &self.transform),
            None => &self.transform,
        };
        let (pos, _rot) = transform.drawable_position_rotation();

        draw_rectangle(
            pos.x - 14.0,
            pos.y,
            self.size.x,
            position.y,
            macroquad::color::Color::from_rgba(50, 50, 50, 190),
        );

        for texter in self.texters.as_row_major() {
            texter.draw(Some(transform));
        }
    }
}
