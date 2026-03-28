//! Shared Yanqu design tokens and styling helpers.

#![allow(dead_code)]

use iced::widget::{button, checkbox, container, rule, scrollable, text_editor, text_input};
use iced::{border, Background, Border, Color, Shadow, Theme, Vector};

/// Historical module name kept so the rest of the UI can reuse existing imports.
pub mod darcula {
    use super::*;

    // ── Deep dark stage ────────────────────────────────────────────────────────
    pub const BG_MAIN: Color = Color::from_rgb(0.051, 0.067, 0.090);    // #0D1117
    pub const BG_RAISED: Color = Color::from_rgb(0.110, 0.133, 0.157);  // #1C2228
    pub const BG_PANEL: Color = Color::from_rgb(0.086, 0.106, 0.133);   // #161B22
    pub const BG_TOOLBAR: Color = Color::from_rgb(0.071, 0.090, 0.114); // #12171D
    pub const BG_EDITOR: Color = Color::from_rgb(0.059, 0.075, 0.098);  // #0F1319
    pub const BG_NAV: Color = Color::from_rgb(0.071, 0.090, 0.114);     // #12171D
    pub const BG_RAIL: Color = Color::from_rgb(0.051, 0.067, 0.090);    // #0D1117
    pub const BG_STATUS: Color = Color::from_rgb(0.043, 0.055, 0.075);  // #0B0E13
    pub const BG_TAB_ACTIVE: Color = Color::from_rgb(0.086, 0.106, 0.133);  // #161B22
    pub const BG_TAB_HOVER: Color = Color::from_rgb(0.071, 0.090, 0.114);   // #12171D

    // ── Text hierarchy ─────────────────────────────────────────────────────────
    pub const TEXT_PRIMARY: Color = Color::from_rgb(0.902, 0.929, 0.953);   // #E6EDF3
    pub const TEXT_SECONDARY: Color = Color::from_rgb(0.545, 0.580, 0.620); // #8B949E
    pub const TEXT_DISABLED: Color = Color::from_rgb(0.282, 0.310, 0.345);  // #484F58

    // ── Accent — vibrant mint; only 1-2 focal words glow ──────────────────────
    pub const ACCENT: Color = Color::from_rgb(0.224, 0.816, 0.769);      // #39D0C4
    pub const ACCENT_WEAK: Color = Color::from_rgb(0.094, 0.176, 0.169); // #182C2B
    pub const BRAND: Color = Color::from_rgb(0.345, 0.651, 1.0);         // #58A6FF
    pub const BRAND_WEAK: Color = Color::from_rgb(0.110, 0.165, 0.251);  // #1C2A40
    pub const SUCCESS: Color = Color::from_rgb(0.247, 0.718, 0.314);     // #3FB950
    pub const WARNING: Color = Color::from_rgb(0.824, 0.600, 0.133);     // #D29922
    pub const DANGER: Color = Color::from_rgb(0.973, 0.318, 0.286);      // #F85149

    pub const STATUS_ADDED: Color = SUCCESS;
    pub const STATUS_MODIFIED: Color = BRAND;
    pub const STATUS_DELETED: Color = DANGER;
    pub const STATUS_RENAMED: Color = Color::from_rgb(0.212, 0.816, 0.769); // #36D0C4
    pub const STATUS_UNVERSIONED: Color = Color::from_rgb(0.431, 0.463, 0.502); // #6E7681

    // ── Selection / highlight ──────────────────────────────────────────────────
    pub const SELECTION_BG: Color = Color::from_rgb(0.149, 0.278, 0.204);      // #264734
    pub const SELECTION_INACTIVE: Color = Color::from_rgb(0.118, 0.176, 0.141); // #1E2D24
    pub const HIGHLIGHT_BG: Color = Color::from_rgb(0.110, 0.165, 0.251);       // #1C2A40

    // ── Borders / separators ───────────────────────────────────────────────────
    pub const BORDER: Color = Color::from_rgb(0.188, 0.212, 0.239);    // #30363D
    pub const SEPARATOR: Color = Color::from_rgb(0.129, 0.149, 0.176); // #21262D

    // ── Diff surfaces (dark variants) ─────────────────────────────────────────
    pub const DIFF_ADDED_BG: Color = Color::from_rgb(0.071, 0.157, 0.110);   // #122818
    pub const DIFF_MODIFIED_BG: Color = Color::from_rgb(0.071, 0.122, 0.188); // #121F30
    pub const DIFF_DELETED_BG: Color = Color::from_rgb(0.180, 0.063, 0.075);  // #2E1013
}

pub mod spacing {
    pub const XS: f32 = 4.0;
    pub const SM: f32 = 8.0;
    pub const MD: f32 = 16.0;
    pub const LG: f32 = 24.0;
}

