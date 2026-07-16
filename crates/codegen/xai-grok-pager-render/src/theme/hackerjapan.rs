//! Hacker Japan theme — neon cyberpunk / ネオ東京 CLI aesthetic.
//!
//! Deep near-black canvas with matrix-green, hot magenta, electric cyan,
//! and acid yellow accents. Designed for truecolor terminals; pairs with
//! the glitch wave animation in the accent column.

use ratatui::style::{Color, Modifier};

use super::tokyonight::Theme;

const fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color::Rgb(r, g, b)
}

/// Neon cyber palette — matrix green + magenta / cyan chromatic split.
#[allow(dead_code)]
pub mod palette {
    use super::*;

    // -- backgrounds (void black with subtle purple-green cast) ---------------
    pub const VOID: Color = rgb(6, 4, 12); // #06040c
    pub const ABYSS: Color = rgb(10, 8, 18); // #0a0812
    pub const PANEL: Color = rgb(16, 12, 28); // #100c1c
    pub const ELEVATED: Color = rgb(28, 18, 42); // #1c122a
    pub const HOVER: Color = rgb(40, 24, 58); // #28183a
    pub const VISUAL: Color = rgb(32, 20, 48); // #201430

    // -- text -----------------------------------------------------------------
    pub const FG: Color = rgb(220, 255, 230); // #dcffe6 mint-white
    pub const FG_DIM: Color = rgb(160, 200, 180); // #a0c8b4
    pub const MUTED: Color = rgb(90, 110, 100); // #5a6e64
    pub const SUBTLE: Color = rgb(55, 70, 65); // #374641
    pub const BRIGHT_MUTED: Color = rgb(120, 150, 135); // #789687

    // -- neon accents ---------------------------------------------------------
    pub const MATRIX: Color = rgb(0, 255, 136); // #00ff88
    pub const MATRIX_DIM: Color = rgb(0, 180, 100); // #00b464
    pub const MAGENTA: Color = rgb(255, 0, 170); // #ff00aa
    pub const MAGENTA_HOT: Color = rgb(255, 70, 200); // #ff46c8
    pub const CYAN: Color = rgb(0, 240, 255); // #00f0ff
    pub const CYAN_DIM: Color = rgb(0, 180, 200); // #00b4c8
    pub const YELLOW: Color = rgb(255, 230, 0); // #ffe600
    pub const ORANGE: Color = rgb(255, 140, 40); // #ff8c28
    pub const RED: Color = rgb(255, 50, 90); // #ff325a
    pub const PURPLE: Color = rgb(180, 80, 255); // #b450ff
    pub const LIME: Color = rgb(180, 255, 40); // #b4ff28
}
use palette::*;

impl Theme {
    /// Hacker Japan — neon cyber / matrix CLI theme.
    pub const fn hacker_japan() -> Self {
        Self {
            bg_base: VOID,
            bg_light: PANEL,
            bg_dark: ABYSS,
            bg_highlight: ELEVATED,
            bg_hover: HOVER,
            bg_terminal: VOID,

            accent_user: MATRIX,
            accent_assistant: MAGENTA,
            accent_thinking: MUTED,
            accent_tool: BRIGHT_MUTED,
            accent_system: CYAN,
            accent_error: RED,
            accent_success: MATRIX,
            accent_running: MAGENTA_HOT,
            accent_skill: CYAN_DIM,

            text_primary: FG,
            text_secondary: FG_DIM,

            gray_dim: SUBTLE,
            gray: MUTED,
            gray_bright: BRIGHT_MUTED,

            command: YELLOW,
            path: ORANGE,
            running: CYAN,
            warning: YELLOW,

            fuzzy_accent: MATRIX,

            accent_plan: YELLOW,
            accent_verify: PURPLE,
            accent_feedback: LIME,
            accent_remember: MATRIX_DIM,

            selection_border: rgb(0, 120, 90),
            hover_border: rgb(80, 40, 100),
            prompt_border: rgb(40, 60, 55),
            prompt_border_active: MATRIX,

            accent_model: CYAN,

            // Track dark, thumb neon-green so scrollbar pops.
            scrollbar_bg: ABYSS,
            scrollbar_fg: MATRIX_DIM,

            diff_delete_bg: rgb(50, 8, 20),
            diff_delete_fg: RED,
            diff_insert_bg: rgb(4, 40, 24),
            diff_insert_fg: MATRIX,
            diff_equal_fg: MUTED,
            diff_gutter_fg: MUTED,

            bg_visual: VISUAL,

            paste_bg: ABYSS,
            paste_fg: FG_DIM,
            paste_dim: MUTED,

            md_heading_h1: MATRIX,
            md_heading_h1_mod: Modifier::BOLD,
            md_heading_h2: MAGENTA_HOT,
            md_heading_h2_mod: Modifier::BOLD,
            md_heading_h3: CYAN,
            md_heading_h3_mod: Modifier::BOLD,
            md_heading_h4: YELLOW,
            md_heading_h4_mod: Modifier::BOLD.union(Modifier::ITALIC),
            md_heading_h5: PURPLE,
            md_heading_h5_mod: Modifier::BOLD,
            md_heading_h6: LIME,
            md_heading_h6_mod: Modifier::BOLD,
            md_code: CYAN,
            md_task_checked: MATRIX,
            md_task_unchecked: FG_DIM,
            md_muted: MUTED,
            md_code_bg: ABYSS,
            md_text: FG,
            link_fg: CYAN,
        }
    }
}

