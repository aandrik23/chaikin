use crate::animation::{generate_curve_at_step, AnimationState, MAX_ANIMATION_STEP};
use chaikin::chaikin::Point;
use macroquad::prelude::*;

const CONTROL_POINT_RADIUS: f32 = 5.0;
const CURVE_LINE_THICKNESS: f32 = 2.0;
const EMPTY_ENTER_MESSAGE: &str = "Add at least one point before starting.";
const MESSAGE_DURATION_SECONDS: f32 = 2.0;
const DRAG_PICK_RADIUS: f32 = 10.0;
const HINT_TEXT: &str = "Left click: add/drag  -  C: clear  -  Enter: animate  -  Esc: quit";

pub struct App {
    pub control_points: Vec<Point>,
    pub animation: AnimationState,
    pub empty_enter_message_timer: f32,
    pub dragged_point_index: Option<usize>,
}

impl App {
    pub fn new() -> Self {
        Self {
            control_points: Vec::new(),
            animation: AnimationState::new(),
            empty_enter_message_timer: 0.0,
            dragged_point_index: None,
        }
    }

    fn enter_pressed() -> bool {
        is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::KpEnter)
    }

    pub fn add_control_point(&mut self, x: f32, y: f32) {
        self.control_points.push(Point::new(x, y));
        self.empty_enter_message_timer = 0.0;
    }

    pub fn clear(&mut self) {
        self.control_points.clear();
        self.animation.reset();
        self.empty_enter_message_timer = 0.0;
        self.dragged_point_index = None;
    }

    pub fn on_enter_pressed(&mut self) {
        if self.control_points.is_empty() {
            self.empty_enter_message_timer = MESSAGE_DURATION_SECONDS;
            return;
        }

        self.animation.start(&self.control_points);
    }

    pub fn curve_polyline(&self) -> &[Point] {
        if self.animation.is_running {
            &self.animation.displayed_curve
        } else {
            &[]
        }
    }

    fn point_near_mouse(&self, mouse_x: f32, mouse_y: f32) -> Option<usize> {
        self.control_points.iter().position(|point| {
            let dx = point.x - mouse_x;
            let dy = point.y - mouse_y;
            let distance_squared = dx * dx + dy * dy;

            distance_squared <= DRAG_PICK_RADIUS * DRAG_PICK_RADIUS
        })
    }

    fn start_dragging_or_add_point(&mut self) {
        let (mouse_x, mouse_y) = mouse_position();

        if let Some(index) = self.point_near_mouse(mouse_x, mouse_y) {
            self.dragged_point_index = Some(index);
        } else {
            self.add_control_point(mouse_x, mouse_y);
        }
    }

    fn update_dragging(&mut self) {
        if let Some(index) = self.dragged_point_index {
            let (mouse_x, mouse_y) = mouse_position();

            self.control_points[index] = Point::new(mouse_x, mouse_y);

            if self.animation.is_running {
                self.animation.displayed_curve =
                    generate_curve_at_step(&self.control_points, self.animation.current_step);
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            self.dragged_point_index = None;
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
                WHITE,
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

        draw_text(HINT_TEXT, 10.0, 22.0, 18.0, LIGHTGRAY);

        if self.empty_enter_message_timer > 0.0 {
            draw_text(
                EMPTY_ENTER_MESSAGE,
                10.0,
                50.0,
                24.0,
                RED,
            );
        }

        let step_text = if self.animation.current_step == 0 {
            "Input".to_string()
        } else {
            format!("Step: {}/{}", self.animation.current_step, MAX_ANIMATION_STEP)
        };
        let text_size = measure_text(&step_text, None, 24, 1.0);
        draw_text(
            &step_text,
            screen_width() - text_size.width - 20.0,
            screen_height() - 20.0,
            24.0,
            WHITE,
        );
    }

    pub fn update(&mut self) {
        if is_mouse_button_pressed(MouseButton::Left) {
            self.start_dragging_or_add_point();
        }

        if is_mouse_button_down(MouseButton::Left) {
            self.update_dragging();
        }

        if Self::enter_pressed() {
            self.on_enter_pressed();
        }

        if is_key_pressed(KeyCode::C) {
            self.clear();
        }

        let delta_time = get_frame_time();

        if self.empty_enter_message_timer > 0.0 {
            self.empty_enter_message_timer -= delta_time;
        }

        self.animation.update(&self.control_points, delta_time);
    }
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
    fn curve_polyline_is_empty_until_animation_starts() {
        let mut app = App::new();

        app.add_control_point(0.0, 0.0);
        app.add_control_point(10.0, 0.0);
        app.add_control_point(10.0, 10.0);

        assert!(app.curve_polyline().is_empty());

        app.on_enter_pressed();

        assert!(!app.curve_polyline().is_empty());
    }

    #[test]
    fn clear_removes_points_and_resets_state() {
        let mut app = App::new();
        app.add_control_point(0.0, 0.0);
        app.add_control_point(10.0, 0.0);
        app.add_control_point(10.0, 10.0);
        app.on_enter_pressed();

        app.clear();

        assert!(app.control_points.is_empty());
        assert!(!app.animation.is_running);
        assert_eq!(app.animation.current_step, 0);
        assert!(app.animation.displayed_curve.is_empty());
        assert_eq!(app.empty_enter_message_timer, 0.0);
        assert_eq!(app.dragged_point_index, None);
    }

    #[test]
    fn enter_with_no_points_shows_message_timer() {
        let mut app = App::new();

        app.on_enter_pressed();

        assert!(app.empty_enter_message_timer > 0.0);
    }
}