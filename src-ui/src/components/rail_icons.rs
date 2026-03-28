//! Symbolic SVG icons used by the compact shell chrome.

use iced::widget::svg;
use iced::{Color, Element, Length};
use once_cell::sync::Lazy;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RailIcon {
    Repository,
    OpenRepository,
    Branch,
    Overview,
    Changes,
    Conflicts,
    History,
    Remotes,
    Tags,
    Stashes,
    Rebase,
}

pub fn view<'a, Message: 'a>(
    icon: RailIcon,
    color: Color,
    hover_color: Color,
    size: f32,
) -> Element<'a, Message> {
    svg::Svg::new(handle(icon))
        .width(Length::Fixed(size))
        .height(Length::Fixed(size))
        .style(move |_theme, status| svg::Style {
            color: Some(match status {
                svg::Status::Hovered => hover_color,
                svg::Status::Idle => color,
            }),
        })
        .into()
}

fn handle(icon: RailIcon) -> svg::Handle {
    match icon {
        RailIcon::Repository => REPOSITORY_ICON.clone(),
        RailIcon::OpenRepository => OPEN_REPOSITORY_ICON.clone(),
        RailIcon::Branch => BRANCH_ICON.clone(),
        RailIcon::Overview => OVERVIEW_ICON.clone(),
        RailIcon::Changes => CHANGES_ICON.clone(),
        RailIcon::Conflicts => CONFLICTS_ICON.clone(),
        RailIcon::History => HISTORY_ICON.clone(),
        RailIcon::Remotes => REMOTES_ICON.clone(),
        RailIcon::Tags => TAGS_ICON.clone(),
        RailIcon::Stashes => STASHES_ICON.clone(),
        RailIcon::Rebase => REBASE_ICON.clone(),
    }
}

static REPOSITORY_ICON: Lazy<svg::Handle> = Lazy::new(|| svg::Handle::from_memory(REPOSITORY_SVG));
static OPEN_REPOSITORY_ICON: Lazy<svg::Handle> =
    Lazy::new(|| svg::Handle::from_memory(OPEN_REPOSITORY_SVG));
static BRANCH_ICON: Lazy<svg::Handle> = Lazy::new(|| svg::Handle::from_memory(BRANCH_SVG));
static OVERVIEW_ICON: Lazy<svg::Handle> = Lazy::new(|| svg::Handle::from_memory(OVERVIEW_SVG));
static CHANGES_ICON: Lazy<svg::Handle> = Lazy::new(|| svg::Handle::from_memory(CHANGES_SVG));
static CONFLICTS_ICON: Lazy<svg::Handle> = Lazy::new(|| svg::Handle::from_memory(CONFLICTS_SVG));
static HISTORY_ICON: Lazy<svg::Handle> = Lazy::new(|| svg::Handle::from_memory(HISTORY_SVG));
static REMOTES_ICON: Lazy<svg::Handle> = Lazy::new(|| svg::Handle::from_memory(REMOTES_SVG));
static TAGS_ICON: Lazy<svg::Handle> = Lazy::new(|| svg::Handle::from_memory(TAGS_SVG));
static STASHES_ICON: Lazy<svg::Handle> = Lazy::new(|| svg::Handle::from_memory(STASHES_SVG));
static REBASE_ICON: Lazy<svg::Handle> = Lazy::new(|| svg::Handle::from_memory(REBASE_SVG));

const REPOSITORY_SVG: &[u8] = br#"
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round">
  <path d="M2.25 4.5h3.7l1.35 1.5h6.45v5.25A1.25 1.25 0 0 1 12.5 12.5h-9A1.25 1.25 0 0 1 2.25 11.25z"/>
</svg>
"#;

const OPEN_REPOSITORY_SVG: &[u8] = br#"
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round">
  <path d="M2.25 5h3.6L7.2 6.5h6.55v4.75A1.25 1.25 0 0 1 12.5 12.5h-9A1.25 1.25 0 0 1 2.25 11.25z"/>
  <path d="M8.1 2.75v3.1"/>
  <path d="M6.55 4.3h3.1"/>
