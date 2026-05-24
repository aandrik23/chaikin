#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

pub fn chaikin_iteration(points: &[Point]) -> Vec<Point> {
    match points.len() {
        0 => Vec::new(),
        1 => points.to_vec(),
        _ => {
            let mut next = Vec::new();

            for segment in points.windows(2) {
                let p0 = segment[0];
                let p1 = segment[1];

                let q = Point::new(
                    0.75 * p0.x + 0.25 * p1.x,
                    0.75 * p0.y + 0.25 * p1.y,
                );

                let r = Point::new(
                    0.25 * p0.x + 0.75 * p1.x,
                    0.25 * p0.y + 0.75 * p1.y,
                );

                next.push(q);
                next.push(r);
            }

            next
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f32 = 1e-6;

    fn assert_close(a: Point, b: Point) {
        assert!(
            (a.x - b.x).abs() < EPS && (a.y - b.y).abs() < EPS,
            "expected {:?}, got {:?}",
            b,
            a
        );
    }

    #[test]
    fn one_iteration_produces_quarter_and_three_quarter_points() {
        let input = [Point::new(0.0, 0.0), Point::new(4.0, 0.0)];
        let out = chaikin_iteration(&input);
        assert_eq!(out.len(), 2);
        assert_close(out[0], Point::new(1.0, 0.0));
        assert_close(out[1], Point::new(3.0, 0.0));
    }

    #[test]
    fn one_iteration_handles_two_dimensional_segment() {
        let out = chaikin_iteration(&[Point::new(0.0, 0.0), Point::new(8.0, 4.0)]);
        assert_close(out[0], Point::new(2.0, 1.0));
        assert_close(out[1], Point::new(6.0, 3.0));
    }

    #[test]
    fn point_count_grows_to_two_n_minus_two() {
        for n in 2..=10 {
            let input: Vec<Point> = (0..n).map(|i| Point::new(i as f32, 0.0)).collect();
            let out = chaikin_iteration(&input);
            assert_eq!(out.len(), 2 * n - 2, "N={n}");
        }
    }

    #[test]
    fn empty_input_returns_empty() {
        assert!(chaikin_iteration(&[]).is_empty());
    }

    #[test]
    fn single_point_input_is_preserved() {
        let input = [Point::new(1.5, 2.5)];
        assert_eq!(chaikin_iteration(&input), input.to_vec());
    }

    #[test]
    fn two_point_input_returns_exactly_two_collinear_points() {
        let p0 = Point::new(0.0, 0.0);
        let p1 = Point::new(10.0, 0.0);
        let out = chaikin_iteration(&[p0, p1]);
        assert_eq!(out.len(), 2);
        for p in &out {
            assert!((p.y - 0.0).abs() < EPS);
        }
    }
}
