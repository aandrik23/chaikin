# Chaikin

An interactive visualizer for [Chaikin's corner-cutting algorithm](https://www.cs.unc.edu/~dm/UNC/COMP258/LECTURES/Chaikins-Algorithm.pdf). Place control points with the mouse, then watch the polyline smooth step by step over seven iterations before the animation loops.

Built in Rust with [macroquad](https://github.com/not-fl3/macroquad).

## Demo behavior

- **Step 0** shows the raw control polyline (the input shape).
- **Steps 1–7** apply Chaikin iterations, subdividing each segment at 25% and 75% along its length.
- The first and last control points are preserved on each iteration so the curve stays anchored at the endpoints.
- With only two control points, the app draws a straight line instead of running the animation loop.

## Controls

| Input | Action |
|-------|--------|
| Left click | Add a control point |
| Left click + drag | Move an existing control point |
| Enter | Start or restart the animation |
| C | Clear all points and reset |
| Esc | Quit |

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024)

## Build and run

```bash
cargo run
```

Release build:

```bash
cargo run --release
```

## Tests

Unit tests cover the Chaikin iteration logic, animation stepping, and app state handling:

```bash
cargo test
```

Tests verify:

- New points land at the 25% and 75% positions after one iteration
- Point count grows as `2N - 2` for an open curve with `N` points
- Edge cases with 0, 1, or 2 input points are handled safely

## Project structure

```
src/
├── main.rs       # Window setup and main loop
├── app.rs        # Input handling, rendering, and UI
├── animation.rs  # Seven-step animation state machine
├── chaikin.rs    # Point type and Chaikin iteration
└── lib.rs        # Library entry (algorithm module)
```

## Algorithm

For each segment between consecutive control points `P₀` and `P₁`, one Chaikin iteration inserts two new points:

- `Q = 0.75 · P₀ + 0.25 · P₁`
- `R = 0.25 · P₀ + 0.75 · P₁`

Repeating this process smooths sharp corners into a curve. This project animates that process so each subdivision step is visible.

## References

- [Chaikin's Algorithm (PDF)](https://www.cs.unc.edu/~dm/UNC/COMP258/LECTURES/Chaikins-Algorithm.pdf)
- [Reference demo video](https://youtu.be/PbB2eKnA2QI)
