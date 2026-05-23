# Chaikin Project Team Split

## Scope

Build a Rust desktop app that lets users place control points with the mouse, displays those points, and animates Chaikin's algorithm step by step. The animation runs through 7 iterations and then restarts. The project must include unit tests for the Chaikin algorithm.

Source instructions:

- `docs/instructions.md`
- `docs/audit.md`

Current code status:

- `src/main.rs` only prints `Hello, world!`.
- `Cargo.toml` has no graphics/windowing dependencies yet.
- The full application still needs to be implemented.

## Suggested Ownership

### Person 1: Algorithm and Tests

Owns the Chaikin algorithm implementation, point data model, and unit tests. This work should be independent of the windowing/rendering library so it can be tested with `cargo test`.

### Person 2: Window, Input, and Rendering

Owns the app shell: creating the window/canvas, handling mouse and keyboard input, drawing points, drawing lines, and quitting on `Escape`.

### Person 3: Animation, State, and QA

Owns the animation loop, app state transitions, restart behavior after 7 steps, edge cases, bonus controls, and final validation against the audit checklist.

## Recommended Work Order

1. Person 1 creates the algorithm module and tests first.
2. Person 2 chooses and wires the graphics/windowing library.
3. Person 2 integrates the point model from Person 1.
4. Person 3 adds animation state using Person 1's algorithm and Person 2's rendering primitives.
5. All three run through `docs/audit.md` together before submission.

## Tickets

### CHK-001: Add Point Model and Chaikin Algorithm

Owner: Person 1

Type: Core

Description:
Create a reusable point type and a pure function that performs one open-curve Chaikin iteration. For an input list of `N` points, the next iteration should contain `2N - 2` points when `N >= 2`.

Requirements:

- Add a point representation, for example `Point { x: f32, y: f32 }`.
- Add a function similar to `chaikin_iteration(points: &[Point]) -> Vec<Point>`.
- For each segment from `p0` to `p1`, generate:
  - `q = 0.75 * p0 + 0.25 * p1`
  - `r = 0.25 * p0 + 0.75 * p1`
- Handle `0`, `1`, and `2` input points without crashing.
- Keep this logic separate from rendering code.

Acceptance criteria:

- One iteration produces mathematically correct 25% and 75% cut points.
- Empty and single-point inputs are safe.
- Two-point input returns a straight-line-compatible result.
- Function is usable from both tests and app runtime.

### CHK-002: Add Unit Tests for Algorithm

Owner: Person 1

Type: Test

Description:
Add unit tests required by the project instructions.

Requirements:

- Test coordinates after one Chaikin iteration.
- Test point count growth for open curves.
- Test edge cases for `0`, `1`, and `2` points.
- Use approximate float comparisons where needed.

Acceptance criteria:

- `cargo test` passes.
- Tests clearly cover the required cases from `docs/instructions.md`.

### CHK-003: Choose and Configure Windowing/Rendering Library

Owner: Person 2

Type: Infrastructure

Description:
Pick a Rust library for window creation, drawing, mouse input, and keyboard input. A simple crate such as `macroquad` is a practical fit for this assignment because it covers rendering and input in one dependency.

Requirements:

- Add the selected crate to `Cargo.toml`.
- Replace the starter `main` with an application loop.
- Open a visible window/canvas.
- Keep the project runnable with `cargo run`.

Acceptance criteria:

- `cargo run` opens the app window.
- The app compiles without warnings.
- Pressing `Escape` exits cleanly.

### CHK-004: Implement Mouse Point Placement

Owner: Person 2

Type: Feature

Description:
Allow the user to place control points with the left mouse button.

Requirements:

- Left click adds a control point at the mouse position.
- Points remain visible after placement.
- Pressing `Enter` before points are added must not break point placement.

Acceptance criteria:

- User can place one or more control points.
- Points are stored in app state.
- Points can still be added after an empty `Enter` press.

### CHK-005: Render Control Points and Lines

Owner: Person 2

Type: Feature

Description:
Draw the user-selected control points and the current curve state.

Requirements:

- Draw a small circle around each control point.
- For one point, show only the point.
- For two points, draw a straight line.
- For three or more points, draw line segments for the current displayed curve.