pub mod density {
    pub const INLINE_GAP: f32 = 8.0;
    pub const CONTROL_GAP: f32 = 12.0;
    pub const TOOLBAR_PADDING: [u16; 2] = [6, 12];
    pub const SECONDARY_BAR_PADDING: [u16; 2] = [5, 12];
    pub const PANE_PADDING: [u16; 2] = [12, 12];
    pub const STATUS_PADDING: [u16; 2] = [4, 12];
    pub const STANDARD_CONTROL_HEIGHT: f32 = 32.0;
    pub const COMPACT_CONTROL_HEIGHT: f32 = 28.0;
    pub const COMPACT_CHIP_PADDING: [u16; 2] = [3, 8];
}

pub mod radius {
    pub const SM: f32 = 4.0;
    pub const MD: f32 = 8.0;
    pub const LG: f32 = 12.0;
}

/// Semantic text styles — 5-tier hierarchy for consistent typography.
pub mod typography {
    /// Display / hero text — repo name, modal titles.
    pub const DISPLAY_SIZE: u16 = 16;
    pub const DISPLAY_WEIGHT: iced::font::Weight = iced::font::Weight::Bold;

    /// Title — section headers, dialog headings.
    pub const TITLE_SIZE: u16 = 13;
    pub const TITLE_WEIGHT: iced::font::Weight = iced::font::Weight::Semibold;

    /// Body — primary content, list items, descriptions.
    pub const BODY_SIZE: u16 = 12;
    pub const BODY_WEIGHT: iced::font::Weight = iced::font::Weight::Normal;

    /// Caption — secondary info, timestamps, metadata.
    pub const CAPTION_SIZE: u16 = 11;
    pub const CAPTION_WEIGHT: iced::font::Weight = iced::font::Weight::Normal;

    /// Micro — badges, chips, compact labels.
    pub const MICRO_SIZE: u16 = 10;
    pub const MICRO_WEIGHT: iced::font::Weight = iced::font::Weight::Medium;
}

/// Motion design tokens — transition durations and easing references.
/// Iced doesn't support custom easing yet, but these constants serve as
/// documentation and future-proofing for animation support.
pub mod motion {
    /// Fast micro-interactions: button press, checkbox toggle.
    pub const DURATION_FAST: u32 = 120; // ms
    /// Standard transitions: hover state, panel switch.
    pub const DURATION_NORMAL: u32 = 200; // ms
    /// Emphasized transitions: modal appear, view swap.
    pub const DURATION_EMPHASIZED: u32 = 350; // ms

    /// Standard easing curve (Material 3 equivalent).
    pub const EASING_STANDARD: &str = "cubic-bezier(0.4, 0.0, 0.2, 1)";
    /// Deceleration easing — elements entering screen.
    pub const EASING_DECEL: &str = "cubic-bezier(0.0, 0.0, 0.2, 1)";
    /// Acceleration easing — elements leaving screen.
    pub const EASING_ACCEL: &str = "cubic-bezier(0.4, 0.0, 1.0, 1.0)";
}

pub mod layout {
    pub const WINDOW_DEFAULT_WIDTH: f32 = 1280.0;
    pub const WINDOW_DEFAULT_HEIGHT: f32 = 800.0;
    pub const WINDOW_MIN_WIDTH: f32 = 800.0;
    pub const WINDOW_MIN_HEIGHT: f32 = 600.0;

    pub const SIDEBAR_WIDTH: f32 = 192.0;
    pub const RAIL_WIDTH: f32 = 56.0;
    pub const TOP_BAR_HEIGHT: f32 = 30.0;
    pub const SECONDARY_BAR_HEIGHT: f32 = 28.0;
    pub const STATUS_BAR_HEIGHT: f32 = 24.0;
    pub const CONTROL_HEIGHT: f32 = 26.0;
    pub const SHELL_GAP: f32 = 8.0;
    pub const SHELL_PADDING: f32 = 16.0;
    pub const SECTION_PADDING: f32 = 16.0;
}

