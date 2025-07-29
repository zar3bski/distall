use nih_plug::params::Param;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::vizia::{binding::Lens, view::View};
use nih_plug_vizia::widgets::param_base::ParamWidgetBase;

#[derive(Lens)]
pub struct CategoricalPicker {
    param_base: ParamWidgetBase,
    style: CategoricalPickerStyle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Data)]
pub enum CategoricalPickerStyle {
    Centered,
}

#[derive(Debug, Clone, Copy)]
enum CategoryEvent {
    Increment,
    Decrement,
}

impl CategoricalPicker {
    pub fn new<L, Params, P, FMap>(
        cx: &mut Context,
        params: L,
        params_to_param: FMap,
    ) -> Handle<Self>
    where
        L: Lens<Target = Params> + Clone,
        Params: 'static,
        P: Param + 'static,
        FMap: Fn(&Params) -> &P + Copy + 'static,
    {
        Self {
            param_base: ParamWidgetBase::new(cx, params, params_to_param),
            style: CategoricalPickerStyle::Centered,
        }
        .build(
            cx,
            ParamWidgetBase::build_view(params, params_to_param, move |cx, param_data| {
                let display_value_lens = param_data.make_lens(|param| {
                    param.normalized_value_to_string(param.unmodulated_normalized_value(), true)
                });
                Binding::new(cx, CategoricalPicker::style, move |cx, style| {
                    HStack::new(cx, |cx: &mut Context| {
                        Self::button_view(cx, CategoryEvent::Decrement, "<<");
                        Self::text_input_view(cx, display_value_lens);
                        Self::button_view(cx, CategoryEvent::Increment, ">>");
                    });
                });
            }),
        )
    }

    fn button_view(cx: &mut Context, event: CategoryEvent, text: &str) {
        //Button::new(cx, cx.emit(event), |cx| Label::new(cx, text));
        Button::new(
            cx,
            move |ex| ex.emit(event.clone()),
            |cx| Label::new(cx, text),
        );
    }

    fn text_input_view(cx: &mut Context, display_value_lens: impl Lens<Target = String>) {
        Textbox::new(cx, display_value_lens)
            .class("categorie")
            //TODO: complete
            //.on_submit(|cx, string, success| {
            //    if success {
            //        cx.emit(ParamSliderEvent::TextInput(string))
            //    } else {
            //        cx.emit(ParamSliderEvent::CancelTextInput);
            //    }
            //})
            //.on_cancel(|cx| {
            //    cx.emit(ParamSliderEvent::CancelTextInput);
            //})
            //.on_build(|cx| {
            //    cx.emit(TextEvent::StartEdit);
            //    cx.emit(TextEvent::SelectAll);
            //})
            // `.child_space(Stretch(1.0))` no longer works
            .class("align_center")
            .child_top(Stretch(1.0))
            .child_bottom(Stretch(1.0))
            .height(Stretch(1.0))
            .width(Stretch(1.0));
    }
}

impl View for CategoricalPicker {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        let current = self.param_base.modulated_plain_value();
        let max = self.param_base.step_count().unwrap();
        event.map(
            |categorical_picker_event, meta| match categorical_picker_event {
                CategoryEvent::Decrement => {
                    if current != 0.0 {
                        self.param_base.begin_set_parameter(cx);
                        self.param_base.set_normalized_value(
                            cx,
                            self.param_base.previous_normalized_step(current, false),
                        );
                        self.param_base.end_set_parameter(cx);
                    };

                    meta.consume();
                }
                CategoryEvent::Increment => {
                    if current != max as f32 {
                        self.param_base.begin_set_parameter(cx);
                        self.param_base.set_normalized_value(
                            cx,
                            self.param_base.next_normalized_step(current, false),
                        );
                        self.param_base.end_set_parameter(cx);
                    }

                    meta.consume();
                }
            },
        )
    }
}
