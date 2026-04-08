use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
};
use unicode_width::UnicodeWidthStr;

use crate::state::AppState;

/// Cat animation state machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CatState {
    Idle,
    WalkRight,
    Working,
    WalkLeft,
}

pub const CAT_WIDTH: u16 = 7;
pub const CAT_HOME_X: u16 = 1;
pub const DESK_OFFSET: u16 = 0;
pub const DESK_WIDTH: u16 = 4;
pub const CHAIR_WIDTH: u16 = 2;
/// Gap between chair and desk.
pub const CHAIR_DESK_GAP: u16 = 1;
pub const MAX_PAPER_HEIGHT: u16 = 2;
/// Ticks between idle bobs (~8 seconds at 200ms tick).
pub const BOB_INTERVAL: usize = 40;
const CAT_BODY: Color = Color::Indexed(208);
const CAT_EYE: Color = Color::Indexed(114);
const CAT_NOSE: Color = Color::Indexed(174);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum IdleMotion {
    Rest,
    Jump,
    Blink,
}

fn sitting_sprite() -> Vec<Line<'static>> {
    vec![
        Line::from(vec![
            Span::raw(" "),
            Span::styled("▄", Style::new().fg(CAT_BODY)),
            Span::raw(" "),
            Span::styled("▄", Style::new().fg(CAT_BODY)),
        ]),
        Line::from(vec![
            Span::styled("▄", Style::new().fg(CAT_BODY)),
            Span::styled("▀", Style::new().fg(CAT_EYE)),
            Span::styled("▀", Style::new().fg(CAT_NOSE)),
            Span::styled("▀", Style::new().fg(CAT_EYE)),
            Span::styled("▄", Style::new().fg(CAT_BODY)),
        ]),
        Line::from(vec![
            Span::raw(" "),
            Span::styled("▀", Style::new().fg(CAT_BODY)),
            Span::raw(" "),
            Span::styled("▀", Style::new().fg(CAT_BODY)),
        ]),
    ]
}

fn sitting_sprite_blink() -> Vec<Line<'static>> {
    vec![
        Line::from(vec![
            Span::raw(" "),
            Span::styled("▄", Style::new().fg(CAT_BODY)),
            Span::raw(" "),
            Span::styled("▄", Style::new().fg(CAT_BODY)),
        ]),
        Line::from(vec![
            Span::styled("▄", Style::new().fg(CAT_BODY)),
            Span::styled("─", Style::new().fg(CAT_EYE)),
            Span::styled("▀", Style::new().fg(CAT_NOSE)),
            Span::styled("─", Style::new().fg(CAT_EYE)),
            Span::styled("▄", Style::new().fg(CAT_BODY)),
        ]),
        Line::from(vec![
            Span::raw(" "),
            Span::styled("▀", Style::new().fg(CAT_BODY)),
            Span::raw(" "),
            Span::styled("▀", Style::new().fg(CAT_BODY)),
        ]),
    ]
}

fn walking_right_1() -> Vec<Line<'static>> {
    vec![
        Line::from(vec![
            Span::raw(" "),
            Span::styled("▄", Style::new().fg(CAT_BODY)),
            Span::raw(" "),
            Span::styled("▄", Style::new().fg(CAT_BODY)),
        ]),
        Line::from(vec![
            Span::styled("▄", Style::new().fg(CAT_BODY)),
            Span::styled("▀", Style::new().fg(CAT_EYE)),
            Span::styled("▀", Style::new().fg(CAT_NOSE)),
            Span::styled("▀", Style::new().fg(CAT_EYE)),
            Span::styled("▄", Style::new().fg(CAT_BODY)),
        ]),
        Line::from(vec![
            Span::styled("▖▖", Style::new().fg(CAT_BODY)),
            Span::raw(" "),
            Span::styled("▗▗", Style::new().fg(CAT_BODY)),
        ]),
    ]
}

fn walking_right_2() -> Vec<Line<'static>> {
    vec![
        Line::from(vec![
            Span::raw(" "),
            Span::styled("▄", Style::new().fg(CAT_BODY)),
            Span::raw(" "),
            Span::styled("▄", Style::new().fg(CAT_BODY)),
        ]),
        Line::from(vec![
            Span::styled("▄", Style::new().fg(CAT_BODY)),
            Span::styled("▀", Style::new().fg(CAT_EYE)),
            Span::styled("▀", Style::new().fg(CAT_NOSE)),
            Span::styled("▀", Style::new().fg(CAT_EYE)),
            Span::styled("▄", Style::new().fg(CAT_BODY)),
        ]),
        Line::from(vec![
            Span::styled("▗▗", Style::new().fg(CAT_BODY)),
            Span::raw(" "),
            Span::styled("▖▖", Style::new().fg(CAT_BODY)),
        ]),
    ]
}