#[derive(Debug, Clone, Copy)]
pub enum Surface {
    Root,
    Nav,
    Rail,
    Toolbar,
    Status,
    Panel,
    Raised,
    Editor,
    Accent,
    Selection,
    Success,
    Warning,
    Danger,
    ToolbarField,
    ListRow,
    ListSelection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonTone {
    Primary,
    Secondary,
    Ghost,
    TabActive,
    TabInactive,
    RailActive,
    RailInactive,
    Success,
    Warning,
    Danger,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonChrome {
    Standard,
    Tab,
    Rail,
    ToolbarIcon,
    SplitLeft,
    SplitRight,
}

#[derive(Debug, Clone, Copy)]
pub enum BadgeTone {
    Neutral,
    Accent,
    Success,
    Warning,
    Danger,
}

fn mix(base: Color, overlay: Color, amount: f32) -> Color {
    let amount = amount.clamp(0.0, 1.0);
    Color {
        r: (base.r * (1.0 - amount)) + (overlay.r * amount),
        g: (base.g * (1.0 - amount)) + (overlay.g * amount),
        b: (base.b * (1.0 - amount)) + (overlay.b * amount),
        a: (base.a * (1.0 - amount)) + (overlay.a * amount),
    }
}

fn soft_shadow(alpha: f32, y: f32, blur: f32) -> Shadow {
    Shadow {
        color: Color {
            a: alpha,
            ..Color::BLACK
        },
        offset: Vector::new(0.0, y),
        blur_radius: blur,
    }
}

/// Colored glow — used for accent-lit surfaces (motionsites signature effect).
fn accent_glow(color: Color, alpha: f32, blur: f32) -> Shadow {
    Shadow {
        color: Color { a: alpha, ..color },
        offset: Vector::new(0.0, 2.0),
        blur_radius: blur,
    }
}

fn surface_background(surface: Surface) -> Color {
    match surface {
        Surface::Root => darcula::BG_MAIN,
        Surface::Nav => darcula::BG_NAV,
        Surface::Rail => darcula::BG_RAIL,
        Surface::Toolbar => darcula::BG_TOOLBAR,
        Surface::Status => darcula::BG_STATUS,
        Surface::Panel => darcula::BG_PANEL,
        Surface::Raised => darcula::BG_RAISED,
        Surface::Editor => darcula::BG_EDITOR,
        // Accent surface: BG_PANEL tinted with dark accent → subtle teal panel
        Surface::Accent => mix(darcula::BG_PANEL, darcula::ACCENT_WEAK, 0.80),
        Surface::Selection => mix(darcula::BG_PANEL, darcula::SELECTION_BG, 0.70),
        // Status surfaces: mix accent-color into editor bg at low amount
        Surface::Success => mix(darcula::BG_EDITOR, darcula::SUCCESS, 0.14),
        Surface::Warning => mix(darcula::BG_EDITOR, darcula::WARNING, 0.16),
        Surface::Danger => mix(darcula::BG_EDITOR, darcula::DANGER, 0.12),
        // Input field: slightly raised vs panel
        Surface::ToolbarField => mix(darcula::BG_EDITOR, darcula::ACCENT_WEAK, 0.60),
        Surface::ListRow => darcula::BG_PANEL,
        Surface::ListSelection => mix(darcula::BG_PANEL, darcula::SELECTION_BG, 0.75),
    }
}

/// Create the shared application theme.
pub fn darcula_theme() -> Theme {
    use iced::theme::Palette;

    Theme::custom(
        "YanquWorkbench".to_string(),
        Palette {
            background: darcula::BG_MAIN,
            text: darcula::TEXT_PRIMARY,
            primary: darcula::ACCENT,
            success: darcula::SUCCESS,
            warning: darcula::WARNING,
            danger: darcula::DANGER,
        },
    )
}

pub fn panel_style(surface: Surface) -> impl Fn(&Theme) -> container::Style {
    move |_theme| {
        let (background, border_width, border_color, radius, shadow) = match surface {
            Surface::Root => (
                surface_background(surface),
                0.0,
                Color::TRANSPARENT,
                0.0,
                Shadow::default(),
            ),
            Surface::Editor => (
                surface_background(surface),
                1.0,
                darcula::BORDER.scale_alpha(0.88),
                radius::LG,
                Shadow::default(),
            ),
            Surface::Toolbar | Surface::Nav | Surface::Rail | Surface::Status => (
                surface_background(surface),
                0.0,
                Color::TRANSPARENT,
                0.0,
                Shadow::default(),
            ),
            Surface::ToolbarField => (
                mix(darcula::BG_PANEL, darcula::ACCENT_WEAK, 0.18),
                1.0,
                darcula::SEPARATOR.scale_alpha(0.84),
                radius::MD,
                Shadow::default(),
            ),
            Surface::ListRow => (
                darcula::BG_PANEL,
                1.0,
                darcula::SEPARATOR.scale_alpha(0.20),
                radius::MD,
                Shadow::default(),
            ),
            Surface::ListSelection => (
                mix(darcula::BG_PANEL, darcula::SELECTION_BG, 0.92),
                1.0,
                darcula::ACCENT.scale_alpha(0.26),
                radius::MD,
                Shadow::default(),
            ),
            Surface::Panel => (
                surface_background(surface),
                1.0,
                darcula::BORDER.scale_alpha(0.70),
                radius::LG,
                soft_shadow(0.28, 4.0, 16.0),
            ),
            Surface::Raised => (
                surface_background(surface),
                1.0,
                darcula::SEPARATOR.scale_alpha(0.82),
                radius::MD,
                Shadow::default(),
            ),
            Surface::Accent => (
                surface_background(surface),
                1.0,
                darcula::ACCENT.scale_alpha(0.28),
                radius::LG,
                Shadow::default(),
            ),
            Surface::Selection => (
                surface_background(surface),
                1.0,
                darcula::ACCENT.scale_alpha(0.40),
                radius::LG,
                Shadow::default(),
            ),
            Surface::Success => (
                surface_background(surface),
                1.0,
                darcula::SUCCESS.scale_alpha(0.26),
                radius::LG,
                Shadow::default(),
            ),
            Surface::Warning => (
                surface_background(surface),
                1.0,
                darcula::WARNING.scale_alpha(0.28),
                radius::LG,
                Shadow::default(),
            ),
            Surface::Danger => (
                surface_background(surface),
                1.0,
                darcula::DANGER.scale_alpha(0.24),
                radius::LG,
                Shadow::default(),
            ),
        };

        container::Style {
            background: Some(Background::Color(background)),
            border: Border {
                width: border_width,
                color: border_color,
                radius: radius.into(),
            },
            shadow,
            ..Default::default()
        }
    }
}

pub fn frame_style(surface: Surface) -> impl Fn(&Theme) -> container::Style {
    move |_theme| {
        let (border_width, border_color, shadow) = match surface {
            Surface::Toolbar => (0.0, Color::TRANSPARENT, soft_shadow(0.03, 3.0, 10.0)),
            Surface::Nav => (1.0, darcula::SEPARATOR.scale_alpha(0.60), Shadow::default()),
            Surface::Rail => (1.0, darcula::SEPARATOR.scale_alpha(0.72), Shadow::default()),
            Surface::Status => (1.0, darcula::SEPARATOR.scale_alpha(0.60), Shadow::default()),
            _ => (0.0, Color::TRANSPARENT, Shadow::default()),
        };

        container::Style {
            background: Some(Background::Color(surface_background(surface))),
            border: Border {
                width: border_width,
                color: border_color,
                radius: 0.0.into(),
            },
            shadow,
            ..Default::default()
        }
    }
}

pub fn badge_style(tone: BadgeTone) -> impl Fn(&Theme) -> container::Style {
    move |_theme| {
        let (color, border_color) = match tone {
            BadgeTone::Neutral => (
                mix(darcula::BG_MAIN, darcula::BG_PANEL, 0.70),
                darcula::BORDER.scale_alpha(0.92),
            ),
            BadgeTone::Accent => (
                mix(darcula::BG_PANEL, darcula::ACCENT_WEAK, 0.96),
                darcula::ACCENT.scale_alpha(0.24),
            ),
            BadgeTone::Success => (
                mix(darcula::BG_PANEL, darcula::SUCCESS, 0.12),
                darcula::SUCCESS.scale_alpha(0.24),
            ),
            BadgeTone::Warning => (
                mix(darcula::BG_PANEL, darcula::WARNING, 0.16),
                darcula::WARNING.scale_alpha(0.26),
            ),
            BadgeTone::Danger => (
                mix(darcula::BG_PANEL, darcula::DANGER, 0.10),
                darcula::DANGER.scale_alpha(0.24),
            ),
        };

        container::Style {
            background: Some(Background::Color(color)),
            border: Border {
                width: 1.0,
                color: border_color,
                radius: radius::MD.into(),
            },
            ..Default::default()
        }
    }
}

pub fn button_style(tone: ButtonTone) -> impl Fn(&Theme, button::Status) -> button::Style {
    button_style_for(tone, ButtonChrome::Standard)
}

fn subtle_button(tone: ButtonTone) -> bool {
    matches!(
        tone,
        ButtonTone::Ghost | ButtonTone::TabInactive | ButtonTone::RailInactive
    )
}

fn button_radius(chrome: ButtonChrome) -> border::Radius {
    match chrome {
        ButtonChrome::Standard => border::Radius::new(radius::LG),
        ButtonChrome::Tab => border::Radius::new(radius::MD),
        ButtonChrome::Rail => border::Radius::new(13.0),
        ButtonChrome::ToolbarIcon => border::Radius::new(10.0),
        ButtonChrome::SplitLeft => border::Radius::default()
            .top_left(radius::LG)
            .bottom_left(radius::LG),
        ButtonChrome::SplitRight => border::Radius::default()
            .top_right(radius::LG)
            .bottom_right(radius::LG),
    }
}

fn button_border_width(tone: ButtonTone, chrome: ButtonChrome, status: button::Status) -> f32 {
    match status {
        button::Status::Active => {
            if subtle_button(tone) {
                match chrome {
                    ButtonChrome::Tab | ButtonChrome::ToolbarIcon => 1.0,
                    ButtonChrome::SplitLeft | ButtonChrome::SplitRight => 1.0,
                    _ => 0.0,
                }
            } else {
                1.0
            }
        }
        button::Status::Hovered | button::Status::Pressed => 1.0,
        button::Status::Disabled => match tone {
            ButtonTone::TabInactive if chrome == ButtonChrome::Tab => 1.0,
            ButtonTone::Ghost | ButtonTone::RailInactive => 0.0,
            ButtonTone::TabInactive => 0.0,
            _ => 1.0,
        },
    }
}

fn button_shadow(tone: ButtonTone, chrome: ButtonChrome, status: button::Status) -> Shadow {
    match status {
        button::Status::Active => match tone {
            // Mint accent glow — motionsites signature effect on dark stage
            ButtonTone::Primary => accent_glow(darcula::ACCENT, 0.38, 14.0),
            ButtonTone::Secondary
            | ButtonTone::Success
            | ButtonTone::Warning
            | ButtonTone::Danger => soft_shadow(0.20, 2.0, 8.0),
            ButtonTone::TabActive if chrome == ButtonChrome::Tab => soft_shadow(0.10, 1.0, 4.0),
            _ => Shadow::default(),
        },
        button::Status::Hovered => match tone {
            ButtonTone::Primary => accent_glow(darcula::ACCENT, 0.52, 20.0),
            ButtonTone::Secondary
            | ButtonTone::Success
            | ButtonTone::Warning
            | ButtonTone::Danger => soft_shadow(0.28, 3.0, 10.0),
            ButtonTone::Ghost
                if matches!(
                    chrome,
                    ButtonChrome::ToolbarIcon | ButtonChrome::SplitLeft | ButtonChrome::SplitRight
                ) =>
            {
                soft_shadow(0.12, 2.0, 6.0)
            }
            _ => Shadow::default(),
        },
        button::Status::Pressed => Shadow {
            offset: Vector::new(0.0, 1.0),
            blur_radius: 1.0,
            ..Shadow::default()
        },
        button::Status::Disabled => Shadow::default(),
    }
}

pub fn button_style_for(
    tone: ButtonTone,
    chrome: ButtonChrome,
) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let passive_text = mix(darcula::TEXT_SECONDARY, darcula::TEXT_PRIMARY, 0.24);
        let (base_background, text_color, base_border) = match tone {
            ButtonTone::Primary => (
                mix(darcula::ACCENT, darcula::BRAND, 0.12),
                Color::WHITE,
                mix(darcula::ACCENT, darcula::BRAND, 0.20),
            ),
            ButtonTone::Secondary => (
                mix(darcula::BG_PANEL, darcula::ACCENT_WEAK, 0.26),
                darcula::TEXT_PRIMARY,
                mix(darcula::BORDER, darcula::ACCENT.scale_alpha(0.35), 0.30),
            ),
            ButtonTone::Ghost => (Color::TRANSPARENT, passive_text, Color::TRANSPARENT),
            ButtonTone::TabActive => (
                mix(darcula::BG_PANEL, darcula::ACCENT_WEAK, 0.96),
                darcula::TEXT_PRIMARY,
                darcula::ACCENT.scale_alpha(0.26),
            ),
            ButtonTone::TabInactive => (
                if chrome == ButtonChrome::Tab {
                    mix(darcula::BG_PANEL, darcula::BG_MAIN, 0.34)
                } else {
                    Color::TRANSPARENT
                },
                passive_text,
                if chrome == ButtonChrome::Tab {
                    darcula::SEPARATOR.scale_alpha(0.74)
                } else {
                    Color::TRANSPARENT
                },
            ),
            ButtonTone::RailActive => (
                mix(darcula::BG_PANEL, darcula::ACCENT_WEAK, 0.98),
                darcula::ACCENT,
                darcula::ACCENT.scale_alpha(0.22),
            ),
            ButtonTone::RailInactive => (Color::TRANSPARENT, passive_text, Color::TRANSPARENT),
            ButtonTone::Success => (darcula::SUCCESS, Color::WHITE, darcula::SUCCESS),
            ButtonTone::Warning => (darcula::WARNING, Color::WHITE, darcula::WARNING),
            ButtonTone::Danger => (darcula::DANGER, Color::WHITE, darcula::DANGER),
        };

        let (background, border_color, resolved_text) = match status {
            button::Status::Active => (base_background, base_border, text_color),
            button::Status::Hovered => (
                if subtle_button(tone) {
                    match chrome {
                        ButtonChrome::Tab => mix(darcula::BG_PANEL, darcula::ACCENT_WEAK, 0.65),
                        ButtonChrome::ToolbarIcon => {
                            mix(darcula::BG_PANEL, darcula::ACCENT_WEAK, 0.58)
                        }
                        ButtonChrome::SplitLeft | ButtonChrome::SplitRight => {
                            mix(darcula::BG_PANEL, darcula::ACCENT_WEAK, 0.55)
                        }
                        ButtonChrome::Rail => mix(darcula::BG_RAISED, darcula::ACCENT_WEAK, 0.60),
                        ButtonChrome::Standard => {
                            mix(darcula::BG_RAISED, darcula::ACCENT_WEAK, 0.40)
                        }
                    }
                } else {
                    // On dark bg: lighten slightly instead of darken
                    mix(base_background, Color::WHITE, 0.08)
                },
                if subtle_button(tone) {
                    darcula::ACCENT.scale_alpha(0.40)
                } else {
                    mix(base_border, Color::WHITE, 0.10)
                },
                if subtle_button(tone) {
                    mix(darcula::TEXT_PRIMARY, darcula::ACCENT, 0.40)
                } else {
                    text_color
                },
            ),
            button::Status::Pressed => (
                if subtle_button(tone) {
                    match chrome {
                        ButtonChrome::Tab => mix(darcula::BG_RAISED, darcula::ACCENT_WEAK, 0.80),
                        ButtonChrome::ToolbarIcon => {
                            mix(darcula::BG_RAISED, darcula::ACCENT_WEAK, 0.75)
                        }
                        ButtonChrome::SplitLeft | ButtonChrome::SplitRight => {
                            mix(darcula::BG_RAISED, darcula::ACCENT_WEAK, 0.70)
                        }
                        ButtonChrome::Rail => mix(darcula::BG_RAISED, darcula::ACCENT_WEAK, 0.90),
                        ButtonChrome::Standard => {
                            mix(darcula::BG_RAISED, darcula::ACCENT_WEAK, 0.65)
                        }
                    }
                } else {
                    mix(base_background, Color::BLACK, 0.18)
                },
                if subtle_button(tone) {
                    darcula::ACCENT
                } else {
                    mix(base_border, Color::WHITE, 0.06)
                },
                if subtle_button(tone) {
                    darcula::ACCENT
                } else {
                    text_color
                },
            ),
            button::Status::Disabled => match tone {
                ButtonTone::Ghost | ButtonTone::RailInactive => (
                    Color::TRANSPARENT,
                    Color::TRANSPARENT,
                    darcula::TEXT_DISABLED,
                ),
                ButtonTone::TabInactive => (
                    if chrome == ButtonChrome::Tab {
                        mix(darcula::BG_PANEL, darcula::BG_MAIN, 0.50)
                    } else {
                        Color::TRANSPARENT
                    },
                    if chrome == ButtonChrome::Tab {
                        darcula::SEPARATOR.scale_alpha(0.46)
                    } else {
                        Color::TRANSPARENT
                    },
                    darcula::TEXT_DISABLED,
                ),
                ButtonTone::Primary => (
                    mix(darcula::BG_RAISED, darcula::ACCENT_WEAK, 0.60),
                    darcula::ACCENT.scale_alpha(0.16),
                    mix(darcula::TEXT_DISABLED, darcula::TEXT_PRIMARY, 0.18),
                ),
                ButtonTone::Secondary => (
                    darcula::BG_RAISED,
                    darcula::SEPARATOR.scale_alpha(0.56),
                    darcula::TEXT_DISABLED,
                ),
                ButtonTone::TabActive => (
                    mix(darcula::BG_RAISED, darcula::ACCENT_WEAK, 0.55),
                    darcula::ACCENT.scale_alpha(0.18),
                    darcula::TEXT_DISABLED,
                ),
                ButtonTone::RailActive => (
                    mix(darcula::BG_RAISED, darcula::ACCENT_WEAK, 0.55),
                    darcula::ACCENT.scale_alpha(0.16),
                    mix(darcula::TEXT_DISABLED, darcula::ACCENT, 0.16),
                ),
                ButtonTone::Success | ButtonTone::Warning | ButtonTone::Danger => (
                    mix(darcula::BG_PANEL, base_background, 0.42),
                    mix(darcula::SEPARATOR, base_border, 0.26),
                    darcula::TEXT_DISABLED,
                ),
            },
        };

        button::Style {
            background: Some(Background::Color(background)),
            border: Border {
                width: button_border_width(tone, chrome, status),
                color: border_color,
                radius: button_radius(chrome),
            },
            shadow: button_shadow(tone, chrome, status),
            text_color: resolved_text,
            ..Default::default()
        }
    }
}

