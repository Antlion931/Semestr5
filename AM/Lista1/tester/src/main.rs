use std::env;
use std::process;
use std::path::Path;
use rand::seq::SliceRandom;
use tspf::TspBuilder;
use rand::prelude;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() != 2 {
        println!("Wrong amount of operands, usage: tester <file_path>");
        process::exit(1)
    }

    let file_path = args.get(1).expect("We know there are args is len 2");

    match TspBuilder::parse_path(Path::new(file_path)) {
        Ok(tsp) => {
            let nodes: Vec<[f64; 2]> = tsp.node_coords().values().map(|point| [point.pos()[0], point.pos()[1]]).collect();

            let (mst, weight) = minimal_spaning_tree_weight(&nodes);
            println!("{}", weight);
            println!("{}", cost(&nodes, cycle_from_mst(mst)));
            
            let mut cycle: Vec<_> = (0..nodes.len()).collect();
            let mut rng = rand::thread_rng();
            cycle.shuffle(&mut rng);
            cycle.push(0);

            println!("{}", cost(&nodes, cycle));
        }
        Err(e) => eprint!("{:?}", e),
    }
}

fn metric(x: [f64; 2], y: [f64; 2]) -> u64 {
    ((x[0] - y[0]).powi(2) + (x[1] - y[1]).powi(2)).sqrt().round() as u64
}

pub struct Counter {
    index: usize,
}

impl Counter {
    pub fn new() -> Self {
        Self { index: 0 }
    }

    pub fn next(&mut self) -> usize {
        self.index += 1;
        self.index - 1
    }
}

fn cycle_from_mst(edges_matrix: Vec<Vec<bool>>) -> Vec<usize> {
    let mut result = Vec::new();
    let mut stack = Vec::new();

    stack.push(0);

    while let Some(x) = stack.pop() {
        result.push(x);
        edges_matrix[x].iter().enumerate().filter(|(_, b)| **b).for_each(|(n, _)| {
            if !stack.contains(&n) && !result.contains(&n) {
                stack.push(n);
            }
        });
    }

    result.push(0);
    result
}

fn cost(nodes: &[[f64; 2]], cycle: Vec<usize>) -> u64 {
    cycle.windows(2).map(|w| metric(nodes[w[0]], nodes[w[1]])).sum()
}

fn minimal_spaning_tree_weight(nodes: &[[f64; 2]]) -> (Vec<Vec<bool>>, u64) {
    let mut result_vec = vec![vec![false; nodes.len()]; nodes.len()];
    let mut result = 0;

    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
    enum CState {
        Unconected,
        Connected(usize),
    }
    let mut unions = vec![CState::Unconected; nodes.len()];
    let mut weighted = Vec::new();

    for x in 0..nodes.len() {
        for y in x + 1..nodes.len() {
            weighted.push((metric(nodes[x], nodes[y]), x, y));
        }
    }

    weighted.sort_unstable_by_key(|x| x.0);
    let mut counter = Counter::new();

    for w in weighted {
        match (unions[w.1], unions[w.2]) {
            (CState::Connected(x), CState::Connected(y)) if x == y => continue,
            (CState::Connected(x), CState::Connected(y)) => {
                unions.iter_mut().filter(|c| **c == CState::Connected(y)).for_each(|c| *c = CState::Connected(x));
                result += w.0;
                result_vec[w.1][w.2] = true;
                result_vec[w.2][w.1] = true;
            }
            (CState::Connected(x), CState::Unconected) => {
                unions[w.2] = CState::Connected(x);
                result += w.0;
                result_vec[w.1][w.2] = true;
                result_vec[w.2][w.1] = true;
            }
            (CState::Unconected, CState::Connected(y)) => {
                unions[w.1] = CState::Connected(y);
                result += w.0;
                result_vec[w.1][w.2] = true;
                result_vec[w.2][w.1] = true;
            }
            (CState::Unconected, CState::Unconected) => {
                let c = counter.next();
                unions[w.1] = CState::Connected(c);
                unions[w.2] = CState::Connected(c);
                result += w.0;
                result_vec[w.1][w.2] = true;
                result_vec[w.2][w.1] = true;
            }
        }
    }

    (result_vec, result)
}


#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn minimal_spaning_tree_weight_test() {
        let (mut vec, weight) = minimal_spaning_tree_weight(&[[0.0,0.0], [5.0, 0.0], [3.0,3.0], [7.0,3.0], [7.0,-3.0]]);

        assert_eq!(weight, 16);
    }
}