</svg>
"#;

const BRANCH_SVG: &[u8] = br#"
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round">
  <circle cx="4" cy="3.25" r="1.5"/>
  <circle cx="12" cy="5.25" r="1.5"/>
  <circle cx="4" cy="12.25" r="1.5"/>
  <path d="M5.5 3.25h2A2.5 2.5 0 0 1 10 5.75v0"/>
  <path d="M4 4.75v6"/>
  <path d="M5.5 12.25h1.5"/>
</svg>
"#;

const OVERVIEW_SVG: &[u8] = br#"
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.25" stroke-linecap="round" stroke-linejoin="round">
  <rect x="2.5" y="2.5" width="4.25" height="4.25" rx="0.8"/>
  <rect x="9.25" y="2.5" width="4.25" height="4.25" rx="0.8"/>
  <rect x="2.5" y="9.25" width="4.25" height="4.25" rx="0.8"/>
  <rect x="9.25" y="9.25" width="4.25" height="4.25" rx="0.8"/>
</svg>
"#;

const CHANGES_SVG: &[u8] = br#"
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.35" stroke-linecap="round" stroke-linejoin="round">
  <path d="M4.75 4h8"/>
  <path d="M4.75 8h8"/>
  <path d="M4.75 12h8"/>
  <circle cx="2.75" cy="4" r="0.65" fill="currentColor" stroke="none"/>
  <circle cx="2.75" cy="8" r="0.65" fill="currentColor" stroke="none"/>
  <circle cx="2.75" cy="12" r="0.65" fill="currentColor" stroke="none"/>
</svg>
"#;

const CONFLICTS_SVG: &[u8] = br#"
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.35" stroke-linecap="round" stroke-linejoin="round">
  <path d="M8 2.5l5.3 9.25H2.7z"/>
  <path d="M8 6v3.2"/>
  <circle cx="8" cy="11.4" r="0.7" fill="currentColor" stroke="none"/>
</svg>
"#;

const HISTORY_SVG: &[u8] = br#"
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.35" stroke-linecap="round" stroke-linejoin="round">
  <path d="M3.5 7.25A4.75 4.75 0 1 1 8 12.75a4.6 4.6 0 0 1-3.4-1.45"/>
  <path d="M3.5 3.75v3.5H7"/>
  <path d="M8 5.5v2.9l2.2 1.35"/>
</svg>
"#;

const REMOTES_SVG: &[u8] = br#"
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.35" stroke-linecap="round" stroke-linejoin="round">
  <path d="M5 12.5V3.5"/>
  <path d="M3.25 5.25L5 3.5l1.75 1.75"/>
  <path d="M11 3.5v9"/>
  <path d="M9.25 10.75L11 12.5l1.75-1.75"/>
</svg>
"#;

const TAGS_SVG: &[u8] = br#"
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.35" stroke-linecap="round" stroke-linejoin="round">
  <path d="M8.1 2.75h3.55l1.6 1.6v3.55L7.55 13.6 2.4 8.45z"/>
  <circle cx="10.6" cy="5.4" r="0.65"/>
</svg>
"#;

const STASHES_SVG: &[u8] = br#"
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.35" stroke-linecap="round" stroke-linejoin="round">
  <rect x="2.5" y="4.25" width="11" height="8.25" rx="1.1"/>
  <path d="M2.5 7.1h11"/>
  <path d="M6 9.8h4"/>
</svg>
"#;

const REBASE_SVG: &[u8] = br#"
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.35" stroke-linecap="round" stroke-linejoin="round">
  <circle cx="4" cy="3.5" r="1.2"/>
  <circle cx="12" cy="5.5" r="1.2"/>
  <circle cx="4" cy="12" r="1.2"/>
  <path d="M5.2 3.5h1.8A3 3 0 0 1 10 6.5"/>
  <path d="M4 4.7v4a3.3 3.3 0 0 0 3.3 3.3h3.5"/>
  <path d="M9.7 10.4l1.8 1.6-1.8 1.5"/>
</svg>
"#;
