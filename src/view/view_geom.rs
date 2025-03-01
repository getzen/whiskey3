use std::f32::consts::PI;

use macroquad::math::{Vec2, vec2};

pub const SCREEN: Vec2 = vec2(1000., 1000.);

pub const PLAY_TOP_LEFT: Vec2 = vec2(0.0, 100.0);
pub const PLAY_BOTTOM_RIGHT: Vec2 = vec2(SCREEN.x, SCREEN.y);
pub const PLAY_CENTER: Vec2 = vec2(
    PLAY_TOP_LEFT.x + (PLAY_BOTTOM_RIGHT.x - PLAY_TOP_LEFT.x) * 0.5,
    PLAY_TOP_LEFT.y + (PLAY_BOTTOM_RIGHT.y - PLAY_TOP_LEFT.y) * 0.5,
);

pub const NEST_EXCHANGE_POS: Vec2 = vec2(PLAY_CENTER.x, PLAY_CENTER.y - 30.0);
pub const NEST_ASIDE_POS: Vec2 = vec2(PLAY_TOP_LEFT.x + 90.0, PLAY_BOTTOM_RIGHT.y - 120.);
pub const MESSAGE_POS: Vec2 = vec2(PLAY_CENTER.x, PLAY_CENTER.y + 110.0);
pub const SCORE_TABLE_POS: Vec2 = vec2(0., 0.);
pub const PLAY_BUTTON_POS: Vec2 = vec2(PLAY_CENTER.x, PLAY_CENTER.y + 100.0);
pub const BID_PANEL_POS: Vec2 = vec2(PLAY_CENTER.x, PLAY_CENTER.y + 220.);
pub const DONE_EXCHANGING_BUTTON_POS: Vec2 = vec2(PLAY_CENTER.x, PLAY_CENTER.y + 60.0);
pub const NEXT_HAND_BUTTON_POS: Vec2 = vec2(PLAY_CENTER.x, PLAY_CENTER.y + 200.0);
pub const TRUMP_CHOOSER_POS: Vec2 = vec2(PLAY_CENTER.x, PLAY_CENTER.y + 70.);

pub const CARD_SPEED: f32 = 800.0;
pub const ROT_SPEED: f32 = 10.0;

pub type Z = u16;
// The base z-order for cards.
const CARD_Z: Z = 100;

pub struct ViewGeom {
    pub pos: Vec2,
    pub rot: f32,
    pub z: Z,
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
        pos: position_from(PLAY_CENTER, rad, 420.0),
        ..Default::default()
    }
}

pub fn bid_marker_geom(player: usize, player_count: usize) -> ViewGeom {
    let rad = player_radians_from_center(player, player_count);
    ViewGeom {
        pos: position_from(PLAY_CENTER, rad, 220.0),
        ..Default::default()
    }
}

pub fn deck_geom(dealer: Option<usize>, player_count: usize, _index: usize) -> ViewGeom {
    if let Some(dealer) = dealer {
        let rad = player_radians_from_center(dealer, player_count);
        ViewGeom {
            pos: position_from(PLAY_CENTER, rad, 190.0),
            rot: player_rotation(dealer, player_count),
            ..Default::default()
        }
    } else {
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
        z: CARD_Z + index as Z,
    }
}

pub fn nest_aside_geom(index: usize, count: usize) -> ViewGeom {
    let max_width = 130.;
    let max_spacing: f32 = 40.;

    let computed_width = max_width / count as f32;
    let x_spacing = max_spacing.min(computed_width);

    let mut offset = (count - 1) as f32 * -x_spacing / 2.0;
    offset += index as f32 * x_spacing;
    let pos = NEST_ASIDE_POS + vec2(offset, -offset);

    ViewGeom {
        pos,
        rot: -0.3,
        z: CARD_Z + index as Z,
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

pub fn hand_card_geom(player: usize, index: usize, hand_count: usize, player_count: usize, is_bot: bool) -> ViewGeom {
    let distance_from_center = 330.0;

    let max_width = match is_bot {
        true => 300.,
        false => 530.,
    };
    let max_spacing: f32 = 60.;

    let computed_width = max_width / hand_count as f32;
    let x_spacing = max_spacing.min(computed_width);

    let mut x_offset = (hand_count - 1) as f32 * -x_spacing / 2.0;
    x_offset += index as f32 * x_spacing;

    let rad = player_radians_from_center(player, player_count);
    let mut pos = position_from(PLAY_CENTER, rad, distance_from_center);

    let angle = player_rotation(player, player_count);
    pos.x += x_offset * angle.cos();
    pos.y += x_offset * angle.sin();

    ViewGeom {
        pos,
        rot: angle,
        z: CARD_Z + index as Z,
    }
}

pub fn trick_card_geom(player: usize, player_count: usize) -> ViewGeom {
    let distance_from_center = 105.0;
    let rad = player_radians_from_center(player, player_count);
    let pos = position_from(PLAY_CENTER, rad, distance_from_center);
    let angle = player_rotation(player, player_count);

    ViewGeom {
        pos,
        rot: angle,
        z: CARD_Z + 200,
    }
}

pub fn taken_geom(player: usize, player_count: usize) -> ViewGeom {
    let distance_from_center = 450.0;
    let rad = player_radians_from_center(player, player_count);
    let pos = position_from(PLAY_CENTER, rad, distance_from_center);
    let angle = player_rotation(player, player_count) + PI * 0.5;
    ViewGeom { pos, rot: angle, z: 0 }
}