pub fn text_input_style() -> impl Fn(&Theme, text_input::Status) -> text_input::Style {
    move |_theme, status| {
        let (background, border, value, icon) = match status {
            text_input::Status::Active => (
                mix(darcula::BG_EDITOR, darcula::ACCENT_WEAK, 0.55),
                darcula::BORDER.scale_alpha(0.80),
                darcula::TEXT_PRIMARY,
                darcula::TEXT_SECONDARY,
            ),
            text_input::Status::Hovered => (
                mix(darcula::BG_EDITOR, darcula::ACCENT_WEAK, 0.65),
                darcula::ACCENT.scale_alpha(0.35),
                darcula::TEXT_PRIMARY,
                darcula::TEXT_SECONDARY,
            ),
            text_input::Status::Focused { .. } => (
                mix(darcula::BG_EDITOR, darcula::ACCENT_WEAK, 0.72),
                darcula::ACCENT.scale_alpha(0.80),
                darcula::TEXT_PRIMARY,
                darcula::ACCENT,
            ),
            text_input::Status::Disabled => (
                darcula::BG_MAIN,
                darcula::SEPARATOR.scale_alpha(0.60),
                darcula::TEXT_DISABLED,
                darcula::TEXT_DISABLED,
            ),
        };

        text_input::Style {
            background: Background::Color(background),
            border: Border {
                width: 1.0,
                color: border,
                radius: radius::LG.into(),
            },
            icon,
            placeholder: darcula::TEXT_DISABLED,
            value,
            selection: darcula::SELECTION_BG,
        }
    }
}

