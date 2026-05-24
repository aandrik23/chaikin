use chaikin::chaikin::Point;
use macroquad::prelude::*;

const CONTROL_POINT_RADIUS: f32 = 5.0;
const CURVE_LINE_THICKNESS: f32 = 2.0;

pub struct App {
    pub control_points: Vec<Point>,
}

impl App {
    pub fn new() -> Self {
        Self {
            control_points: Vec::new(),
        }
    }

    pub fn add_control_point(&mut self, x: f32, y: f32) {
        self.control_points.push(Point::new(x, y));
    }

    /// Handles `Enter` / keypad Enter. Empty-canvas presses are a no-op so placement keeps working.
    pub fn on_enter_pressed(&mut self) {}

    /// Polyline vertices for the current curve (control polygon before animation).
    pub fn curve_polyline(&self) -> &[Point] {
        if self.control_points.len() >= 2 {
            &self.control_points
        } else {
            &[]
        }
    }

    pub fn draw(&self) {
        let curve = self.curve_polyline();
        for segment in curve.windows(2) {
            let p0 = segment[0];
            let p1 = segment[1];
            draw_line(
                p0.x,
                p0.y,
                p1.x,
                p1.y,
                CURVE_LINE_THICKNESS,
                BLACK,
            );
        }

        for point in &self.control_points {
            draw_circle(
                point.x,
                point.y,
                CONTROL_POINT_RADIUS,
                BLUE,
            );
        }
    }

    pub fn update(&mut self) {
        if is_mouse_button_pressed(MouseButton::Left) {
            let (x, y) = mouse_position();
            self.add_control_point(x, y);
        }

        if enter_pressed() {
            self.on_enter_pressed();
        }
    }
}

fn enter_pressed() -> bool {
    is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::KpEnter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_multiple_control_points() {
        let mut app = App::new();
        app.add_control_point(10.0, 20.0);
        app.add_control_point(30.0, 40.0);

        assert_eq!(app.control_points.len(), 2);
        assert_eq!(app.control_points[0], Point::new(10.0, 20.0));
        assert_eq!(app.control_points[1], Point::new(30.0, 40.0));
    }

    #[test]
    fn enter_with_no_points_does_not_block_future_placement() {
        let mut app = App::new();
        app.on_enter_pressed();
        app.add_control_point(5.0, 6.0);

        assert_eq!(app.control_points.len(), 1);
    }

    #[test]
    fn enter_with_existing_points_does_not_clear_them() {
        let mut app = App::new();
        app.add_control_point(1.0, 2.0);
        app.on_enter_pressed();

        assert_eq!(app.control_points.len(), 1);
        assert_eq!(app.control_points[0], Point::new(1.0, 2.0));
    }

    #[test]
    fn curve_polyline_empty_for_zero_or_one_point() {
        let app = App::new();
        assert!(app.curve_polyline().is_empty());

        let mut app = App::new();
        app.add_control_point(1.0, 2.0);
        assert!(app.curve_polyline().is_empty());
    }

    #[test]
    fn curve_polyline_uses_control_points_for_two_or_more() {
        let mut app = App::new();
        app.add_control_point(0.0, 0.0);
        app.add_control_point(10.0, 0.0);
        assert_eq!(app.curve_polyline(), &[Point::new(0.0, 0.0), Point::new(10.0, 0.0)]);

        app.add_control_point(10.0, 10.0);
        assert_eq!(
            app.curve_polyline(),
            &[
                Point::new(0.0, 0.0),
                Point::new(10.0, 0.0),
                Point::new(10.0, 10.0),
            ]
        );
    }
}
