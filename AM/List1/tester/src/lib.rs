fn metric(x: [i64; 2], y: [i64; 2]) -> i64 {
    (((x[0] - y[0]).pow(2) + (x[1] - y[1]).pow(2)) as f64)
        .sqrt()
        .round() as i64
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

pub fn cycle_from_mst(edges_matrix: &[Vec<bool>]) -> Vec<usize> {
    let mut result = Vec::new();
    let mut stack = Vec::new();

    stack.push(0);

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

pub fn cost(nodes: &[[i64; 2]], cycle: &[usize]) -> i64 {
    cycle
        .windows(2)
        .map(|w| metric(nodes[w[0]], nodes[w[1]]))
        .sum()
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
