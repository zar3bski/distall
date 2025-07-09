pub type Distortion = fn(f32, f32, &mut [f32]);

//pub fn soft_clipping(pre_gain: f32, post_gain: f32, sample: &mut f32) {
//    let top: f32 = 1.0;
//    let bottom: f32 = -1.0;
//
//    *sample *= pre_gain;
//    if *sample <= bottom {
//        *sample = -0.66666666;
//    } else if *sample >= top {
//        *sample = 0.66666666;
//    } else {
//        *sample = *sample - (sample.powf(3.0) / 3.0);
//    }
//    *sample *= post_gain;
//}

pub fn soft_clipping(pre_gain: f32, post_gain: f32, samples: &mut [f32]) {
    let top: f32 = 1.0;
    let bottom: f32 = -1.0;
    for sample in samples {
        *sample *= pre_gain;
        if *sample <= bottom {
            *sample = -0.66666666;
        } else if *sample >= top {
            *sample = 0.66666666;
        } else {
            *sample = *sample - (sample.powf(3.0) / 3.0);
        }
        *sample *= post_gain;
    }
}
