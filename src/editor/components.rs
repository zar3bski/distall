use nih_plug::params::FloatParam;
use nih_plug_iced::container::StyleSheet;
use nih_plug_iced::widgets::param_slider::State;
use nih_plug_iced::widgets::ParamSlider;
use nih_plug_iced::{Column, Container, IcedEditor, Text};

use crate::editor::gui::Message;
use crate::editor::style::Theme;

pub fn slider_factory<'a>(
    param: &'a FloatParam,
    state: &'a mut State,
    label: &'a str,
) -> Container<'a, Message> {
    let container = Container::new(
        Column::new()
            .push(Text::new(label))
            .push(ParamSlider::new(state, param).map(Message::ParamUpdate)),
    );
    //FIXME
    //container.style(Theme::DARK);

    return container;
}
