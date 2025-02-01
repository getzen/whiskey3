use macroquad::prelude::*;

mod bot_monte;
mod card;
mod controller;
use controller::Controller;
mod game;
mod scoring;
mod trick;
mod view;

fn conf() -> Conf {
    Conf {
        window_title: String::from("Whiskey"),
        window_width: view::view_geom::SCREEN.x as i32,
        window_height: view::view_geom::SCREEN.y as i32,
        high_dpi: true,
        window_resizable: false,
        ..Default::default() // others available
    }
}

#[macroquad::main(conf)]
async fn main() {
    // Set up backtracing for debugging.
    std::env::set_var("RUST_BACKTRACE", "1");
   
    let mut controller = Controller::new().await; 
    controller.go().await;
}
