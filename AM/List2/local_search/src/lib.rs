use async_std::task;
use futures::stream::FuturesUnordered;
use rand::{thread_rng, Rng};
use std::{sync::Arc, collections::BTreeSet};
use futures::{
    stream::{Stream, StreamExt, FusedStream},
    select,
};

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

pub fn cycle_from_mst(edges_matrix: &[Vec<bool>], start: usize) -> Vec<usize> {
    let mut result = Vec::new();
    let mut stack = Vec::new();

    stack.push(start);

    while let Some(x) = stack.pop() {
        result.push(x);
        edges_matrix[x]
            .iter()
            .enumerate()
            .filter(|(_, b)| **b)
            .for_each(|(n, _)| {
                if !stack.contains(&n) && !result.contains(&n) {
                    stack.push(n);
                }
            });
    }

    result.push(0);
    result
}

pub fn minimal_spaning_tree_weight(nodes: &[[i64; 2]]) -> (Vec<Vec<bool>>, i64) {
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
                unions
                    .iter_mut()
                    .filter(|c| **c == CState::Connected(y))
                    .for_each(|c| *c = CState::Connected(x));
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

pub fn cost(permutation: &Vec<[i64; 2]>) -> i64 {
    permutation.windows(2).map(|x| metric(x[0], x[1])).sum::<i64>() + metric(permutation[0], *permutation.last().unwrap())
}

fn metric(x: [i64; 2], y: [i64; 2]) -> i64 {
    (((x[0] - y[0]).pow(2) + (x[1] - y[1]).pow(2)) as f64)
        .sqrt()
        .round() as i64
}

async fn my_test(i: usize, start_point: Arc<Vec<[i64; 2]>>) -> (i64, (usize, usize)) {
    let mut min_diff = 0;
    let mut min_i_k = (0, 0);
    for k in i+1..start_point.len() {
        if (k - i) + 2 >= start_point.len() {
            continue;
        }

        let mut diff = 0;

        if i == 0 {
            diff -= metric(start_point[i], start_point[start_point.len() - 1]);
            diff += metric(start_point[k], start_point[start_point.len() - 1]);
        } else {
            diff -= metric(start_point[i - 1], start_point[i]);
            diff += metric(start_point[i - 1], start_point[k]);
        }

        if k + 1 == start_point.len() {
            diff -= metric(start_point[0], start_point[k]);
            diff += metric(start_point[0], start_point[i]);
        } else {
            diff -= metric(start_point[k + 1], start_point[k]);
            diff += metric(start_point[k + 1], start_point[i]);
        }

        if min_diff > diff {
            min_diff = diff;
            min_i_k = (i, k);
        }
    }

    (min_diff, min_i_k)
}

pub fn invert_tsp_local_search(start_point: Vec<[i64; 2]>) -> (Vec<[i64; 2]>, u64) {
    let mut steps = 0;
    let mut start_point_arc = Arc::new(start_point);
    loop {
        steps += 1;
        let mut tasks: FuturesUnordered<_> = (0..start_point_arc.len())
            .map(|i| {
                let start_point_clone = Arc::clone(&start_point_arc);
                task::spawn(my_test(i, start_point_clone))
            })
            .collect();


        let (min_diff, (mut i, mut k)) = task::block_on(async {
            let mut min_diff = 0;
            let mut i = 0;
            let mut k = 0;

            loop {
                let item = select! {
                    x = tasks.next() => x,
                    complete => break,
                };
                if let Some(next) = item {
                    if next.0 < min_diff {
                        min_diff = next.0;
                        i = next.1.0;
                        k = next.1.1;
                    }
                }
            }

            (min_diff, (i, k))
        });

        if min_diff == 0 {
            return (Arc::into_inner(start_point_arc).unwrap(), steps);
        }

        while i < k {
            Arc::get_mut(&mut start_point_arc).unwrap().swap(i, k);
            i += 1;
            k -= 1;
        }
    }
}

pub fn invert_tsp_local_search_n_neighbors(mut start_point: Vec<[i64; 2]>) -> (Vec<[i64; 2]>, u64) {
    let mut steps = 0;
    let mut rng = thread_rng();
    loop {
        steps += 1;
        let mut generated = BTreeSet::new();
        let mut generated_vec = Vec::with_capacity(start_point.len());

        while generated_vec.len() < start_point.len() {
            let i = rng.gen_range(0..start_point.len()-1);
            let k = rng.gen_range(i+1..start_point.len());

            if generated.insert((i, k)) {
                generated_vec.push((i, k));
            }
        }

        let mut min_diff = 0;
        let mut min_i = 0;
        let mut min_k = 0;

        for (i, k) in generated_vec {
            if (k - i) + 2 >= start_point.len() {
                continue;
            }

            let mut diff = 0;

            if i == 0 {
                diff -= metric(start_point[i], start_point[start_point.len() - 1]);
                diff += metric(start_point[k], start_point[start_point.len() - 1]);
            } else {
                diff -= metric(start_point[i - 1], start_point[i]);
                diff += metric(start_point[i - 1], start_point[k]);
            }

            if k + 1 == start_point.len() {
                diff -= metric(start_point[0], start_point[k]);
                diff += metric(start_point[0], start_point[i]);
            } else {
                diff -= metric(start_point[k + 1], start_point[k]);
                diff += metric(start_point[k + 1], start_point[i]);
            }

            if min_diff > diff {
                min_diff = diff;
                min_i = i;
                min_k = k;
            }
        }

        if min_diff == 0 {
            return (start_point, steps);
        }

        while min_i < min_k {
            start_point.swap(min_i, min_k);
            min_i += 1;
            min_k -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
