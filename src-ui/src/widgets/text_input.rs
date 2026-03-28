//! Styled text input helpers.

use crate::theme;
use iced::widget::TextInput;
use iced::Length;

pub fn styled<'a, Message: Clone + 'a>(
    placeholder: &'a str,
    value: &'a str,
    on_change: impl Fn(String) -> Message + 'a,
) -> TextInput<'a, Message> {
    TextInput::new(placeholder, value)
        .padding([8, 12])
        .size(13)
        .width(Length::Fill)
        .style(theme::text_input_style())
        .on_input(on_change)
}