/// Glitch chromatic-aberration color for animated accent bars.
///
/// Combines the base wave brightness with short random RGB channel
/// displacements (matrix green ↔ magenta ↔ cyan) so running tools look
/// like a CRT / datamosh glitch rather than a smooth sine wave.
///
/// Returns an RGB color blended toward `base` by `brightness`, then
/// occasionally channel-shifted for the glitch spikes.
pub fn glitch_accent_color(
    tick: u64,
    row: u16,
    wave_rows: u16,
    speed: f32,
    base: Color,
    accent: Color,
) -> Color {
    use super::tokyonight::wave_brightness;
    use crate::render::color::blend_color;

    let brightness = wave_brightness(tick, row, wave_rows, speed);
    // Spike: hash tick+row into a deterministic "random" glitch strength.
    let h = tick
        .wrapping_mul(0x9E37_79B9)
        .wrapping_add(row as u64 * 0x85EB_CA6B);
    let spike = ((h >> 16) & 0xFF) as f32 / 255.0;
    // ~18% of cells get a hard glitch spike each frame.
    let glitch_on = spike > 0.82;
    let mut bright = brightness;
    if glitch_on {
        // Snap bright or nearly black for digital tear.
        bright = if (h & 1) == 0 { 1.0 } else { 0.08 };
    }

    let blended = blend_color(base, accent, bright).unwrap_or(accent);

    if !glitch_on {
        return blended;
    }

    // Chromatic split: push R toward magenta, G toward matrix, B toward cyan.
    let Color::Rgb(r, g, b) = blended else {
        return blended;
    };
    let phase = ((tick / 2 + row as u64) % 3) as u8;
    match phase {
        0 => Color::Rgb(r.saturating_add(80).min(255), g.saturating_sub(40), b),
        1 => Color::Rgb(r.saturating_sub(40), g.saturating_add(60).min(255), b),
        _ => Color::Rgb(r, g.saturating_sub(30), b.saturating_add(90).min(255)),
    }
}

/// Whether the active theme wants glitch accent animation.
#[inline]
pub fn wants_glitch_accents() -> bool {
    matches!(
        super::cache::current_kind(),
        super::ThemeKind::HackerJapan
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hacker_japan_is_dark() {
        assert!(Theme::hacker_japan().is_dark());
    }

    #[test]
    fn scrollbar_thumb_lighter_than_track() {
        let t = Theme::hacker_japan();
        let Color::Rgb(tr, tg, tb) = t.scrollbar_bg else {
            panic!("track must be rgb");
        };
        let Color::Rgb(hr, hg, hb) = t.scrollbar_fg else {
            panic!("thumb must be rgb");
        };
        let track = tr as i32 + tg as i32 + tb as i32;
        let thumb = hr as i32 + hg as i32 + hb as i32;
        assert!(
            thumb - track >= 30,
            "thumb Σ{thumb} must be ≥30 lighter than track Σ{track}"
        );
    }

    #[test]
    fn glitch_accent_returns_rgb() {
        let c = glitch_accent_color(
            42,
            3,
            16,
            0.15,
            Color::Rgb(0, 0, 0),
            Color::Rgb(0, 255, 136),
        );
        assert!(matches!(c, Color::Rgb(_, _, _)));
    }
}
