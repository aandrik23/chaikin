mod app;

use app::App;
use macroquad::prelude::*;

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "Chaikin".to_owned(),
        window_width: WINDOW_WIDTH as i32,
        window_height: WINDOW_HEIGHT as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut app = App::new();

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        app.update();

        clear_background(WHITE);
        app.draw();

        next_frame().await;
    }
}
