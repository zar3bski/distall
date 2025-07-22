use atomic_float::AtomicF32;
use nih_plug::params::{Param, Params};
use nih_plug::prelude::{util, Editor, GuiContext, ParamSetter};
use nih_plug_vizia::vizia::context::EventContext;
use nih_plug_vizia::vizia::prelude::*;

use nih_plug_vizia::widgets::{ParamEvent, ParamSlider, ParamSliderExt, PeakMeter, ResizeHandle};
use nih_plug_vizia::{assets, create_vizia_editor, ViziaState, ViziaTheming};
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::distortions::DistortionType;
use crate::DistAllParams;

#[derive(Lens)]
struct Data {
    params: Arc<DistAllParams>,
    peak_meter: Arc<AtomicF32>,
}

pub enum AppEvent {
    Increment,
    Decrement,
}

impl Model for Data {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        let param = &self.params.distortion;
        event.map(|app_event, meta| match app_event {
            AppEvent::Decrement => {
                cx.emit(ParamEvent::BeginSetParameter(param).upcast());
                cx.emit(
                    ParamEvent::SetParameter(param, param.previous_step(param.value(), false))
                        .upcast(),
                );
                cx.emit(ParamEvent::EndSetParameter(param).upcast());
            }
            AppEvent::Increment => {
                cx.emit(ParamEvent::BeginSetParameter(param).upcast());
                cx.emit(
                    ParamEvent::SetParameter(param, param.next_step(param.value(), false)).upcast(),
                );
                cx.emit(ParamEvent::EndSetParameter(param).upcast());
            }
        });
    }
}

// Makes sense to also define this here, makes it a bit easier to keep track of
pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (400, 150))
}

pub(crate) fn create(
    params: Arc<DistAllParams>,
    peak_meter: Arc<AtomicF32>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        assets::register_noto_sans_light(cx);
        assets::register_noto_sans_thin(cx);

        Data {
            params: params.clone(),
            peak_meter: peak_meter.clone(),
        }
        .build(cx);

        cx.add_stylesheet(include_style!("src/editor/theme.css"))
            .expect("Failed to load stylesheet");

        HStack::new(cx, |cx| {
            VStack::new(cx, |cx| {
                Label::new(cx, "Pre-Gain");

                ParamSlider::new(cx, Data::params, |params| &params.pre_gain);

                PeakMeter::new(
                    cx,
                    Data::peak_meter
                        .map(|peak_meter| util::gain_to_db(peak_meter.load(Ordering::Relaxed))),
                    Some(Duration::from_millis(600)),
                )
                // This is how adding padding works in vizia
                .top(Pixels(10.0));
            })
            .row_between(Pixels(0.0))
            .child_left(Stretch(1.0))
            .child_right(Stretch(1.0));

            VStack::new(cx, |cx: &mut Context| {
                Label::new(cx, "Distortion");
                HStack::new(cx, |cx: &mut Context| {
                    Button::new(
                        cx,
                        |ex| ex.emit(AppEvent::Decrement),
                        |cx| Label::new(cx, "<<"),
                    );
                    Label::new(cx, &Data::params.get(cx).distortion.to_string()); //FIXME: MAJ
                                                                                  //Label::new(cx, &Data::params.get_val(cx).distortion.to_string());
                    Button::new(
                        cx,
                        |ex| ex.emit(AppEvent::Increment),
                        |cx| Label::new(cx, ">>"),
                    );
                });
            });
            VStack::new(cx, |cx| {
                Label::new(cx, "Post-Gain");

                ParamSlider::new(cx, Data::params, |params| &params.post_gain);
            })
            .row_between(Pixels(0.0))
            .child_left(Stretch(1.0))
            .child_right(Stretch(1.0));
        });
        ResizeHandle::new(cx);
    })
}
