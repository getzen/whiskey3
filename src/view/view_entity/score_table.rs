use macroquad::{
    math::{Vec2, vec2},
    shapes::draw_rectangle,
};

use crate::{
    game::Game,
    view::{
        mouse_state::MouseState, transform::Transform, translation_anim::TranslationAnimator,
        utility_graphics::rect_contains_point, view::FONT, view_geom::SCORE_TABLE_POS,
    },
};

use super::{
    text::{AlignH, AlignV, Text},
    view_entity::ViewEntity,
};

pub struct ScoreTable {
    pub visible: bool,
    rect_size: Vec2,
    transform: Transform,
    mouse_state: MouseState,
    trans_anim: Option<TranslationAnimator>,
    texts: Vec<Text>,
}

impl ScoreTable {
    pub fn new(position: Vec2) -> Self {

        //let position = vec2(-450.0, 10.0);

        let mut inset_position = Vec2::new(10.0, 10.0);
        let row_height = 20.0;
        let font = FONT.get().unwrap().clone();

        let mut default_text = Text::new(Vec2::ZERO, "", font.clone(), 14);
        default_text.align_h = AlignH::Left;
        default_text.align_v = AlignV::Top;

        let mut texts = Vec::new();

        for _row in 0..3 {
            let mut text = default_text.clone();
            text.transform.translation = inset_position;
            inset_position.y += row_height;
            texts.push(text);
        }

        let rect_size = vec2(555.0, 70.0);

        Self {
            visible: false,
            rect_size,
            transform: Transform::from_translation(position),
            mouse_state: MouseState::new(),
            trans_anim: None,
            texts,
        }
    }

    pub fn update_scoring(&mut self, game: &Game) {
        let scoring = &game.scoring;
        let row_labels = ["We", "They"];

        // There is no way to save the format string "{:<5}..." to a variable and use it with format!().
        // I couldn't find a crate that does it and can handle centering {:^7} commands.

        for i in 0..3 {
            let text = match i {
                0 => format!(
                    "{:<5}{:^8}{:^7}{:^14}{:^12}{:^7}{:^12}",
                    "", "Taken", "Nest", "Most Tricks", "Total/Bid", "Hand", "Total/Win"
                ),
                _ => {
                    let p = i - 1;
                    let subtotal_bid = format!("{}/{}", scoring.hand_subtotal[p], scoring.bid[p]);
                    let total_game = format!("{}/{}", scoring.game[p], game.options.points_to_win_game);
                    format!(
                        "{:<5}{:^8}{:^7}{:^14}{:^12}{:^7}{:^12}",
                        row_labels[p],
                        scoring.points_taken[p],
                        scoring.nest[p],
                        scoring.majority_bonus[p],
                        subtotal_bid,
                        scoring.hand_final[p],
                        total_game
                    )
                }
            };
            self.texts[i].text = text;
        }
    }
}

impl ViewEntity for ScoreTable {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn update(&mut self, time_delta: f32) {
        if let Some(translator) = &mut self.trans_anim {
            self.transform.translation = translator.update(time_delta);
            if translator.completed {
                self.trans_anim = None;
            }
        }
    }

    fn contains_point(&mut self, point: &Vec2, parent_transform: &Transform) -> bool {
        let transform = *parent_transform * self.transform;
        let local_pt = transform.convert_point_to_local(point);
        rect_contains_point(Vec2::ZERO, self.rect_size, local_pt)
    }

    fn process_mouse(&mut self, point: &Vec2, parent_transform: &Transform) -> bool {
        if !self.visible {
            return false;
        }

        let contains_pt = self.contains_point(point, parent_transform);
        self.mouse_state.update(contains_pt, point);

        /* ----- Disable score table animation for now ----- */

        // if self.mouse_state.mouse_entered {
        //     let drop_down_pos = SCORE_TABLE_POS;
        //     self.trans_anim = Some(TranslationAnimator::new(
        //         self.transform.translation,
        //         drop_down_pos,
        //         600.0,
        //     ));
        // }

        // if self.mouse_state.mouse_exited {
        //     self.trans_anim = Some(TranslationAnimator::new(
        //         self.transform.translation,
        //         vec2(-450.0, 10.0),
        //         600.0,
        //     ));
        // }

        contains_pt
    }

    fn draw(&mut self, parent_transform: &Transform) {
        if !self.visible {
            return;
        }

        let transform = *parent_transform * self.transform;
        let (pos, _rot, _scale) = transform.trans_rot_scale();

        draw_rectangle(
            pos.x,
            pos.y,
            self.rect_size.x,
            self.rect_size.y,
            macroquad::color::Color::from_rgba(50, 50, 50, 190),
        );

        for text in &mut self.texts {
            text.draw(&transform);
        }
    }
}
