pub mod animators;
pub mod bid_marker;
pub mod bid_panel;
pub mod button_shaded;
pub mod button_state;
pub mod button_text;
pub mod card_view;
pub mod eventer;
pub mod imager;
pub mod score_table;
pub mod sprite;
pub mod texter;
pub mod transform;
//pub mod transform4;
pub mod trump_chooser;
pub mod trump_marker;
pub mod view;
pub mod view_geom;

use macroquad::prelude::*;

#[allow(dead_code)]
/// Returns the number of physical pixels per logical pixel.
pub fn dpi_scale() -> f32 {
    miniquad::window::dpi_scale()
}

#[allow(dead_code)]
/// Draw to a texture using the given function with drawing commands. Width and height
/// are passed in as examples of passing arguments to the draw function.
pub fn draw_to_texture(draw_fn: fn(u32, u32), phys_width: u32, phys_height: u32) -> Texture2D {
    let render_target = render_target(phys_width, phys_height);
    // For pixel art, use:
    // render_target.texture.set_filter(FilterMode::Nearest);

    // The zoom x:y ratio must match the phys_width:phys_height ratio, with 0.01 as the nominal setting.
    let (mut zoom_x, mut zoom_y) = (0.01, 0.01);
    if phys_width > phys_height {
        zoom_y = 0.01 * phys_width as f32 / phys_height as f32;
    } else {
        zoom_x = 0.01 * phys_height as f32 / phys_width as f32;
    }

    set_camera(&Camera2D {
        // It seems that, when rendering to a texture, 0.01 means "no zoom", a 1:1 pixel ratio.
        zoom: vec2(zoom_x, zoom_y),
        // Look at the center of the texture.
        target: vec2(phys_width as f32 / 2.0, phys_height as f32 / 2.0),
        render_target: Some(render_target.clone()),
        ..Default::default()
    });

    draw_fn(phys_width, phys_height);

    // All done. Restore default camera.
    set_default_camera();
    render_target.texture
}
