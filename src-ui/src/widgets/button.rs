//! Styled button helpers shared across the Darcula shell.

use crate::theme::{self, ButtonTone};
use iced::widget::{Button, Container, Text};
use iced::{Element, Length};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ButtonRole {
    Standard,
    Compact,
    Tab,
    Rail,
    ToolbarIcon,
    ToolbarSplitMain,
    ToolbarSplitChevron,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct ButtonMetrics {
    text_size: u16,
    padding: [u16; 2],
    height: u16,
    width: Option<f32>,
}

fn metrics(role: ButtonRole) -> ButtonMetrics {
    match role {
        ButtonRole::Standard => ButtonMetrics {
            text_size: 11,
            padding: theme::density::TOOLBAR_PADDING,
            height: theme::density::STANDARD_CONTROL_HEIGHT as u16,
            width: None,
        },
        ButtonRole::Compact => ButtonMetrics {
            text_size: 10,
            padding: [4, 8],
            height: theme::density::COMPACT_CONTROL_HEIGHT as u16,
            width: None,
        },
        ButtonRole::Tab => ButtonMetrics {
            text_size: 11,
            padding: [5, 10],
            height: theme::density::STANDARD_CONTROL_HEIGHT as u16,
            width: None,
        },
        ButtonRole::Rail => ButtonMetrics {
            text_size: 11,
            padding: [6, 0],
            height: 30,
            width: None,
        },
        ButtonRole::ToolbarIcon => ButtonMetrics {
            text_size: 13,
            padding: [4, 0],
            height: 28,
            width: Some(24.0),
        },
        ButtonRole::ToolbarSplitMain => ButtonMetrics {
            text_size: 11,
            padding: theme::density::TOOLBAR_PADDING,
            height: theme::density::STANDARD_CONTROL_HEIGHT as u16,
            width: None,
        },
        ButtonRole::ToolbarSplitChevron => ButtonMetrics {
            text_size: 10,
            padding: [6, 0],
            height: theme::density::STANDARD_CONTROL_HEIGHT as u16,
            width: Some(22.0),
        },
    }
}

fn build_button<'a, Message: Clone + 'a>(
    label: impl Into<String>,
    tone: ButtonTone,
    role: ButtonRole,
    on_press: Option<Message>,
) -> Button<'a, Message> {
    let metrics = metrics(role);
    let mut button = Button::new(Text::new(label.into()).size(u32::from(metrics.text_size)))
        .padding(metrics.padding)
        .height(Length::Fixed(metrics.height as f32))
        .style(theme::button_style(tone));

    if let Some(width) = metrics.width {
        button = button.width(Length::Fixed(width));
    }

    if let Some(message) = on_press {
        button.on_press(message)
    } else {
        button
    }
}

fn build_content_button<'a, Message: Clone + 'a>(
    content: impl Into<Element<'a, Message>>,
    tone: ButtonTone,
    role: ButtonRole,
    on_press: Option<Message>,
) -> Button<'a, Message> {
    let metrics = metrics(role);
    let mut button = Button::new(content)
        .padding(metrics.padding)
        .height(Length::Fixed(metrics.height as f32))
        .style(theme::button_style(tone));

    if let Some(width) = metrics.width {
        button = button.width(Length::Fixed(width));
    }

    if let Some(message) = on_press {
        button.on_press(message)
    } else {
        button
    }
}

pub fn primary<'a, Message: Clone + 'a>(
    label: impl Into<String>,
    on_press: Option<Message>,
) -> Button<'a, Message> {
    build_button(label, ButtonTone::Primary, ButtonRole::Standard, on_press)
}

pub fn secondary<'a, Message: Clone + 'a>(
    label: impl Into<String>,
    on_press: Option<Message>,
) -> Button<'a, Message> {
    build_button(label, ButtonTone::Secondary, ButtonRole::Standard, on_press)
}

