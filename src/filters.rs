use std::f32::consts::PI;

pub enum FilterType {
    LOWPASS,
}

pub trait Filter {
    fn new(cutoff_frequency: i32, sample_rate: i32, filter_type: FilterType) -> Self;
    fn filter(&mut self, sample: &mut f32) -> f32;
    fn reset(&mut self);
}

#[derive(Debug)]
struct BiquadCoefficients {
    pub a1: f32,
    pub a2: f32,
    pub b0: f32,
    pub b1: f32,
    pub b2: f32,
}

pub struct BiquadFilter {
    coefficients: BiquadCoefficients,
    s1: f32,
    s2: f32,
}

impl Filter for BiquadFilter {
    fn new(cutoff_frequency: i32, sample_rate: i32, filter_type: FilterType) -> Self {
        let q = 3.0;
        let omega = (2.0 * PI * cutoff_frequency as f32) / sample_rate as f32;

        let alpha = omega.sin() / (2.0 * q);

        let a0 = 1.0 + alpha;

        let coefficients = match filter_type {
            FilterType::LOWPASS => {
                let coefficients = BiquadCoefficients {
                    a1: (-2.0 * omega.cos()) / a0,
                    a2: (1.0 - alpha) / a0,
                    b0: ((1.0 - omega.cos()) / 2.0) / a0,
                    b1: (1.0 - omega.cos()) / a0,
                    b2: ((1.0 - omega.cos()) / 2.0) / a0,
                };
                coefficients
            }
        };

        Self {
            coefficients: coefficients,
            s1: 0.0,
            s2: 0.0,
        }
    }

    fn filter(&mut self, sample: &mut f32) -> f32 {
        let result = self.coefficients.b0 * *sample + self.s1;

        self.s1 = self.coefficients.b1 * *sample - self.coefficients.a1 * result + self.s2;
        self.s2 = self.coefficients.b2 * *sample - self.coefficients.a2 * result;
        result
    }

    fn reset(&mut self) {
        self.s1 = 0.0;
        self.s2 = 0.0;
    }
}
