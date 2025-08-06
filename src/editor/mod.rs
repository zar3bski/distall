use atomic_float::AtomicF32;
use nih_plug::prelude::{util, Editor};
use nih_plug_vizia::vizia::image::{open, DynamicImage};
use nih_plug_vizia::vizia::prelude::*;

use nih_plug_vizia::widgets::{ParamSlider, PeakMeter, ResizeHandle};
use nih_plug_vizia::{assets, create_vizia_editor, ViziaState, ViziaTheming};
use std::fmt::Alignment;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Duration;

use crate::editor::widgets::categorical_picker::CategoricalPicker;
use crate::DistAllParams;
mod widgets;

#[derive(Lens)]
struct Data {
    params: Arc<DistAllParams>,
    peak_meter_pre: Arc<AtomicF32>,
    peak_meter_post: Arc<AtomicF32>,
}

impl Model for Data {}

// Makes sense to also define this here, makes it a bit easier to keep track of
pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (700, 250))
}

pub(crate) fn create(
    params: Arc<DistAllParams>,
    peak_meter_pre: Arc<AtomicF32>,
    peak_meter_post: Arc<AtomicF32>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        assets::register_noto_sans_light(cx);
        assets::register_noto_sans_thin(cx);

        Data {
            params: params.clone(),
            peak_meter_pre: peak_meter_pre.clone(),
            peak_meter_post: peak_meter_post.clone(),
        }
        .build(cx);

        cx.add_stylesheet(include_style!("src/editor/theme.css"))
            .expect("Failed to load stylesheet");

        HStack::new(cx, |cx| {
            VStack::new(cx, |cx| {
                Label::new(cx, "IN Gain");
                PeakMeter::new(
                    cx,
                    Data::peak_meter_pre.map(|peak_meter_pre| {
                        util::gain_to_db(peak_meter_pre.load(Ordering::Relaxed))
                    }),
                    Some(Duration::from_millis(600)),
                );
            });
            VStack::new(cx, |cx| {
                Label::new(cx, "OUT Gain");
                PeakMeter::new(
                    cx,
                    Data::peak_meter_post.map(|peak_meter_post| {
                        util::gain_to_db(peak_meter_post.load(Ordering::Relaxed))
                    }),
                    Some(Duration::from_millis(600)),
                );
            })
            .child_left(Stretch(1.0));
        })
        .bottom(Pixels(0.0))
        .top(Pixels(3.0))
        .class("row");

        HStack::new(cx, |cx| {
            VStack::new(cx, |cx| {
                Label::new(cx, "Pre-Gain");

                ParamSlider::new(cx, Data::params, |params| &params.pre_gain).class("gain-slider");
            })
            .child_left(Stretch(1.0))
            .child_right(Stretch(1.0))
            .class("control-panel");

            VStack::new(cx, |cx: &mut Context| {
                Label::new(cx, "Distortion");
                CategoricalPicker::new(cx, Data::params, |params| &params.distortion);
            })
            .class("control-panel")
            .child_left(Stretch(1.0))
            .child_right(Stretch(1.0));
            VStack::new(cx, |cx| {
                Label::new(cx, "Post-Gain");
                ParamSlider::new(cx, Data::params, |params| &params.post_gain).class("gain-slider");
            })
            .child_left(Stretch(1.0))
            .child_right(Stretch(1.0))
            .class("control-panel");
        })
        .class("row");
        HStack::new(cx, |cx| {
            VStack::new(cx, |cx: &mut Context| {}); // TODO: fill or something
            VStack::new(cx, |cx: &mut Context| {
                Label::new(cx, "Oversampling");
                CategoricalPicker::new(cx, Data::params, |params| &params.oversampler);
            })
            .class("control-panel")
            .child_left(Stretch(1.0))
            .child_right(Stretch(1.0));
            VStack::new(cx, |cx: &mut Context| {}); // TODO: fill or something
        })
        .class("row");

        ResizeHandle::new(cx);
    })
}
