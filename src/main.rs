use ndarray::{arr2, Array2};
use std::cmp;

use std::fs::read_to_string;
use std::collections::HashMap;

mod chaos_game;
use chaos_game::*;

mod frac_render;
use frac_render::*;

use std::time::Instant;

#[derive(Debug)]
pub(crate) struct Transform {
    matrix: Array2<f32>,
    prob: f32
}

#[derive(Debug)]
struct Ifs {
    transforms: Vec<Transform>
}

// - todo: use serde Deserializer

fn copy_mat(mat: &Array2<f32>) -> Array2<f32> {
    arr2(&[
        [mat[[0, 0]], mat[[0, 1]], mat[[0, 2]]],
        [mat[[1, 0]], mat[[1, 1]], mat[[1, 2]]]
    ])
}

fn parse_transforms(json: &serde_json::Value) -> Vec<Transform> {
    let arr = json.as_array().expect("Expected array!");
    let mut res: Vec<Transform> = Vec::new();

    for val in arr {
        match val {
           serde_json::Value::Object(map) => {
                let a = map.get_key_value("a").unwrap().1.as_f64().unwrap() as f32;
                let b = map.get_key_value("b").unwrap().1.as_f64().unwrap() as f32;
                let c = map.get_key_value("c").unwrap().1.as_f64().unwrap() as f32;
                let d = map.get_key_value("d").unwrap().1.as_f64().unwrap() as f32;
                let e = map.get_key_value("e").unwrap().1.as_f64().unwrap() as f32;
                let f = map.get_key_value("f").unwrap().1.as_f64().unwrap() as f32;
                let p = map.get_key_value("p").unwrap().1.as_f64().unwrap() as f32;

                res.push(Transform {
                    matrix: arr2(&[[a, b, c],
                                   [d, e, f]]),
                    prob: p
                })
           }
           _ => panic!("Wrong json format!") 
        }
    }

    let mut prepared_list: Vec<Transform> = Vec::new();

    res.sort_by(|a, b| {
        if a.prob < b.prob {
            return cmp::Ordering::Less;
        } else if a.prob > b.prob {
            return cmp::Ordering::Greater;
        }

        return cmp::Ordering::Equal;
    });

    for i in 0..res.len() {
        if i == 0 {
            prepared_list.push(Transform {
                 matrix: copy_mat(&res[i].matrix),
                 prob: res[i].prob 
            });
            continue;
        }

        let mut next_prob = prepared_list[i - 1].prob + res[i].prob;

        if f32::abs(next_prob - 1.0) <= 0.1 && i == res.len() - 1 {
            next_prob = 1.0;
        }

        prepared_list.push(Transform {
            matrix: copy_mat(&res[i].matrix),
            prob: next_prob
       });
    }

    return prepared_list;
}

fn main() {
    let mut db: HashMap<String, Box<Ifs>> = HashMap::new();

    let str = read_to_string("/Users/homer/rustic-fractals/src/ifs_presets.json").expect("Cannot read file!");
    let json: serde_json::Value = serde_json::from_str(&str).expect("JSON was not well-formatted");

    match json {
        serde_json::Value::Array(values) => {
            for val in values{
            
                match val {
                    serde_json::Value::Object(map) => {
                        let name = map.get_key_value("name").unwrap().1.as_str().unwrap();

                        let json_transforms = map.get_key_value("transforms").unwrap().1;
                        let transforms = parse_transforms(json_transforms);

                        let boxed: Box<Ifs> = Box::new(Ifs{ transforms: transforms });
                        db.insert(name.to_string(), boxed);
                    },
                    _ => continue
                };
            };
        },
        _ => { println!("No array!"); return }
    };

    // look in ifs_presets for more IFS samples
   let transforms = db.get_key_value("Barnsley fern").unwrap().1;

   let now = Instant::now();
   // increase number of iterations for longer computation and more precise picture
   let hist = run_chaos_game(&transforms.transforms, 1000*1000);
   let img = img_bw(hist, (500, 500));
   let elapsed = now.elapsed();

   println!("generated 500x500 image in {} (s)", elapsed.as_secs_f32());
   //img.save("Barnsley fern.png").unwrap();
}
