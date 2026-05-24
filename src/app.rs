use chaikin::chaikin::Point;
use macroquad::prelude::*;

pub struct App {
    pub control_points: Vec<Point>,
}

impl App {
    pub fn new() -> Self {
        Self {
            control_points: Vec::new(),
        }
    }

    pub fn update(&mut self) {
        if is_mouse_button_pressed(MouseButton::Left) {
            let (x, y) = mouse_position();
            self.control_points.push(Point::new(x, y));
        }

        if enter_pressed() && self.control_points.is_empty() {
            // Intentionally no-op so point placement keeps working.
        }
    }
}

fn enter_pressed() -> bool {
    is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::KpEnter)
}