pub fn text_editor_style() -> impl Fn(&Theme, text_editor::Status) -> text_editor::Style {
    move |_theme, status| {
        let (background, border, value) = match status {
            text_editor::Status::Active => (
                mix(darcula::BG_EDITOR, darcula::ACCENT_WEAK, 0.55),
                darcula::BORDER.scale_alpha(0.80),
                darcula::TEXT_PRIMARY,
            ),
            text_editor::Status::Hovered => (
                mix(darcula::BG_EDITOR, darcula::ACCENT_WEAK, 0.65),
                darcula::ACCENT.scale_alpha(0.35),
                darcula::TEXT_PRIMARY,
            ),
            text_editor::Status::Focused { .. } => (
                mix(darcula::BG_EDITOR, darcula::ACCENT_WEAK, 0.72),
                darcula::ACCENT.scale_alpha(0.80),
                darcula::TEXT_PRIMARY,
            ),
            text_editor::Status::Disabled => (
                darcula::BG_MAIN,
                darcula::SEPARATOR.scale_alpha(0.60),
                darcula::TEXT_DISABLED,
            ),
        };

        text_editor::Style {
            background: Background::Color(background),
            border: Border {
                width: 1.0,
                color: border,
                radius: radius::LG.into(),
            },
            placeholder: darcula::TEXT_DISABLED,
            value,
            selection: darcula::SELECTION_BG,
        }
    }
}

