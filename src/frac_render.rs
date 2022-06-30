use std::iter;

use image::{RgbImage, Rgb};
use crate::chaos_game::HIST_DIM;

pub fn img_bw(histogram: Box<[u16]>, img_sz: (u32, u32)) -> RgbImage {
    let mut img = RgbImage::new(img_sz.0, img_sz.1);

    let resampled = resample_regular(histogram, img_sz, 1);

    for x in 0..img_sz.0 {
        for y in 0..img_sz.1 {
            let val = resampled[(y*img_sz.0+x) as usize];

            if val > 0.0 {
                img.put_pixel(img_sz.0 - 1 - x, img_sz.1 - 1 - y, Rgb([0xff, 0xff, 0xff]));
            } else {
                img.put_pixel(img_sz.0  - 1- x, img_sz.1 - 1 - y, Rgb([0; 3]));
            }
        }
    }

    return img;
}

// - todo: get rid of as usize
fn resample_regular(histogram: Box<[u16]>, 
    img_sz: (u32, u32),
    sample_width: i32) -> Vec<f32> {
    
    assert!(HIST_DIM % img_sz.0 == 0 && HIST_DIM % img_sz.1 == 0);

    let x_scale = (HIST_DIM / img_sz.0) as usize;
    let y_scale = (HIST_DIM / img_sz.1) as usize;

    let (img_width, img_height) = (img_sz.0 as usize, img_sz.1 as usize);

    let mut subsampled: Vec<f32> = iter::repeat(0.0).take(img_width*img_height).collect();

    let mut count_non_zero = 0;
    for x_img in 0..img_width {
        for y_img in 0..img_height {

            // - todo: sample from the mid of the interval (edges??)
            let sub_x_lower = (x_img*x_scale).min((HIST_DIM - 1) as usize);
            let sub_x_upper = (x_img*x_scale + x_scale + (sample_width as usize)).min((HIST_DIM - 1) as usize);

            let sub_y_lower = (y_img*y_scale).min((HIST_DIM - 1) as usize);
            let sub_y_upper = (y_img*y_scale + y_scale + (sample_width as usize)).min((HIST_DIM - 1) as usize);

            let mut summed: f32 = 0.0;
            let mut count_summed: i16 = 1;

            for sub_x in sub_x_lower..sub_x_upper+1 {
                for sub_y in sub_y_lower..sub_y_upper+1 {

                    let lin_idx = sub_y*(HIST_DIM as usize)+sub_x;

                    summed += histogram[lin_idx] as f32;
                    count_summed += 1;
                }
            }

            if summed > 0.0 {
                count_non_zero += 1;
            }
            subsampled[y_img*img_width+x_img] = summed / (count_summed as f32);
        }
    }

    println!("Non-zero in subsamled {}", count_non_zero);

    subsampled
}