fn walking_left_1() -> Vec<Line<'static>> {
    vec![
        Line::from(vec![
            Span::raw(" "),
            Span::styled("▄", Style::new().fg(CAT_BODY)),
            Span::raw(" "),
            Span::styled("▄", Style::new().fg(CAT_BODY)),
        ]),
        Line::from(vec![
            Span::styled("▄", Style::new().fg(CAT_BODY)),
            Span::styled("▀", Style::new().fg(CAT_EYE)),
            Span::styled("▀", Style::new().fg(CAT_NOSE)),
            Span::styled("▀", Style::new().fg(CAT_EYE)),
            Span::styled("▄", Style::new().fg(CAT_BODY)),
        ]),
        Line::from(vec![
            Span::styled("▗▗", Style::new().fg(CAT_BODY)),
            Span::raw(" "),
            Span::styled("▖▖", Style::new().fg(CAT_BODY)),
        ]),
    ]
}

fn walking_left_2() -> Vec<Line<'static>> {
    vec![
        Line::from(vec![
            Span::raw(" "),
            Span::styled("▄", Style::new().fg(CAT_BODY)),
            Span::raw(" "),
            Span::styled("▄", Style::new().fg(CAT_BODY)),
        ]),
        Line::from(vec![
            Span::styled("▄", Style::new().fg(CAT_BODY)),
            Span::styled("▀", Style::new().fg(CAT_EYE)),
            Span::styled("▀", Style::new().fg(CAT_NOSE)),
            Span::styled("▀", Style::new().fg(CAT_EYE)),
            Span::styled("▄", Style::new().fg(CAT_BODY)),
        ]),
        Line::from(vec![
            Span::styled("▖▖", Style::new().fg(CAT_BODY)),
            Span::raw(" "),
            Span::styled("▗▗", Style::new().fg(CAT_BODY)),
        ]),
    ]
}

/// Working sprite 1: cat facing right, arm down.
/// Feet use bg=CHAIR_COLOR to fill gap to chair below.
fn working_sprite_1() -> Vec<Line<'static>> {
    vec![
        Line::from(vec![
            Span::raw(" "),
            Span::styled("▄▄", Style::new().fg(CAT_BODY)),
        ]),
        Line::from(vec![
            Span::raw(" "),
            Span::styled("█", Style::new().fg(CAT_BODY)),
            Span::styled("▀", Style::new().fg(CAT_EYE)),
            Span::styled("╴", Style::new().fg(CAT_BODY)),
        ]),
        Line::from(vec![
            Span::raw(" "),
            Span::styled("▀▀", Style::new().fg(CAT_BODY).bg(CHAIR_COLOR)),
        ]),
    ]
}

/// Working sprite 2: cat facing right, arm extended.
/// Feet use bg=CHAIR_COLOR to fill gap to chair below.
fn working_sprite_2() -> Vec<Line<'static>> {
    vec![
        Line::from(vec![
            Span::raw(" "),
            Span::styled("▄▄", Style::new().fg(CAT_BODY)),
        ]),
        Line::from(vec![
            Span::raw(" "),
            Span::styled("█", Style::new().fg(CAT_BODY)),
            Span::styled("▀", Style::new().fg(CAT_EYE)),
            Span::styled("─", Style::new().fg(CAT_BODY)),
        ]),
        Line::from(vec![
            Span::raw(" "),
            Span::styled("▀▀", Style::new().fg(CAT_BODY).bg(CHAIR_COLOR)),
        ]),
    ]
}

fn working_sprite_3() -> Vec<Line<'static>> {
    vec![
        Line::from(vec![
            Span::raw(" "),
            Span::styled("▄▄", Style::new().fg(CAT_BODY)),
        ]),
        Line::from(vec![
            Span::raw(" "),
            Span::styled("█", Style::new().fg(CAT_BODY)),
            Span::styled("▀", Style::new().fg(CAT_EYE)),
            Span::styled("╶", Style::new().fg(CAT_BODY)),
        ]),
        Line::from(vec![
            Span::raw(" "),
            Span::styled("▀▀", Style::new().fg(CAT_BODY).bg(CHAIR_COLOR)),
        ]),
    ]
}

const DESK_COLOR: Color = Color::Indexed(137); // brown
const CHAIR_COLOR: Color = Color::Indexed(94); // dark brown

