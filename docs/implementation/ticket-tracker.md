# Ticket Progress Tracker

This file tracks delivery progress for the Chaikin project.

## Team Assignment

| Track | Dev | Focus Area |
|-------|-----|------------|
| **A** | Dev 1 | Algorithm & Unit Tests (Point model, Chaikin iteration, edge-case tests) |
| **B** | Dev 2 | Window, Input & Rendering (canvas, mouse points, keyboard controls, drawing) |
| **C** | Dev 3 | Animation, Bonus Features & Audit (7-step loop, reset behavior, QA sign-off) |

---

Detailed ticket definitions live in:

- `docs/team_split_tickets.md`

The canonical product and technical requirements are in:

- `docs/instructions.md` — assignment instructions and required functionality
- `docs/audit.md` — manual audit checklist

## Update Rules

1. Keep each ticket in the line format: status + ticket ID + short description + dependency fields.
2. Use `[x]` only when the acceptance criteria in `docs/team_split_tickets.md` are satisfied.
3. Use `[-]` only when a meaningful subset of that ticket already exists in code.
4. Keep `Depends on` and `Blocks` synchronized when ticket definitions change.
5. Do not remove completed tickets from the tracker.
6. A test or QA ticket may start early only when all of its direct dependencies are complete.

## Status Legend

- `[ ]` = Not Started
- `[-]` = Partially Implemented / In Progress
- `[x]` = Done

## Summary Snapshot

- Total tickets: `11`
- Done: `5`
- Partially Implemented: `0`
- Not Started: `6`

---

## Implementation Order — Foundation-First Strategy

The implementation is organized into **4 waves**. Wave 1 lands testable algorithm foundations. Wave 2 builds the visible app shell. Wave 3 adds animation and bonus behavior. Wave 4 is final QA against the audit checklist.

### Wave 1 — Algorithm Foundation (P0)

> **Goal:** create the pure Chaikin implementation and required tests so rendering and animation can integrate against stable logic.

| # | Status | Ticket | Track | Description | Depends on | Blocks |
|---|--------|--------|-------|-------------|------------|--------|
| 1 | [x] | **A01** | A | Add Point model and pure Chaikin iteration function for open curves | None | A02, B02, B03, C01, C02 |
| 2 | [x] | **A02** | A | Add unit tests for one-step math, point-count growth, and 0/1/2-point edge cases | A01 | C04 |

### Wave 2 — Interactive App Shell (P1)

> **Goal:** open a real window, collect user control points, render the required visible state, and support required keyboard behavior.

| # | Status | Ticket | Track | Description | Depends on | Blocks |
|---|--------|--------|-------|-------------|------------|--------|
| 3 | [x] | **B01** | B | Choose and configure windowing/rendering library; replace starter `main` with app loop | None | B02, B03, C01, C02, C03, C04 |
| 4 | [x] | **B02** | B | Implement left-click point placement and preserve input after empty `Enter` press | A01, B01 | B03, C01, C02, C03, C04 |
| 5 | [x] | **B03** | B | Render control point circles, one-point state, two-point straight line, and curve segments | A01, B01, B02 | C01, C02, C03, C04 |

### Wave 3 — Animation and Bonus Features (P2)

> **Goal:** connect algorithm output to the running app, animate through 7 steps, and add optional usability improvements.

| # | Status | Ticket | Track | Description | Depends on | Blocks |
|---|--------|--------|-------|-------------|------------|--------|
| 6 | [ ] | **C01** | C | Add animation state machine started by `Enter`, with correct 1/2/3+ point behavior | A01, B02, B03 | C02, C03, C04 |
| 7 | [ ] | **C02** | C | Restart animation automatically after exactly 7 Chaikin steps | A01, B03, C01 | C04 |
| 8 | [ ] | **C03** | C | Add clear-screen bonus control, recommended binding `C` | B02, B03 | C04 |
| 9 | [ ] | **C04** | C | Add optional empty-`Enter` message without blocking future input | B01, B02 | C06 |
| 10 | [ ] | **C05** | C | Add optional drag-control-points bonus with live curve update | B02, B03, C01 | C06 |

### Wave 4 — Audit Sign-off (P3)

> **Goal:** verify the complete project against the required checklist and mark it ready for review.

| # | Status | Ticket | Track | Description | Depends on | Blocks |
|---|--------|--------|-------|-------------|------------|--------|
| 11 | [ ] | **C06** | C | Final manual QA against `docs/audit.md`, including `cargo run`, `cargo test`, input, animation, and exit behavior | A02, B03, C02 | None |