pub fn ghost<'a, Message: Clone + 'a>(
    label: impl Into<String>,
    on_press: Option<Message>,
) -> Button<'a, Message> {
    build_button(label, ButtonTone::Ghost, ButtonRole::Standard, on_press)
}

pub fn compact_ghost<'a, Message: Clone + 'a>(
    label: impl Into<String>,
    on_press: Option<Message>,
) -> Button<'a, Message> {
    build_button(label, ButtonTone::Ghost, ButtonRole::Compact, on_press)
}

pub fn toolbar_icon<'a, Message: Clone + 'a>(
    label: impl Into<String>,
    on_press: Option<Message>,
) -> Button<'a, Message> {
    build_button(label, ButtonTone::Ghost, ButtonRole::ToolbarIcon, on_press)
}

pub fn toolbar_split_main<'a, Message: Clone + 'a>(
    content: impl Into<Element<'a, Message>>,
    tone: ButtonTone,
    on_press: Option<Message>,
) -> Button<'a, Message> {
    let metrics = metrics(ButtonRole::ToolbarSplitMain);
    let button = Button::new(content)
        .padding(metrics.padding)
        .style(theme::button_style(tone))
        .height(Length::Fixed(metrics.height as f32));

    if let Some(message) = on_press {
        button.on_press(message)
    } else {
        button
    }
}

pub fn toolbar_split_chevron<'a, Message: Clone + 'a>(
    label: impl Into<String>,
    tone: ButtonTone,
    on_press: Option<Message>,
) -> Button<'a, Message> {
    let metrics = metrics(ButtonRole::ToolbarSplitChevron);
    let button = Button::new(Text::new(label.into()).size(u32::from(metrics.text_size)))
        .padding(metrics.padding)
        .width(Length::Fixed(metrics.width.expect("split chevron width")))
        .height(Length::Fixed(metrics.height as f32))
        .style(theme::button_style(tone));

    if let Some(message) = on_press {
        button.on_press(message)
    } else {
        button
    }
}

pub fn tab<'a, Message: Clone + 'a>(
    label: impl Into<String>,
    active: bool,
    on_press: Option<Message>,
) -> Button<'a, Message> {
    build_button(
        label,
        if active {
            ButtonTone::TabActive
        } else {
            ButtonTone::TabInactive
        },
        ButtonRole::Tab,
        on_press,
    )
}

pub fn rail<'a, Message: Clone + 'a>(
    label: impl Into<String>,
    active: bool,
    on_press: Option<Message>,
) -> Button<'a, Message> {
    build_button(
        label,
        if active {
            ButtonTone::RailActive
        } else {
            ButtonTone::RailInactive
        },
        ButtonRole::Rail,
        on_press,
    )
}

pub fn rail_icon<'a, Message: Clone + 'a>(
    content: impl Into<Element<'a, Message>>,
    active: bool,
    on_press: Option<Message>,
) -> Button<'a, Message> {
    build_content_button(
        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fixed(18.0))
            .center_x(Length::Fill)
            .center_y(Length::Fill),
        if active {
            ButtonTone::RailActive
        } else {
            ButtonTone::RailInactive
        },
        ButtonRole::Rail,
        on_press,
    )
}

pub fn warning<'a, Message: Clone + 'a>(
    label: impl Into<String>,
    on_press: Option<Message>,
) -> Button<'a, Message> {
    build_button(label, ButtonTone::Warning, ButtonRole::Standard, on_press)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toolbar_split_segments_share_same_vertical_padding() {
        let main = metrics(ButtonRole::ToolbarSplitMain);
        let chevron = metrics(ButtonRole::ToolbarSplitChevron);

        assert_eq!(main.padding[0], chevron.padding[0]);
        assert_eq!(main.height, chevron.height);
        assert_eq!(main.height, theme::density::STANDARD_CONTROL_HEIGHT as u16);
    }

    #[test]
    fn tabs_share_the_standard_control_height() {
        assert_eq!(
            metrics(ButtonRole::Tab).height,
            theme::density::STANDARD_CONTROL_HEIGHT as u16
        );
    }
}
