use array2d::Array2D;
use macroquad::{
    math::{Vec2, vec2},
    shapes::draw_rectangle,
    text::Font,
};

use crate::game::Game;

use super::{
    animators::TranslationAnimator,
    eventer_old::Eventer,
    texter::{AlignH, AlignV, Texter},
    transform_old::Transform,
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
        let default_texter = Texter::new(Vec2::ZERO, "-", font, 14, AlignH::Center, AlignV::Center);
        let mut texters = Array2D::filled_with(default_texter, 4, 9);

        let column_widths = vec![50, 75, 60, 75, 75, 75, 75, 80, 66, 75];
        let row_heights = vec![20, 20, 20, 20];
        let starting_pos = vec2(0.0, 0.0);

        let col_headings0 = ["", "Taken", "#", "Majority", "Nest", "Total", "Slam", "Hand", "Game"];
        let col_headings1 = ["", "Pts", "Tricks", "Bonus", "Pts", "/ Bid", "Bonus", "Score", "/ Win"];
        for i in 0..col_headings0.len() {
            texters[(0, i)].text = col_headings0[i].to_string();
            texters[(1, i)].text = col_headings1[i].to_string();
        }

        let row_headings = ["", "", "We", "They"];
        for row in 0..row_headings.len() {
            texters[(row, 0)].text = row_headings[row].to_string();
        }

        // Position all the texters and determine the overall size.
        let mut pos = starting_pos;
        let mut size = Vec2::ZERO;
        for row in 0..texters.num_rows() {
            pos.y += row_heights[row] as f32;
            for col in 0..texters.num_columns() {
                pos.x += column_widths[col] as f32;
                texters[(row, col)].transform.translation = pos;
            }
            size.x = pos.x;
            pos.x = starting_pos.x;
        }
        size.y = pos.y;
        // Expand the size to give a margin.
        size += vec2(40.0, 20.0);

        let mut transform = Transform::from_translation(position);
        transform.size = size;

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
        let mut col = 1;
        self.texters[(2, col)].text = scoring.points_taken[0].to_string();
        self.texters[(3, col)].text = scoring.points_taken[1].to_string();
        col += 1;

        self.texters[(2, col)].text = scoring.trick_count[0].to_string();
        self.texters[(3, col)].text = scoring.trick_count[1].to_string();
        col += 1;

        self.texters[(2, col)].text = scoring.majority_bonus[0].to_string();
        self.texters[(3, col)].text = scoring.majority_bonus[1].to_string();
        col += 1;

        self.texters[(2, col)].text = scoring.nest[0].to_string();
        self.texters[(3, col)].text = scoring.nest[1].to_string();
        col += 1;

        let total0 = scoring.hand_subtotal[0];
        let total1 = scoring.hand_subtotal[1];
        self.texters[(2, col)].text = format!("{}/{}", total0, scoring.bid[0]);
        self.texters[(3, col)].text = format!("{}/{}", total1, scoring.bid[1]);
        col += 1;

        self.texters[(2, col)].text = scoring.slam_bonus[0].to_string();
        self.texters[(3, col)].text = scoring.slam_bonus[1].to_string();
        col += 1;

        self.texters[(2, col)].text = scoring.hand_final[0].to_string();
        self.texters[(3, col)].text = scoring.hand_final[1].to_string();
        col += 1;

        self.texters[(2, col)].text = format!("{}/{}", scoring.game[0], game.options.points_to_win_game);
        self.texters[(3, col)].text = format!("{}/{}", scoring.game[1], game.options.points_to_win_game);
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
    pub fn process_events(&mut self, parent_transform: Option<&Transform>, mouse_pos: Vec2) -> bool {
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

        let transform = match parent_transform {
            Some(parent) => &Transform::combine(parent, &self.transform),
            None => &self.transform,
        };
        let (pos, _rot) = transform.drawable_position_rotation();

        draw_rectangle(
            pos.x,
            pos.y,
            self.size.x,
            self.size.y,
            macroquad::color::Color::from_rgba(50, 50, 50, 190),
        );

        for texter in self.texters.as_row_major() {
            texter.draw(Some(transform));
        }
    }
}
