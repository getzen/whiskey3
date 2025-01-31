use array2d::Array2D;
use macroquad::{math::Vec2, shapes::draw_rectangle, text::Font};

use crate::{game::POINTS_TO_WIN, scoring::Scoring};

use super::{
    texter::{AlignH, AlignV, Texter},
    transform::Transform,
};

pub struct ScoreTable {
    pub visible: bool,
    transform: Transform,
    texters: Array2D<Texter>,
}

impl ScoreTable {
    pub fn new(position: Vec2, font: Font) -> Self {
        let transform = Transform::from_translation(position);

        let def_texter = Texter::new(position, "----", font, 14, AlignH::Center, AlignV::Center);

        let mut texters = Array2D::filled_with(def_texter, 10, 3);

        let col_headings = ["", "We", "They"];
        for col in 0..col_headings.len() {
            texters[(0, col)].text = col_headings[col].to_string();
        }

        let row_headings = [
            "",
            "Taken",
            "Last Trick",
            "Nest",
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

        // Assign positions.
        let column_x = [0.0, 130.0, 220.0];
        let line_spacing = 19.0;

        for row in 0..texters.num_rows() {
            for col in 0..texters.num_columns() {
                let mut pos = Vec2::ZERO;

                // Row position
                pos.y += row as f32 * line_spacing;

                // Column position
                pos.x += column_x[col];

                texters[(row, col)].transform.translation = pos;
            }
        }

        Self {
            visible: false,
            transform,
            texters,
        }
    }

    pub fn update(&mut self, scoring: &Scoring) {
        self.texters[(1, 1)].text = scoring.taken[0].to_string();
        self.texters[(1, 2)].text = scoring.taken[1].to_string();

        self.texters[(2, 1)].text = scoring.last_trick[0].to_string();
        self.texters[(2, 2)].text = scoring.last_trick[1].to_string();

        self.texters[(3, 1)].text = scoring.nest[0].to_string();
        self.texters[(3, 2)].text = scoring.nest[1].to_string();

        // Dividing line is row 4.

        let total0 = scoring.taken[0] + scoring.last_trick[0] + scoring.nest[0];
        let total1 = scoring.taken[1] + scoring.last_trick[1] + scoring.nest[1];
        self.texters[(5, 1)].text = format!("{}/{}", total0, scoring.bid[0]);
        self.texters[(5, 2)].text = format!("{}/{}", total1, scoring.bid[1]);

        self.texters[(6, 1)].text = scoring.bonus[0].to_string();
        self.texters[(6, 2)].text = scoring.bonus[1].to_string();

        // Dividing line is row 7.

        self.texters[(8, 1)].text = scoring.hand[0].to_string();
        self.texters[(8, 2)].text = scoring.hand[1].to_string();

        self.texters[(9, 1)].text = format!("{}/{}", scoring.game[0], POINTS_TO_WIN);
        self.texters[(9, 2)].text = format!("{}/{}", scoring.game[1], POINTS_TO_WIN);
    }

    pub fn draw(&self, parent_transform: Option<&Transform>) {
        if !self.visible {
            return;
        }

        let transform = match parent_transform {
            Some(parent) => &Transform::combine(parent, &self.transform),
            None => &self.transform,
        };
        let (pos, _rot) = transform.drawable_position_rotation();

        draw_rectangle(
            pos.x - 14.0,
            0.0,
            400.0,
            210.0,
            macroquad::color::Color::from_rgba(50, 50, 50, 190),
        );

        for texter in self.texters.as_row_major() {
            texter.draw(Some(transform));
        }
    }
}
