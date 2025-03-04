use macroquad::prelude::*;

mod bot_monte;
mod card;
mod controller;
use controller::Controller;
mod game;
mod game_options;
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
    unsafe {
        std::env::set_var("RUST_BACKTRACE", "1");
    }


    // &mut dyn std::any::Any;
    fn print_if_string(value: Box<dyn std::any::Any>) {
        if let Ok(mut string) = value.downcast::<String>() {
            string.push_str("abc");
            println!("String ({}): {}", string.len(), string);
        }
    }
    
    let my_string = "Hello World".to_string();
    let mut my_box = Box::new(my_string.clone());
    my_box.push_str("abc");
    print_if_string(Box::new(my_string));
    print_if_string(Box::new(0i8));

    let mut controller = Controller::new().await;
    controller.go().await;
}
