use std::f32::consts::PI;

use macroquad::math::{vec2, Vec2};

pub const SCREEN: Vec2 = vec2(800., 800.);
pub const CENTER: Vec2 = vec2(SCREEN.x * 0.5, SCREEN.y * 0.5);

pub const DECK_POS: Vec2 = vec2(200.0, 200.0); //vec2(SCREEN.x - 80.0, SCREEN.y - 100.0);
pub const NEST_EXCHANGE_POS: Vec2 = vec2(CENTER.x, CENTER.y - 30.0);
pub const MESSAGE_POS: Vec2 = vec2(CENTER.x, CENTER.y + 110.0);
pub const SCORE_TABLE_POS: Vec2 = vec2(SCREEN.x - 260.0, 0.0); // -175?
pub const PLAY_BUTTON_POS: Vec2 = vec2(CENTER.x, CENTER.y + 100.0);
pub const BID_PANEL_POS: Vec2 = vec2(CENTER.x, CENTER.y + 220.);
pub const DONE_EXCHANGING_BUTTON_POS: Vec2 = vec2(CENTER.x, CENTER.y + 60.0);
pub const TRUMP_CHOOSER_POS: Vec2 = vec2(CENTER.x, CENTER.y + 70.);

//pub const TURN_MARKER_SPEED: f32 = 600.0;
pub const CARD_SPEED: f32 = 800.0;
pub const ROT_SPEED: f32 = 10.0;

pub struct ViewGeom {
    pub pos: Vec2,
    pub rot: f32,
    pub z: usize,
}

impl Default for ViewGeom {
    fn default() -> Self {
        Self {
            pos: Vec2::ZERO,
            rot: 0.0,
            z: 0,
        }
    }
}

pub fn turn_marker_geom(player: usize, player_count: usize) -> ViewGeom {
    let rad = player_radians_from_center(player, player_count);
    ViewGeom {
        pos: position_from(CENTER, rad, 240.0),
        ..Default::default()
    }
}

pub fn bid_marker_geom(player: usize, player_count: usize) -> ViewGeom {
    let rad = player_radians_from_center(player, player_count);
    ViewGeom {
        pos: position_from(CENTER, rad, 220.0),
        ..Default::default()
    }
}

pub fn deck_geom(dealer: Option<usize>, player_count: usize, index: usize) -> ViewGeom {
    if let Some(dealer) = dealer {
        let rad = player_radians_from_center(dealer, player_count);
        ViewGeom {
            pos: position_from(CENTER, rad, 190.0),
            rot: player_rotation(dealer, player_count),
            ..Default::default()
        }
    }
    else {
        ViewGeom {
            pos: SCREEN + vec2(100.0, 100.0),
            ..Default::default()
        }        
    }
}

pub fn nest_exchange_geom(index: usize, count: usize) -> ViewGeom {
    let max_width = 300.;
    let max_spacing: f32 = 80.;

    let computed_width = max_width / count as f32;
    let x_spacing = max_spacing.min(computed_width);

    let mut x_offset = (count - 1) as f32 * -x_spacing / 2.0;
    x_offset += index as f32 * x_spacing;
    let pos = NEST_EXCHANGE_POS + vec2(x_offset, 0.0);

    ViewGeom {
        pos,
        rot: 0.0,
        z: index,
    }
}

pub fn nest_aside_geom(index: usize, count: usize) -> ViewGeom {
    let max_width = 150.;
    let max_spacing: f32 = 30.;

    // let computed_width = max_width / count as f32;
    // let x_spacing = max_spacing.min(computed_width);

    // let mut x_offset = (count - 1) as f32 * -x_spacing / 2.0;
    // x_offset += index as f32 * x_spacing;
    // let pos = vec2(210., SCREEN.y - 210.0) + vec2(x_offset, 0.0);

    let start = vec2(50.0, 70.0);
    let x_offset = max_spacing * index as f32;
    let pos = start + vec2(x_offset, 0.0);

    ViewGeom {
        pos,
        rot: 0.0, // was -0.2
        z: index,
    }
}

pub fn player_rotation(player: usize, count: usize) -> f32 {
    player as f32 * PI * 2.0 / count as f32
}

pub fn player_radians_from_center(player: usize, count: usize) -> f32 {
    player as f32 * PI * 2.0 / count as f32 + PI / 2.0
}

pub fn position_from(start_pos: Vec2, radians: f32, magnitude: f32) -> Vec2 {
    vec2(
        start_pos.x + radians.cos() * magnitude,
        start_pos.y + radians.sin() * magnitude,
    )
}

pub fn hand_card_geom(
    player: usize,
    index: usize,
    hand_count: usize,
    player_count: usize,
    is_bot: bool,
) -> ViewGeom {
    let distance_from_center = 330.0;

    let max_width = match is_bot {
        true => 250.,
        false => 530.,
    };
    let max_spacing: f32 = 60.;

    let computed_width = max_width / hand_count as f32;
    let x_spacing = max_spacing.min(computed_width);

    let mut x_offset = (hand_count - 1) as f32 * -x_spacing / 2.0;
    x_offset += index as f32 * x_spacing;

    let rad = player_radians_from_center(player, player_count);
    let mut pos = position_from(CENTER, rad, distance_from_center);

    let angle = player_rotation(player, player_count);
    pos.x += x_offset * angle.cos();
    pos.y += x_offset * angle.sin();

    ViewGeom {
        pos,
        rot: angle,
        z: index + 100,
    }
}

pub fn trick_card_geom(player: usize, player_count: usize) -> ViewGeom {
    let distance_from_center = 105.0;
    let rad = player_radians_from_center(player, player_count);
    let pos = position_from(CENTER, rad, distance_from_center);
    let angle = player_rotation(player, player_count);

    ViewGeom {
        pos,
        rot: angle,
        z: 200,
    }
}

pub fn taken_geom(player: usize, player_count: usize) -> ViewGeom {
    let distance_from_center = 470.0;
    let rad = player_radians_from_center(player, player_count);
    let pos = position_from(CENTER, rad, distance_from_center);
    let angle = player_rotation(player, player_count) + PI * 0.5;
    ViewGeom {
        pos,
        rot: angle,
        z: 0,
    }
}