pub fn scrollable_style() -> impl Fn(&Theme, scrollable::Status) -> scrollable::Style {
    move |_theme, status| {
        let idle_scroller = mix(darcula::TEXT_DISABLED, darcula::BORDER, 0.50).scale_alpha(0.18);
        let (rail_background, rail_border, scroller_color) = match status {
            scrollable::Status::Active { .. } => (
                Background::Color(Color::TRANSPARENT),
                Color::TRANSPARENT,
                idle_scroller,
            ),
            scrollable::Status::Hovered { .. } => (
                Background::Color(Color::TRANSPARENT),
                Color::TRANSPARENT,
                mix(darcula::TEXT_DISABLED, darcula::ACCENT, 0.12).scale_alpha(0.28),
            ),
            scrollable::Status::Dragged { .. } => (
                Background::Color(Color::TRANSPARENT),
                Color::TRANSPARENT,
                darcula::ACCENT.scale_alpha(0.52),
            ),
        };

        scrollable::Style {
            container: container::Style::default(),
            vertical_rail: scrollable::Rail {
                background: Some(rail_background),
                border: Border {
                    width: 0.0,
                    color: rail_border,
                    radius: radius::MD.into(),
                },
                scroller: scrollable::Scroller {
                    background: Background::Color(scroller_color),
                    border: Border {
                        width: 0.0,
                        color: rail_border,
                        radius: radius::MD.into(),
                    },
                },
            },
            horizontal_rail: scrollable::Rail {
                background: Some(rail_background),
                border: Border {
                    width: 0.0,
                    color: rail_border,
                    radius: radius::MD.into(),
                },
                scroller: scrollable::Scroller {
                    background: Background::Color(scroller_color),
                    border: Border {
                        width: 0.0,
                        color: rail_border,
                        radius: radius::MD.into(),
                    },
                },
            },
            gap: None,
            auto_scroll: scrollable::AutoScroll {
                background: Background::Color(mix(darcula::BG_RAISED, darcula::ACCENT_WEAK, 0.70)),
                border: Border {
                    width: 1.0,
                    color: darcula::ACCENT.scale_alpha(0.28),
                    radius: radius::MD.into(),
                },
                shadow: soft_shadow(0.20, 3.0, 8.0),
                icon: darcula::TEXT_SECONDARY,
            },
        }
    }
}

