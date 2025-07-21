mod distortions;
mod filters;
mod gui;
mod oversamplers;

use nih_plug::prelude::*;
use std::sync::Arc;

use crate::{
    distortions::DistortionType,
    gui::{editor::ViziaEditor, state::ViziaState},
    oversamplers::{NaiveOversampler, Oversampler, Oversampling, BLOCK_SIZE},
};

// This is a shortened version of the gain example with most comments removed, check out
// https://github.com/robbert-vdh/nih-plug/blob/master/plugins/examples/gain/src/lib.rs to get
// started

/// The time it takes for the peak meter to decay by 12 dB after switching to complete silence.
const PEAK_METER_DECAY_MS: f64 = 150.0;

struct DistAll {
    params: Arc<DistAllParams>,
    naive_oversamplers: Vec<NaiveOversampler>,
    peak_meter: Arc<AtomicF32>,
    peak_meter_decay_weight: f32,
}

#[derive(Params)]
struct DistAllParams {
    /// The parameter's ID is used to identify the parameter in the wrappred plugin API. As long as
    /// these IDs remain constant, you can rename and reorder these fields as you wish. The
    /// parameters are exposed to the host in the same order they were defined. In this case, this
    /// gain parameter is stored as linear gain while the values are displayed in decibels.
    #[persist = "editor-state"]
    editor_state: Arc<ViziaState>,
    #[id = "pre_gain"]
    pub pre_gain: FloatParam,
    #[id = "post_gain"]
    pub post_gain: FloatParam,
    #[id = "oversampler"]
    pub oversampler: EnumParam<Oversampler>,
    #[id = "distortion"]
    pub distortion: EnumParam<DistortionType>,
}

impl Default for DistAll {
    fn default() -> Self {
        Self {
            params: Arc::new(DistAllParams::default()),
            naive_oversamplers: vec![],
            peak_meter: Arc::new(AtomicF32::new(util::MINUS_INFINITY_DB)),
            peak_meter_decay_weight: 1.0,
        }
    }
}

impl Default for DistAllParams {
    fn default() -> Self {
        Self {
            // This gain is stored as linear gain. NIH-plug comes with useful conversion functions
            // to treat these kinds of parameters as if we were dealing with decibels. Storing this
            // as decibels is easier to work with, but requires a conversion for every sample.
            editor_state: gui::default_state(),
            pre_gain: FloatParam::new(
                "Pre Gain",
                util::db_to_gain(20.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-30.0),
                    max: util::db_to_gain(30.0),
                    // This makes the range appear as if it was linear when displaying the values as
                    // decibels
                    factor: FloatRange::gain_skew_factor(-30.0, 30.0),
                },
            )
            // Because the gain parameter is stored as linear gain instead of storing the value as
            // decibels, we need logarithmic smoothing
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" dB")
            // There are many predefined formatters we can use here. If the gain was stored as
            // decibels instead of as a linear gain value, we could have also used the
            // `.with_step_size(0.1)` function to get internal rounding.
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),
            post_gain: FloatParam::new(
                "Post Gain",
                util::db_to_gain(-12.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-30.0),
                    max: util::db_to_gain(30.0),
                    // This makes the range appear as if it was linear when displaying the values as
                    // decibels
                    factor: FloatRange::gain_skew_factor(-30.0, 30.0),
                },
            )
            // Because the gain parameter is stored as linear gain instead of storing the value as
            // decibels, we need logarithmic smoothing
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" dB")
            // There are many predefined formatters we can use here. If the gain was stored as
            // decibels instead of as a linear gain value, we could have also used the
            // `.with_step_size(0.1)` function to get internal rounding.
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),
            oversampler: EnumParam::new("Oversampler", Oversampler::None),
            distortion: EnumParam::new("Distortion", DistortionType::SOFT),
        }
    }
}

