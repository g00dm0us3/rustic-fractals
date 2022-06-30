use crate::Transform;
use ndarray::{Array1, arr1};
use rand::prelude::*;

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32
}


#[derive(Debug)]
struct Rect {
    top_left: Point,
    bottom_right: Point
}

pub const HIST_DIM: u32 = 1000;
const HIST_SIZE: usize = (HIST_DIM*HIST_DIM) as usize;

pub(crate) fn run_chaos_game(transforms: &Vec<Transform>,
                                iterations: i32) -> Box<[u16]> {

        let bounds = find_bounds(transforms, iterations);
        compute_histogram(transforms, iterations, &bounds)
}

// - todo: figure out closures
fn run_chaos_game_<F>(transforms: &Vec<Transform>,
                    iterations: i32,
                    mut point_visitor: F) where F: FnMut(&Array1<f32>) -> () {
    
    
    let mut point: Array1<f32> = arr1(&[0.0, 0.0]);

    let find_transform = |r: f32| {
        for t in transforms {
            if r <= t.prob {
                return Some(t);
            }
        }

        None
    };

    let mut rng = rand::thread_rng();
    for i in 0..iterations {
        let r: f32 = rng.gen();

        let transform: &Transform = find_transform(r).expect("Didn't find transform!");

        let mut three_point = arr1(&[point[0], point[1], 1.0]);

        three_point = transform.matrix.dot(&three_point);

        point = three_point;
        if i <= 50 { continue; }

        point_visitor(&point);
    }
}

// -todo: impl for transforms struct
fn find_bounds(transforms: &Vec<Transform>, iterations: i32) -> Rect {

    let mut max_x: f32 = 0.0;
    let mut max_y: f32 = 0.0;
    let mut min_x: f32 = f32::MAX;
    let mut min_y: f32 = f32::MAX;

    run_chaos_game_(transforms, iterations, |point| {
        if point[0] > max_x {
            max_x = point[0];
        }

        if point[0] < min_x {
            min_x = point[0];
        }

        if point[1] > max_y {
            max_y = point[1];
        }

        if point[1] < min_y {
            min_y = point[1];
        }
    });

    Rect { 
        top_left: Point{ x: min_x, y: min_y}, 
        bottom_right: Point{ x: max_x, y: max_y}
     }
}

#[inline(always)]
fn remap(val: f32, old_min: f32, old_max: f32, new_min: f32, new_max: f32) -> f32 {
    let old_range = old_max - old_min;
    let new_range = new_max - new_min;

    ((val - old_min)*new_range / old_range) + new_min
}

fn compute_histogram(transforms: &Vec<Transform>, iterations: i32, bounds: &Rect) -> Box<[u16]> {
    
    println!("Bounds {:?}", bounds);
    let mut hist: [u16; HIST_SIZE] = [0; HIST_SIZE];

    run_chaos_game_(transforms, iterations, |point| {
        let x_coord = remap(point[0], bounds.top_left.x, bounds.bottom_right.x, 0.0, HIST_DIM as f32).round() as u32;
        let y_coord = remap(point[1], bounds.top_left.y, bounds.bottom_right.y, 0.0, HIST_DIM as f32).round() as u32;

        let lin_coord = (HIST_DIM * HIST_DIM - 1).min(y_coord*HIST_DIM+x_coord) as usize;
        hist[lin_coord] = hist[lin_coord].saturating_add(1);
    });

    let mut count_non_zero = 0;
    for i in 0..HIST_SIZE {
        if hist[i] != 0 {
            count_non_zero += 1;
        }
    }

    println!("Non-zero pixels {}", count_non_zero);

    Box::new(hist)
}