pub fn checkbox_style() -> impl Fn(&Theme, checkbox::Status) -> checkbox::Style {
    move |_theme, status| match status {
        checkbox::Status::Active { is_checked } => checkbox_base_style(is_checked, false, false),
        checkbox::Status::Hovered { is_checked } => checkbox_base_style(is_checked, true, false),
        checkbox::Status::Disabled { is_checked } => checkbox_base_style(is_checked, false, true),
    }
}

fn checkbox_base_style(is_checked: bool, hovered: bool, disabled: bool) -> checkbox::Style {
    let background = if is_checked {
        if disabled {
            mix(darcula::ACCENT, darcula::BG_MAIN, 0.52)
        } else if hovered {
            mix(darcula::ACCENT, Color::WHITE, 0.12)
        } else {
            darcula::ACCENT
        }
    } else if disabled {
        mix(darcula::BG_PANEL, darcula::BG_MAIN, 0.55)
    } else if hovered {
        mix(darcula::BG_RAISED, darcula::ACCENT_WEAK, 0.65)
    } else {
        darcula::BG_RAISED
    };

    let border_color = if is_checked {
        if disabled {
            mix(darcula::ACCENT, darcula::BG_MAIN, 0.40)
        } else {
            darcula::ACCENT
        }
    } else if hovered {
        darcula::ACCENT.scale_alpha(0.54)
    } else {
        darcula::BORDER
    };

    checkbox::Style {
        background: Background::Color(background),
        icon_color: if disabled {
            Color::WHITE.scale_alpha(0.72)
        } else {
            Color::WHITE
        },
        border: Border {
            width: 1.0,
            color: border_color,
            radius: radius::SM.into(),
        },
        text_color: Some(if disabled {
            darcula::TEXT_DISABLED
        } else {
            darcula::TEXT_PRIMARY
        }),
    }
}

