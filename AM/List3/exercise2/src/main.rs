use tspf::TspBuilder;
use std::process;
use rand::prelude::*;
use local_search::*;
use std::fs;
use std::path::Path;

fn main() {
    let directory_path = "../data";

    if let Ok(entries) = fs::read_dir(directory_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_path = entry.path();
                match TspBuilder::parse_path(file_path.clone()) {
                    Ok(tsp) => {
                        println!("For {}", file_path.display());
                        let nodes: Vec<[i64; 2]> = tsp
                            .node_coords()
                            .values()
                            .map(|point| [point.pos()[0] as i64, point.pos()[1] as i64])
                            .collect();
                        let mut rng = thread_rng();

                        let mut steps_sum = 0u64;
                        let mut min = u64::MAX;
                        let mut cost_sum = 0u64;

                        let times = if nodes.len() > 1000 {
                            100
                        } else {
                            nodes.len()
                        };

                        for _ in 0..times {
                            let mut n = nodes.clone();
                            n.shuffle(&mut rng);

                            let (r, s) = invert_tsp_tabu_search(n);
                            steps_sum += s;
                            let c = cost(&r) as u64;

                            if c < min {
                                min = c;
                            }
                            cost_sum += c;
                        }

                        println!("avg = {}\nmin = {}\nsteps = {}", cost_sum as f64 / times as f64, min, steps_sum as f64 / times as f64)
                    }
                    Err(_) => {
                        println!("Problem with file");
                        process::exit(1);
                    }
                }
            } else {
                eprintln!("Error iterating over directory entries");
            }
        }
    } else {
        eprintln!("Error opening directory: {}", directory_path);
    }
}

