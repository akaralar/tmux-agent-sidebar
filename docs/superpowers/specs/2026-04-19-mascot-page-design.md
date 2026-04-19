# Mascot feature page (website)

## Goal

Add a docs page that explains the sidebar mascot — the cat that lives at the
bottom of the sidebar and animates in response to agent activity. Surface it
under the existing **Features** sidebar group so it sits alongside the other
visible-from-the-UI features (agent pane, worktree, activity log, git status,
notifications).

The page should match the format of the existing feature pages: hero
screenshot at the top, short intro paragraph, then per-element subsections
with images.

## Out of scope

- Changes to the mascot implementation itself (`src/ui/mascot.rs`).
- Animated demos in the docs (GIF / WebM). Static screenshots only for v1.
- A separate marketplace / promo page for the mascot.

## Page

Path: `website/src/content/docs/features/mascot.mdx`

Structure:

```
---
title: Mascot
description: A short, evergreen blurb (under 160 chars). Used as og:description.
---

[hero image: mascot in Working state, cropped to the mascot band]

A short paragraph explaining what the mascot is, where it lives in the sidebar,
and that its state is driven by the running-agent count.

## States

### Idle
[mascot-idle.png]
1-2 sentences: 0 running agents → cat sits at the home position; occasional
blink / wave animation while it waits.

### Walking
[mascot-walking.png]
1-2 sentences: when running count transitions (0 → ≥1 or ≥1 → 0), cat walks
to the desk or back to home.

### Working
[mascot-working.png]
1-2 sentences: ≥1 running agent → cat sits at the desk shuffling papers. The
paper stack height grows with the running-agent count.
```

Tone & length: match the existing feature pages — terse, descriptive, no
marketing voice. Use `import { Image } from 'astro:assets';` and the same
`densities={[1.5, 2]}` pattern as `agent-pane.mdx`.

## Sidebar wiring

`website/astro.config.mjs`, in the `Features` group, append:

```js
{ slug: 'features/mascot' },
```

as the 5th entry (after `notifications`).

## Capture pipeline

Three new static scenarios under `fixtures/scenarios/`:

1. `mascot-idle/scenario.sh`
2. `mascot-walking/scenario.sh`
3. `mascot-working/scenario.sh`

Each scenario:

- Sources `common/_lib.sh` and uses the same `setup` / `build_layout` /
  `start_sidebar` / `capture_single` helpers as the existing scenarios.
- Sets `BOTTOM_HEIGHT=0` so the bottom panel doesn't crowd the mascot band.
- Sets `CROP_COLS=0:46` (full sidebar width) and `CROP_ROWS=<mascot band>`
  to crop output down to just the cat + desk area.
  - Exact row range to be determined empirically when wiring the first
    scenario; the band is approximately 4 rows above the bottom panel.
- Manipulates running-agent count to drive the mascot into the desired state,
  then sleeps long enough for the animation to settle before calling
  `capture_single`.

State control mechanics, derived from `src/ui/mascot.rs` and the existing
`build_layout` helper:

| Scenario          | How to drive the mascot                                                 |
|-------------------|-------------------------------------------------------------------------|
| `mascot-idle`     | Override `_seed_pane` so all panes have `status=waiting`/`idle` (no `running`). After `start_sidebar`, sleep ~2 s — mascot stays at `MASCOT_HOME_X`. |
| `mascot-working`  | Default `build_layout` already includes 2 running agents. Sleep long enough for `WalkRight` to traverse from home to the desk and the state to flip to `Working`. The walk advances 1 col per spinner tick (200 ms) over ~40 cols, so ~10 s should be safe. |
| `mascot-walking`  | Same starting condition as `mascot-working`, but capture earlier — at a sleep duration where the cat is mid-traverse but not yet at the desk. Likely 2–4 s after `start_sidebar`. |

Pipeline registration: `scripts/build-assets.sh`, after the existing
`render_static` calls, append:

```bash
render_static mascot-idle
render_static mascot-walking
render_static mascot-working
```

Image assets land in `website/src/assets/captures/` and are imported from the
mdx page via `astro:assets`.

## Risks & open questions

- **Walking is transient.** The chosen sleep duration in
  `mascot-walking/scenario.sh` is timing-sensitive — the cat advances 1
  column per 200 ms tick. If the captured frame is consistently at home or
  at the desk, the fallback options are:
  1. Add a debug-only env var or build feature that forces `MascotState`
     for capture purposes.
  2. Switch the Walking image to a short animated capture
     (`fixtures/scenarios/mascot-walking/` produces a `.webm` or a single
     well-chosen frame from a `capture_loop`).
  3. Drop Walking from the docs page and only show Idle + Working.

  v1 ships with the static-frame approach; revisit only if unstable.

- **Mascot band crop range.** Exact `CROP_ROWS` value isn't known up
  front — needs to be measured against the rendered frame. The
  implementation plan should call this out as a "render once, inspect, set
  the crop" step rather than guessing.

- **Sidebar order.** Mascot is the most cosmetic / least functional of the
  Features entries, so placing it last in the group is intentional. If the
  user later wants it higher (e.g. just under `agent-pane`), reordering is
  a one-line change in `astro.config.mjs`.

## Success criteria

- `npm run build` (or the Astro build command) inside `website/` succeeds.
- `scripts/build-assets.sh` produces the three new PNGs without errors.
- The new `Mascot` entry appears in the Features sidebar on the rendered
  site, with a working hero image and three legible state thumbnails.
- The page reads consistently with the other feature pages — no
  marketing-voice drift, no inconsistent heading depth.