/// Get color for file status.
pub fn status_color(status: &str) -> Color {
    match status {
        "added" | "Added" | "新增" => darcula::STATUS_ADDED,
        "modified" | "Modified" | "已修改" => darcula::STATUS_MODIFIED,
        "deleted" | "Deleted" | "已删除" => darcula::STATUS_DELETED,
        "renamed" | "Renamed" | "已重命名" => darcula::STATUS_RENAMED,
        "unversioned" | "Unversioned" | "未版本控制" => darcula::STATUS_UNVERSIONED,
        _ => darcula::TEXT_SECONDARY,
    }
}

/// Themed horizontal separator using SEPARATOR color instead of default.
pub fn separator_rule_style() -> impl Fn(&Theme) -> rule::Style {
    move |_theme| rule::Style {
        color: darcula::SEPARATOR,
        radius: 0.0.into(),
        fill_mode: rule::FillMode::Full,
        snap: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn homepage_density_tokens_match_spec() {
        assert_eq!(density::TOOLBAR_PADDING, [6, 12]);
        assert_eq!(density::SECONDARY_BAR_PADDING, [5, 12]);
        assert_eq!(density::PANE_PADDING, [12, 12]);
        assert_eq!(density::STATUS_PADDING, [4, 12]);
        assert_eq!(density::STANDARD_CONTROL_HEIGHT, 32.0);
        assert_eq!(density::COMPACT_CONTROL_HEIGHT, 28.0);
        assert_eq!(density::COMPACT_CHIP_PADDING, [3, 8]);
    }

    #[test]
    fn homepage_compact_surfaces_are_available() {
        let theme = darcula_theme();
        let toolbar_field = panel_style(Surface::ToolbarField)(&theme);
        let list_row = panel_style(Surface::ListRow)(&theme);
        let list_selection = panel_style(Surface::ListSelection)(&theme);

        assert_eq!(toolbar_field.shadow.blur_radius, 0.0);
        assert_eq!(list_row.shadow.blur_radius, 0.0);
        assert!(list_selection.border.color.a > list_row.border.color.a);
    }
}