Acceptance criteria:

- Control points are visually identifiable.
- One-point and two-point behavior matches the instructions.
- Rendering is clear enough to verify manually.

### CHK-006: Add Animation State Machine

Owner: Person 3

Type: Feature

Description:
Start the Chaikin animation when `Enter` is pressed and there are control points.

Requirements:

- `Enter` starts animation only when at least one point exists.
- The animation displays each algorithm step in sequence.
- The app tracks the current iteration number.
- The displayed curve is regenerated from the original control points for each step.

Acceptance criteria:

- With three or more points, pressing `Enter` starts the animation.
- The app visibly changes from one Chaikin step to the next.
- With one point, the app does not cycle.
- With two points, the app shows a straight line.

### CHK-007: Restart Animation After 7 Steps

Owner: Person 3

Type: Feature

Description:
Complete the required 7-step animation cycle and restart it automatically.

Requirements:

- Run exactly through steps `1` to `7`.
- After step `7`, restart the animation cycle from the beginning.
- Keep original control points unchanged during animation.
- Avoid accumulating Chaikin points indefinitely across cycles.

Acceptance criteria:

- Three or more points animate through 7 steps.
- After the 7th step, the animation restarts.
- Memory and point counts stay bounded to the expected iteration output.

### CHK-008: Add Clear Screen Bonus

Owner: Person 3

Type: Bonus

Description:
Add a key binding to clear the current control points and reset animation state.

Recommended binding:

- `C` clears the screen.

Requirements:

- Clear all control points.
- Stop any active animation.
- Reset the current iteration to zero.
- Allow the user to add new points immediately after clearing.

Acceptance criteria:

- User can clear without restarting the program.
- New points can be added after clearing.

### CHK-009: Add Optional Empty-Enter Message

Owner: Person 3

Type: Bonus

Description:
Show a short message if the user presses `Enter` before drawing points.

Requirements:

- Message should not block input.
- Message should disappear automatically after a short time or after placing a point.
- App must continue running normally.

Acceptance criteria:

- Pressing `Enter` with no points gives visible feedback.
- The user can still add points afterward.

### CHK-010: Add Dragging Bonus

Owner: Person 3

Type: Bonus

Description:
Allow users to drag existing control points and see the curve update.

Requirements:

- Detect when the mouse is pressed near an existing control point.
- Move the selected point while dragging.
- Recompute the current displayed curve from the updated control points.
- Keep behavior stable while animation is running.

Acceptance criteria:

- Existing points can be repositioned.
- The rendered curve updates after dragging.
- Dragging does not create duplicate points accidentally.

### CHK-011: Manual QA Against Audit Checklist

Owner: Person 3

Type: QA

Description:
Run through `docs/audit.md` and record pass/fail notes before submission.

Requirements:

- Run `cargo run`.
- Run `cargo test`.
- Verify mouse placement.
- Verify control point circles.
- Verify `Enter` behavior with `0`, `1`, `2`, and `3+` points.
- Verify 7-step restart behavior.
- Verify `Escape` exits cleanly.
- Verify bonus behavior if implemented.

Acceptance criteria:

- All required audit items pass.
- Any skipped bonus item is clearly marked as not implemented.
- Known issues are documented before handoff.

## Integration Notes

- Keep algorithm code pure and deterministic so Person 1 can test it without depending on the graphics library.
- Keep original control points separate from generated curve points.
- Generate each displayed iteration from the original control points. This makes restart behavior simpler and avoids compounding state bugs.
- For float tests, avoid exact equality when values are produced by arithmetic; use an epsilon comparison.
- Agree on the public function names early so Person 2 and Person 3 can integrate without churn.

## Definition of Done

- `cargo run` opens the app and supports all required interactions.
- `cargo test` passes.
- Left click adds visible control points.
- `Enter` starts animation only when appropriate.
- One point stays as one point.
- Two points render as a straight line.
- Three or more points animate through 7 Chaikin steps and restart.
- `Escape` exits the app.
- Required algorithm tests are present.
- `docs/audit.md` has been manually checked.
