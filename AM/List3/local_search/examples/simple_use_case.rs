extern crate local_search;
use std::env;
use std::path::Path;
use std::process;
use rand::seq::SliceRandom;
use tspf::TspBuilder;
use std::hint::black_box;
use rand::thread_rng;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() != 2 {
        println!("Wrong amount of operands, usage: tsp_tester <file_path>");
        process::exit(1)
    }

    let file_path = args.get(1).expect("We know there are args is len 2");

    match TspBuilder::parse_path(Path::new(file_path)) {
        Ok(tsp) => {
            let mut nodes: Vec<[i64; 2]> = tsp
                .node_coords()
                .values()
                .map(|point| [point.pos()[0] as i64, point.pos()[1] as i64])
                .collect();
            let mut rng = thread_rng();

            nodes.shuffle(&mut rng);

            let (r, s) = local_search::invert_tsp_local_search_n_neighbors(nodes.clone());
            println!("{}, {}", local_search::cost(&r), s);
        }
        Err(e) => {
            println!("Problem with file");
            process::exit(1);
        }
    }
}

