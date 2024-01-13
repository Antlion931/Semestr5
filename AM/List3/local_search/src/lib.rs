use async_std::task;
use futures::stream::FuturesUnordered;
use rand::{thread_rng, Rng};
use std::{sync::Arc, collections::BTreeSet};
use futures::{
    stream::{Stream, StreamExt, FusedStream},
    select,
};
use std::fmt::Debug;

#[derive(Debug)]
pub struct CircularBuffer<T> {
    body: Vec<T>,
    capacity: usize,
    size: usize,
    youngest: usize,
    oldest: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

fn circular_next(value: usize, max: usize) -> usize {
    (value + 1) % max
}

impl<T: Clone + Debug> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        CircularBuffer {
            body: Vec::with_capacity(capacity),
            capacity,
            youngest: 0,
            oldest: 0,
            size: 0,
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.size == self.capacity {
            return Err(Error::FullBuffer);
        }

        if self.size != 0 {
            self.youngest = circular_next(self.youngest, self.capacity);
        }

        self.size += 1;

        if let Some(x) = self.body.get_mut(self.youngest) {
            *x = element;
        } else {
            self.body.push(element);
        }

        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.size == 0 {
            return Err(Error::EmptyBuffer);
        }

        self.size -= 1;

        let result = self.body[self.oldest].clone();
        self.oldest = circular_next(self.oldest, self.capacity);
        Ok(result)
    }

    pub fn clear(&mut self) {
        self.oldest = 0;
        self.youngest = 0;
        self.size = 0;
        self.body.clear();
    }

    pub fn overwrite(&mut self, element: T) {
        if self.size != 0 {
            self.youngest = circular_next(self.youngest, self.capacity);
        }

        if self.size == self.capacity {
            self.oldest = circular_next(self.oldest, self.capacity);
        } else {
            self.size += 1;
        }

        if let Some(x) = self.body.get_mut(self.youngest) {
            *x = element;
        } else {
            self.body.push(element);
        }
    }
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

pub fn cost(permutation: &Vec<[i64; 2]>) -> i64 {
    permutation.windows(2).map(|x| metric(x[0], x[1])).sum::<i64>() + metric(permutation[0], *permutation.last().unwrap())
}

fn metric(x: [i64; 2], y: [i64; 2]) -> i64 {
    (((x[0] - y[0]).pow(2) + (x[1] - y[1]).pow(2)) as f64)
        .sqrt()
        .round() as i64
}

async fn my_test(i: usize, start_point: Arc<Vec<[i64; 2]>>, tabu_list: Arc<CircularBuffer<[usize; 2]>>, global_min_diff: i64) -> (i64, (usize, usize)) {
    let mut min_diff = i64::MAX;
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
            if global_min_diff <= min_diff && tabu_list.body.iter().any(|x| x == &[i, k]) {
                continue;
            }
            min_diff = diff;
            min_i_k = (i, k);
        }
    }

    (min_diff, min_i_k)
}

pub fn invert_tsp_tabu_search(start_point: Vec<[i64; 2]>) -> (Vec<[i64; 2]>, u64) {
    let mut steps = 0;
    let mut minminal_path = start_point.clone();
    let mut minimal_cost = cost(&minminal_path);
    let mut start_point_arc = Arc::new(start_point);
    let mut actural_cost = minimal_cost;
    let mut tabu_list = Arc::new(CircularBuffer::new(7));

    let mut iterations_without_improvement = 0;

    loop {
        let global_min_diff = minimal_cost - actural_cost;
        steps += 1;
        iterations_without_improvement += 1;

        let mut tasks: FuturesUnordered<_> = (0..start_point_arc.len())
            .map(|i| {
                let start_point_clone = Arc::clone(&start_point_arc);
                let tabu_list_clone = Arc::clone(&tabu_list);
                task::spawn(my_test(i, start_point_clone, tabu_list_clone, global_min_diff))
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

        Arc::get_mut(&mut tabu_list).unwrap().overwrite([i, k]);

        while i < k {
            Arc::get_mut(&mut start_point_arc).unwrap().swap(i, k);
            i += 1;
            k -= 1;
        }

        actural_cost += min_diff;

        if actural_cost < minimal_cost {
            iterations_without_improvement = 0;
            minimal_cost = actural_cost;
            minminal_path = (*start_point_arc).clone();
        }
        
        if iterations_without_improvement > start_point_arc.len() as u64 {
            return (minminal_path, steps);
        }
    }
}

pub fn invert_tsp_simulated_annealing(start_point: Vec<[i64; 2]>) -> (Vec<[i64; 2]>, u64) {
    let mut steps = 0;
    let mut minminal_path = start_point.clone();
    let mut minimal_cost = cost(&minminal_path);
    let mut actual_path = start_point.clone();
    let mut actual_cost = minimal_cost;

    let mut iterations_without_improvement = 0;
    let mut rng = thread_rng();

    let mut temperature = 0.5 * start_point.len() as f64;
    let cooling_rate = 0.95;
    let iterations_per_temperature = start_point.len() as u64;
    let mut iteration = 0;
    let max_iterations_without_improvement = start_point.len() as u64;

    while iterations_without_improvement < max_iterations_without_improvement {
        let mut i = rng.gen_range(0..start_point.len()-1);
        let mut k = rng.gen_range(i+1..start_point.len());

        if (k - i) + 2 >= start_point.len() {
            continue;
        }

        steps += 1;

        let mut diff = 0;

        if i == 0 {
            diff -= metric(actual_path[i], actual_path[start_point.len() - 1]);
            diff += metric(actual_path[k], actual_path[start_point.len() - 1]);
        } else {
            diff -= metric(actual_path[i - 1], actual_path[i]);
            diff += metric(actual_path[i - 1], actual_path[k]);
        }

        if k + 1 == start_point.len() {
            diff -= metric(actual_path[0], actual_path[k]);
            diff += metric(actual_path[0], actual_path[i]);
        } else {
            diff -= metric(actual_path[k + 1], actual_path[k]);
            diff += metric(actual_path[k + 1], actual_path[i]);
        }

        if iteration >= iterations_per_temperature {
            iterations_without_improvement += 1;
            iteration = 0;
            temperature *= cooling_rate;
        } else {
            iteration += 1;
        }

        if diff >= 0 && rng.gen_range(0.0..1.0) > (-diff as f64 / temperature).exp() {
            continue;
        }

        while i < k {
            actual_path.swap(i, k);
            i += 1;
            k -= 1;
        }

        actual_cost += diff;

        if actual_cost < minimal_cost {
            iterations_without_improvement = 0;
            minimal_cost = actual_cost;
            minminal_path = actual_path.clone();
        }
    }

    (minminal_path, steps)
}
