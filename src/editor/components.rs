use nih_plug::params::FloatParam;
use nih_plug_iced::widgets::param_slider::State;
use nih_plug_iced::widgets::ParamSlider;
use nih_plug_iced::{Column, Container, IcedEditor, Text};

use crate::editor::gui::Message;

pub fn slider_factory(param: &mut FloatParam, state: &mut State, label: &str) -> Container {
    return Container::new(
        Column::new()
            .push(Text::new(label))
            .push(ParamSlider::new(state, param).map(Message::ParamUpdate)),
    );
}
