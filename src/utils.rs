use std::sync::Arc;

use atomic_float::AtomicF32;
use nih_plug::buffer::SamplesIter;

pub fn gain_meter_calculator(
    samples: SamplesIter,
    peak_meter: &Arc<AtomicF32>,
    peak_meter_decay_weight: f32,
) {
    for channel_samples in samples {
        let num_samples = channel_samples.len();
        let mut amplitude = 0.0;
        for sample in channel_samples {
            amplitude += *sample;
        }
        amplitude = (amplitude / num_samples as f32).abs();
        let current_peak_meter = peak_meter.load(std::sync::atomic::Ordering::Relaxed);
        let new_peak_meter = if amplitude > current_peak_meter {
            amplitude
        } else {
            current_peak_meter * peak_meter_decay_weight
                + amplitude * (1.0 - peak_meter_decay_weight)
        };
        peak_meter.store(new_peak_meter, std::sync::atomic::Ordering::Relaxed);
    }
}
