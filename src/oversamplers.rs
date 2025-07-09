use nih_plug::prelude::Enum;

use crate::{
    distortions::Distortion,
    filters::{BiquadFilter, Filter, FilterType},
};

#[derive(Enum, Debug, PartialEq)]
pub enum Oversampler {
    NaiveOversampler,
    None,
}

pub const BLOCK_SIZE: usize = 64;

pub trait Oversampling {
    fn new(sample_rate: f32) -> Self;
    fn upsample(&mut self, block: &mut [f32]);
    fn downsample(&mut self, block: &mut [f32]);
    fn process(&mut self, block: &mut [f32], f: Distortion, pre_gain: f32, post_gain: f32);
    fn reset(&mut self);
}

//
// Simple x2 oversampling applying the following treatment:
// oversample -> Biquad LPF(fs/4) -> non linear function
// Biquad LPF(fs/4) -> downsample
//
pub struct NaiveOversampler {
    oversampled_block: [f32; BLOCK_SIZE * 2],
    filter_upsample: BiquadFilter,
    filter_downsample: BiquadFilter,
}

impl Oversampling for NaiveOversampler {
    fn new(sample_rate: f32) -> Self {
        let cutoff_frequency = (sample_rate / 4.0) as i32;
        let oversampled_block: [f32; BLOCK_SIZE * 2] = [0.0; BLOCK_SIZE * 2];
        let filter_upsample = BiquadFilter::new(cutoff_frequency, 48000, FilterType::LOWPASS);
        let filter_downsample = BiquadFilter::new(cutoff_frequency, 48000, FilterType::LOWPASS);
        Self {
            oversampled_block: oversampled_block,
            filter_upsample: filter_upsample,
            filter_downsample: filter_downsample,
        }
    }

    fn process(&mut self, block: &mut [f32], f: Distortion, pre_gain: f32, post_gain: f32) {
        self.upsample(block);

        f(pre_gain, post_gain, &mut self.oversampled_block);

        self.downsample(block);
    }

    fn upsample(&mut self, block: &mut [f32]) {
        self.oversampled_block = [0.0; BLOCK_SIZE * 2];
        for n in 0..BLOCK_SIZE * 2 {
            if n % 2 == 0 {
                self.oversampled_block[n] = self.filter_upsample.filter(&mut block[n / 2])
            } else {
                self.oversampled_block[n] =
                    self.filter_upsample.filter(&mut self.oversampled_block[n])
            }
        }
    }
    fn downsample(&mut self, block: &mut [f32]) {
        for n in 0..BLOCK_SIZE * 2 {
            let sample = self
                .filter_downsample
                .filter(&mut self.oversampled_block[n]);
            if n % 2 == 0 {
                block[n / 2] = sample
            }
        }
    }

    fn reset(&mut self) {
        self.filter_upsample.reset();
        self.filter_downsample.reset();
    }
}