/// Desk: top plate + legs.
fn desk_sprite() -> Vec<Line<'static>> {
    vec![
        Line::from(Span::styled("████", Style::new().fg(DESK_COLOR))),
        Line::from(Span::styled("█  █", Style::new().fg(DESK_COLOR))),
    ]
}

/// Chair: full block.
fn chair_sprite() -> Vec<Line<'static>> {
    vec![
        Line::from(Span::styled("██", Style::new().fg(CHAIR_COLOR))),
    ]
}

const PAPER_COLOR: Color = Color::Indexed(255); // white

fn paper_sprite(running_count: usize) -> Vec<Line<'static>> {
    let height = match running_count {
        0 => 0,
        1 => 1,
        2..=3 => 2,
        _ => MAX_PAPER_HEIGHT as usize,
    };
    (0..height)
        .map(|_| Line::from(Span::styled("▐█▌", Style::new().fg(PAPER_COLOR))))
        .collect()
}

fn idle_motion(state: &AppState) -> IdleMotion {
    if state.cat_bob_timer == state.cat_idle_jump_tick {
        IdleMotion::Jump
    } else if state.cat_bob_timer == state.cat_idle_blink_tick {
        IdleMotion::Blink
    } else {
        IdleMotion::Rest
    }
}

fn idle_sprite(motion: IdleMotion) -> Vec<Line<'static>> {
    match motion {
        IdleMotion::Blink => sitting_sprite_blink(),
        IdleMotion::Jump | IdleMotion::Rest => sitting_sprite(),
    }
}

/// Draw cat, desk, chair, and papers.
/// `running_count` controls paper stack height.
///
/// Layout: all elements share the same baseline (bottom_area.y - 1).
/// Each element's bottom row sits on that baseline, growing upward.
///
/// Working state example:
/// ```text
///                     ▄▄
///                     █▀╴  ████
///                  ▄▄ ▀▀
/// ```
/// baseline row: chair ▄▄ + cat feet ▀▀ (cat feet on chair)
/// row above:    cat body + desk ████
/// row above:    cat ears
pub fn draw_cat(frame: &mut Frame, state: &AppState, bottom_area: Rect, running_count: usize) {
    let panel_width = bottom_area.width;
    // Baseline: the bottom-most row for all elements
    let baseline = bottom_area.y.saturating_sub(1);

    // --- Positions ---
    let desk_x = bottom_area.x + panel_width.saturating_sub(DESK_OFFSET + DESK_WIDTH);
    let chair_x = desk_x.saturating_sub(CHAIR_WIDTH + CHAIR_DESK_GAP);

    // --- Draw cat first (so desk/chair render on top if overlapping) ---
    let sprite_lines = match state.cat_state {
        CatState::Idle => idle_sprite(idle_motion(state)),
        CatState::WalkRight => {
            match state.cat_frame {
                1 => walking_right_1(),
                2 => walking_right_2(),
                _ => walking_right_1(),
            }
        }
        CatState::Working => {
            match state.cat_frame {
                1 => working_sprite_1(),
                2 => working_sprite_2(),
                3 => working_sprite_3(),
                _ => working_sprite_1(),
            }
        }
        CatState::WalkLeft => {
            match state.cat_frame {
                1 => walking_left_1(),
                2 => walking_left_2(),
                _ => walking_left_1(),
            }
        }
    };

    let sprite_height = sprite_lines.len() as u16;
    let cat_y = match state.cat_state {
        CatState::Working => {
            // Cat sits on top of chair: 1 row above baseline
            baseline.saturating_sub(sprite_height)
        }
        CatState::Idle if matches!(idle_motion(state), IdleMotion::Jump) => {
            baseline.saturating_sub(sprite_height)
        }
        _ => baseline.saturating_sub(sprite_height - 1),
    };
    let cat_x = bottom_area.x + state.cat_x;
    render_lines(frame, &sprite_lines, cat_x, cat_y);

    // --- Draw chair (always visible) ---
    let chair_lines = chair_sprite();
    let chair_height = chair_lines.len() as u16;
    let chair_y = baseline.saturating_sub(chair_height - 1);
    render_lines(frame, &chair_lines, chair_x, chair_y);

    // --- Draw desk (legs on baseline, top plate one row above) ---
    let desk_lines = desk_sprite();
    let desk_height = desk_lines.len() as u16;
    let desk_y = baseline.saturating_sub(desk_height - 1);
    render_lines(frame, &desk_lines, desk_x, desk_y);

    // --- Draw papers above desk ---
    if running_count > 0 {
        let papers = paper_sprite(running_count);
        if !papers.is_empty() {
            let paper_y = desk_y.saturating_sub(papers.len() as u16);
            let paper_x = desk_x + 1;
            render_lines(frame, &papers, paper_x, paper_y);
        }
    }
}

