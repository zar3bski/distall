use std::sync::Arc;

use nih_plug::prelude::{util, AtomicF32, Editor, GuiContext};
use nih_plug::util::window;
use nih_plug_iced::widgets as nih_widgets;
use nih_plug_iced::*;

use crate::editor::components::slider_factory;
use crate::editor::style::Theme;
use crate::DistAllParams;

pub(crate) fn default_state() -> Arc<IcedState> {
    IcedState::from_size(600, 150)
}

pub(crate) fn create(
    params: Arc<DistAllParams>,
    editor_state: Arc<IcedState>,
) -> Option<Box<dyn Editor>> {
    create_iced_editor::<DistallEditor>(editor_state, params)
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    /// Update a parameter's value.
    ParamUpdate(nih_widgets::ParamMessage),
}

struct DistallEditor {
    params: Arc<DistAllParams>,
    context: Arc<dyn GuiContext>,
    pre_gain_slider_state: nih_widgets::param_slider::State,
    post_gain_slider_state: nih_widgets::param_slider::State,
    //peak_meter_state: nih_widgets::peak_meter::State,
}

impl IcedEditor for DistallEditor {
    type Executor = executor::Default;
    type Message = Message;
    type InitializationFlags = Arc<DistAllParams>;

    fn new(
        params: Self::InitializationFlags,
        context: std::sync::Arc<dyn GuiContext>,
    ) -> (Self, Command<Self::Message>) {
        let editor = DistallEditor {
            params,
            context,
            pre_gain_slider_state: Default::default(),
            post_gain_slider_state: Default::default(),
            //peak_meter_state: Default::default(),
        };

        (editor, Command::none())
    }

    fn context(&self) -> &dyn GuiContext {
        self.context.as_ref()
    }

    fn update(
        &mut self,
        window: &mut WindowQueue,
        message: Self::Message,
    ) -> Command<Self::Message> {
        match message {
            Message::ParamUpdate(message) => self.handle_param_message(message),
        }
        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        Container::new(
            Row::new()
                .push(slider_factory(
                    &self.params.pre_gain,
                    &mut self.pre_gain_slider_state,
                    "Pre-gain",
                ))
                .push(slider_factory(
                    &self.params.post_gain,
                    &mut self.post_gain_slider_state,
                    "Post-gain",
                )),
        )
        .width(Length::Fill)
        .into()
    }

    fn background_color(&self) -> nih_plug_iced::Color {
        nih_plug_iced::Color {
            r: 0.5,
            g: 0.5,
            b: 0.98,
            a: 1.0,
        }
    }
}