impl Plugin for DistAll {
    const NAME: &'static str = "DistAll";
    const VENDOR: &'static str = "David Zarebski";
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = "zarebskidavid@gmail.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    // The first audio IO layout is used as the default. The other layouts may be selected either
    // explicitly or automatically by the host or the user depending on the plugin API/backend.
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),

        aux_input_ports: &[],
        aux_output_ports: &[],

        // Individual ports and the layout as a whole can be named here. By default these names
        // are generated as needed. This layout will be called 'Stereo', while a layout with
        // only one input and output channel would be called 'Mono'.
        names: PortNames::const_default(),
    }];

    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    // If the plugin can send or receive SysEx messages, it can define a type to wrap around those
    // messages here. The type implements the `SysExMessage` trait, which allows conversion to and
    // from plain byte buffers.
    type SysExMessage = ();
    // More advanced plugins can use this to run expensive background tasks. See the field's
    // documentation for more information. `()` means that the plugin does not have any background
    // tasks.
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        editor::create(
            self.params.clone(),
            self.peak_meter.clone(),
            self.params.editor_state.clone(),
        )
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        // Resize buffers and perform other potentially expensive initialization operations here.
        // The `reset()` function is always called right after this function. You can remove this
        // function if you do not need it.
        self.peak_meter_decay_weight = 0.25f64
            .powf((_buffer_config.sample_rate as f64 * PEAK_METER_DECAY_MS / 1000.0).recip())
            as f32;

        self.naive_oversamplers
            .push(NaiveOversampler::new(_buffer_config.sample_rate));
        self.naive_oversamplers
            .push(NaiveOversampler::new(_buffer_config.sample_rate));
        true
    }

    fn reset(&mut self) {
        // Reset buffers and envelopes here. This can be called from the audio thread and may not
        // allocate. You can remove this function if you do not need it.
        for oversampler in &mut self.naive_oversamplers {
            oversampler.reset()
        }
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        // Input gain calculator TO MOVE IN A THREAD
        for channel_samples in buffer.iter_samples() {
            let mut amplitude = 0.0;
            let num_samples = channel_samples.len();

            for sample in channel_samples {
                amplitude += *sample;
            }

            // To save resources, a plugin can (and probably should!) only perform expensive
            // calculations that are only displayed on the GUI while the GUI is open
            if self.params.editor_state.is_open() {
                amplitude = (amplitude / num_samples as f32).abs();
                let current_peak_meter = self.peak_meter.load(std::sync::atomic::Ordering::Relaxed);
                let new_peak_meter = if amplitude > current_peak_meter {
                    amplitude
                } else {
                    current_peak_meter * self.peak_meter_decay_weight
                        + amplitude * (1.0 - self.peak_meter_decay_weight)
                };
                self.peak_meter
                    .store(new_peak_meter, std::sync::atomic::Ordering::Relaxed)
            }
        }

        for (_, mut block) in buffer.iter_blocks(BLOCK_SIZE) {
            // Smoothing is optionally built into the parameters themselves
            let pre_gain: f32 = self.params.pre_gain.smoothed.next();
            let post_gain: f32 = self.params.post_gain.smoothed.next();

            let oversampler_type = self.params.oversampler.value();
            let distortion_type = self.params.distortion.value().function();
            let channels = block.channels();

            for channel_index in 0..channels {
                match oversampler_type {
                    Oversampler::None => {
                        distortion_type(pre_gain, post_gain, block.get_mut(channel_index).unwrap());
                    }
                    Oversampler::NaiveOversampler => {
                        match channel_index {
                            0 => self.naive_oversamplers[0].process(
                                block.get_mut(channel_index).unwrap(),
                                distortion_type,
                                pre_gain,
                                post_gain,
                            ),
                            1 => self.naive_oversamplers[1].process(
                                block.get_mut(channel_index).unwrap(),
                                distortion_type,
                                pre_gain,
                                post_gain,
                            ),
                            _ => panic!("Dual channel only"),
                        };
                    }
                }
            }
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for DistAll {
    const CLAP_ID: &'static str = "com.zar3bski.DistAll";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("General purpose distortion");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;

    // Don't forget to change these features
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::AudioEffect,
        ClapFeature::Stereo,
        ClapFeature::Distortion,
    ];
}

impl Vst3Plugin for DistAll {
    const VST3_CLASS_ID: [u8; 16] = *b"Exactly16Chars!!";

    // And also don't forget to change these categories
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Fx,
        Vst3SubCategory::Dynamics,
        Vst3SubCategory::Distortion,
    ];
}

nih_export_clap!(DistAll);
nih_export_vst3!(DistAll);