/// Helper to render a slice of Lines at given position, clipping to frame bounds.
fn render_lines(frame: &mut Frame, lines: &[Line<'_>], x: u16, start_y: u16) {
    for (i, line) in lines.iter().enumerate() {
        let y = start_y + i as u16;
        if y >= frame.area().height {
            continue;
        }
        let line_width: u16 = line.spans.iter().map(|s| s.content.width() as u16).sum();
        let available = frame.area().width.saturating_sub(x);
        if available == 0 {
            continue;
        }
        let w = line_width.min(available);
        let area = Rect::new(x, y, w, 1);
        frame.render_widget(line.clone(), area);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use ratatui::{Terminal, backend::TestBackend};

    /// Convert a sprite (Vec<Line>) to a plain string for visual inspection.
    fn sprite_to_string(lines: &[Line<'_>]) -> String {
        lines
            .iter()
            .map(|line| {
                line.spans.iter().map(|s| s.content.as_ref()).collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    // ── Individual sprite pattern tests ──

    #[test]
    fn sprite_sitting() {
        let s = sprite_to_string(&sitting_sprite());
        assert_eq!(s, [
            " ▄ ▄",
            "▄▀▀▀▄",
            " ▀ ▀",
        ].join("\n"));
    }

    #[test]
    fn sprite_walking_right_frame1() {
        let s = sprite_to_string(&walking_right_1());
        assert_eq!(s, [
            " ▄ ▄",
            "▄▀▀▀▄",
            "▖▖ ▗▗",
        ].join("\n"));
    }

    #[test]
    fn sprite_walking_right_frame2() {
        let s = sprite_to_string(&walking_right_2());
        assert_eq!(s, [
            " ▄ ▄",
            "▄▀▀▀▄",
            "▗▗ ▖▖",
        ].join("\n"));
    }

    #[test]
    fn sprite_working_frame1() {
        let s = sprite_to_string(&working_sprite_1());
        assert_eq!(s, [
            " ▄▄",
            " █▀╴",
            " ▀▀",
        ].join("\n"));
    }

    #[test]
    fn sprite_working_frame2() {
        let s = sprite_to_string(&working_sprite_2());
        assert_eq!(s, [
            " ▄▄",
            " █▀─",
            " ▀▀",
        ].join("\n"));
    }

    #[test]
    fn sprite_working_frame3() {
        let s = sprite_to_string(&working_sprite_3());
        assert_eq!(s, [
            " ▄▄",
            " █▀╶",
            " ▀▀",
        ].join("\n"));
    }

    #[test]
    fn sprite_desk() {
        let s = sprite_to_string(&desk_sprite());
        assert_eq!(s, [
            "████",
            "█  █",
        ].join("\n"));
    }

    #[test]
    fn sprite_chair() {
        let s = sprite_to_string(&chair_sprite());
        assert_eq!(s, "██");
    }

    #[test]
    fn sprite_paper_0() {
        assert_eq!(sprite_to_string(&paper_sprite(0)), "");
    }

    #[test]
    fn sprite_paper_1() {
        assert_eq!(sprite_to_string(&paper_sprite(1)), "▐█▌");
    }

    #[test]
    fn sprite_paper_2() {
        let s = sprite_to_string(&paper_sprite(2));
        assert_eq!(s, [
            "▐█▌",
            "▐█▌",
        ].join("\n"));
    }

    #[test]
    fn all_sprites_have_3_lines() {
        assert_eq!(sitting_sprite().len(), 3);
        assert_eq!(sitting_sprite_blink().len(), 3);
        assert_eq!(walking_right_1().len(), 3);
        assert_eq!(walking_right_2().len(), 3);
        assert_eq!(walking_left_1().len(), 3);
        assert_eq!(walking_left_2().len(), 3);
        assert_eq!(working_sprite_1().len(), 3);
        assert_eq!(working_sprite_2().len(), 3);
        assert_eq!(working_sprite_3().len(), 3);
    }

    #[test]
    fn desk_sprite_has_lines() {
        let desk = desk_sprite();
        assert!(!desk.is_empty());
    }

    #[test]
    fn paper_sprite_height_scales_with_count() {
        assert_eq!(paper_sprite(0).len(), 0);
        assert_eq!(paper_sprite(1).len(), 1);
        assert_eq!(paper_sprite(3).len(), 2);
        assert_eq!(paper_sprite(5).len(), 2);
    }

    #[test]
    fn sprite_sitting_blink() {
        let s = sprite_to_string(&sitting_sprite_blink());
        assert_eq!(s, [
            " ▄ ▄",
            "▄─▀─▄",
            " ▀ ▀",
        ].join("\n"));
    }

    #[test]
    fn idle_sprite_cycles_through_idle_poses() {
        assert_eq!(sprite_to_string(&idle_sprite(IdleMotion::Rest)), sprite_to_string(&sitting_sprite()));
        assert_eq!(sprite_to_string(&idle_sprite(IdleMotion::Jump)), sprite_to_string(&sitting_sprite()));
        assert_eq!(sprite_to_string(&idle_sprite(IdleMotion::Blink)), sprite_to_string(&sitting_sprite_blink()));
    }

    #[test]
    fn idle_motion_schedule_is_sparse_and_non_overlapping() {
        let state = AppState::new("%0".into());
        assert!(state.cat_idle_jump_tick < BOB_INTERVAL);
        assert!(state.cat_idle_blink_tick < BOB_INTERVAL);
        assert_ne!(state.cat_idle_jump_tick, state.cat_idle_blink_tick);
        assert!(state.cat_idle_jump_tick < state.cat_idle_blink_tick);
    }

    /// Helper: render draw_cat into a buffer and return as string for visual inspection.
    fn render_cat_scene(state: &AppState, running_count: usize, width: u16, height: u16) -> String {
        let backend = TestBackend::new(width, height);
        let mut terminal = Terminal::new(backend).unwrap();
        let bottom_y = height.saturating_sub(10);
        terminal
            .draw(|frame| {
                let bottom_area = Rect::new(0, bottom_y, width, 10);
                draw_cat(frame, state, bottom_area, running_count);
            })
            .unwrap();
        let buf = terminal.backend().buffer().clone();
        let area = buf.area;
        let mut lines = Vec::new();
        for y in area.y..area.y + area.height {
            let mut line = String::new();
            for x in area.x..area.x + area.width {
                line.push_str(buf[(x, y)].symbol());
            }
            lines.push(line.trim_end().to_string());
        }
        // Remove trailing empty lines
        while lines.last().is_some_and(|l| l.is_empty()) {
            lines.pop();
        }
        lines.join("\n")
    }

    #[test]
    fn snapshot_idle() {
        let state = AppState::new("%0".into());
        let output = render_cat_scene(&state, 0, 40, 14);
        let expected = [
            "",
            "  ▄ ▄",
            " ▄▀▀▀▄                              ████",
            "  ▀ ▀                            ██ █  █",
        ].join("\n");
        assert_eq!(output, expected);
    }

    #[test]
    fn snapshot_working() {
        let mut state = AppState::new("%0".into());
        state.cat_state = CatState::Working;
        let panel_width = 40u16;
        let working_width = CHAIR_WIDTH + 2;
        let stop_x = panel_width
            .saturating_sub(DESK_OFFSET + DESK_WIDTH + working_width);
        state.cat_x = stop_x;
        state.cat_frame = 1;
        let output = render_cat_scene(&state, 2, panel_width, 14);
        let expected = [
            "                                 ▄▄  ▐█▌",
            "                                 █▀╴ ▐█▌",
            "                                 ▀▀ ████",
            "                                 ██ █  █",
        ].join("\n");
        assert_eq!(output, expected);
    }

    #[test]
    fn snapshot_walking_right() {
        let mut state = AppState::new("%0".into());
        state.cat_state = CatState::WalkRight;
        state.cat_x = 15;
        state.cat_frame = 1;
        let output = render_cat_scene(&state, 1, 40, 14);
        let expected = [
            "",
            "                ▄ ▄                  ▐█▌",
            "               ▄▀▀▀▄                ████",
            "               ▖▖ ▗▗             ██ █  █",
        ].join("\n");
        assert_eq!(output, expected);
    }

    #[test]
    fn snapshot_walking_left() {
        let mut state = AppState::new("%0".into());
        state.cat_state = CatState::WalkLeft;
        state.cat_x = 15;
        state.cat_frame = 1;
        let output = render_cat_scene(&state, 0, 40, 14);
        let expected = [
            "",
            "                ▄ ▄",
            "               ▄▀▀▀▄                ████",
            "               ▗▗ ▖▖             ██ █  █",
        ].join("\n");
        assert_eq!(output, expected);
    }
}
