use chaikin::chaikin::{chaikin_iteration, Point};

pub const MAX_ANIMATION_STEP: usize = 7;
const STEP_DURATION_SECONDS: f32 = 0.6;

#[derive(Debug, Clone)]
pub struct AnimationState {
    pub is_running: bool,
    pub current_step: usize,
    pub displayed_curve: Vec<Point>,
    elapsed_time: f32,
}

impl AnimationState {
    pub fn new() -> Self {
        Self {
            is_running: false,
            current_step: 0,
            displayed_curve: Vec::new(),
            elapsed_time: 0.0,
        }
    }

    pub fn start(&mut self, control_points: &[Point]) {
        if control_points.len() < 3 {
            self.is_running = false;
            self.current_step = 0;
            self.displayed_curve = control_points.to_vec();
            self.elapsed_time = 0.0;
            return;
        }

        self.is_running = true;
        self.current_step = 1;
        self.elapsed_time = 0.0;
        self.displayed_curve = generate_curve_at_step(control_points, self.current_step);
    }

    pub fn update(&mut self, control_points: &[Point], delta_time: f32) {
        if !self.is_running || control_points.len() < 3 {
            return;
        }

        self.elapsed_time += delta_time;

        if self.elapsed_time >= STEP_DURATION_SECONDS {
            self.elapsed_time = 0.0;

            if self.current_step >= MAX_ANIMATION_STEP {
                self.current_step = 1;
            } else {
                self.current_step += 1;
            }

            self.displayed_curve = generate_curve_at_step(control_points, self.current_step);
        }
    }

    pub fn reset(&mut self) {
        self.is_running = false;
        self.current_step = 0;
        self.displayed_curve.clear();
        self.elapsed_time = 0.0;
    }
}

pub fn generate_curve_at_step(control_points: &[Point], step: usize) -> Vec<Point> {
    if control_points.len() < 3 {
        return control_points.to_vec();
    }

    let first = control_points[0];
    let last = control_points[control_points.len() - 1];

    let mut curve = control_points.to_vec();

    for _ in 0..step {
        curve = chaikin_iteration(&curve);

        curve.insert(0, first);
        curve.push(last);
    }

    curve
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn animation_restarts_after_step_seven() {
        let points = vec![
            Point::new(0.0, 0.0),
            Point::new(10.0, 0.0),
            Point::new(10.0, 10.0),
        ];

        let mut animation = AnimationState::new();
        animation.start(&points);

        assert_eq!(animation.current_step, 1);

        for _ in 0..6 {
            animation.update(&points, STEP_DURATION_SECONDS);
        }

        assert_eq!(animation.current_step, 7);

        animation.update(&points, STEP_DURATION_SECONDS);

        assert_eq!(animation.current_step, 1);
    }

    #[test]
    fn animation_step_never_exceeds_max_step() {
        let points = vec![
            Point::new(0.0, 0.0),
            Point::new(10.0, 0.0),
            Point::new(10.0, 10.0),
        ];

        let mut animation = AnimationState::new();
        animation.start(&points);

        for _ in 0..50 {
            animation.update(&points, STEP_DURATION_SECONDS);

            assert!(
                animation.current_step <= MAX_ANIMATION_STEP,
                "animation step exceeded max step: {}",
                animation.current_step
            );
        }
    }
}