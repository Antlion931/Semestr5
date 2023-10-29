use rand::prelude::SliceRandom;
use ratatui::widgets::ListState;
use tsp_tester::*;

#[derive(Debug, Clone, Copy)]
pub enum Screen {
    Results,
    View,
}

pub enum AppError {
    Overflow,
}

/// Application.
#[derive(Debug)]
pub struct App<'a> {
    pub nodes: &'a [[i64; 2]],
    pub mst_weight: i64,
    pub mst_matrix: Vec<Vec<bool>>,
    pub cycle: Vec<usize>,
    pub cycle_weight: i64,

    pub random_permutations: Vec<Vec<usize>>,
    pub random_permutations_weight: Vec<i64>,
    pub avg_from_min_in_ten: f64,
    pub avg_from_min_in_fifty: f64,
    pub minimal: usize,

    current_screen: Screen,
    pub should_quit: bool,
    pub max_x: i64,
    pub max_y: i64,
    pub min_x: i64,
    pub min_y: i64,

    pub view_list_state: ListState,
}

impl<'a> App<'a> {
    /// Constructs a new instance of [`App`].
    pub fn new(nodes: &'a [[i64; 2]]) -> Self {
        let max_x = nodes.iter().map(|n| n[0]).max().unwrap();
        let max_y = nodes.iter().map(|n| n[1]).max().unwrap();
        let min_x = nodes.iter().map(|n| n[0]).min().unwrap();
        let min_y = nodes.iter().map(|n| n[1]).min().unwrap();
        let (mst_matrix, mst_weight) = minimal_spaning_tree_weight(nodes);
        let cycle = cycle_from_mst(&mst_matrix);
        let cycle_weight = cost(nodes, &cycle);

        let mut random_permutations = Vec::with_capacity(1000);
        let mut random_permutations_weight = Vec::with_capacity(1000);

        for _ in 0..1000 {
            let mut cycle: Vec<_> = (0..nodes.len()).collect();
            let mut rng = rand::thread_rng();
            cycle.shuffle(&mut rng);
            cycle.push(0);

            random_permutations_weight.push(cost(nodes, &cycle));
            random_permutations.push(cycle);
        }

        let avg_from_min_in_ten = random_permutations_weight
            .chunks_exact(10)
            .map(|x| *x.iter().min().expect("have 1000 elements"))
            .sum::<i64>() as f64
            / 100.0f64;

        let avg_from_min_in_fifty = random_permutations_weight
            .chunks_exact(50)
            .map(|x| *x.iter().min().expect("have 1000 elements"))
            .sum::<i64>() as f64
            / 20.0f64;

        let minimal = random_permutations_weight
            .iter()
            .enumerate()
            .min_by(|a, b| a.1.cmp(b.1))
            .expect("have 1000 elements")
            .0;

        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            nodes,
            mst_matrix,
            mst_weight,
            cycle,
            cycle_weight,
            random_permutations,
            random_permutations_weight,
            avg_from_min_in_ten,
            avg_from_min_in_fifty,
            minimal,
            current_screen: Screen::Results,
            should_quit: false,
            max_x,
            max_y,
            min_y,
            min_x,
            view_list_state: list_state,
        }
    }

    pub fn get_current_screen(&self) -> Screen {
        self.current_screen
    }

    pub fn reset_zoom(&mut self) {
        self.max_x = self.nodes.iter().map(|n| n[0]).max().unwrap();
        self.max_y = self.nodes.iter().map(|n| n[1]).max().unwrap();
        self.min_x = self.nodes.iter().map(|n| n[0]).min().unwrap();
        self.min_y = self.nodes.iter().map(|n| n[1]).min().unwrap();
    }

    pub fn toggle_screen(&mut self) {
        match self.current_screen {
            Screen::View => self.current_screen = Screen::Results,
            Screen::Results => self.current_screen = Screen::View,
        }
    }

    pub fn scrol_down(&mut self) {
        let mut buffer = self.view_list_state.selected().expect("Is always Some") + 1;
        if buffer >= 1002 {
            buffer = 0;
        }
        self.view_list_state.select(Some(buffer));
    }

    pub fn scrol_up(&mut self) {
        let buffer = self
            .view_list_state
            .selected()
            .expect("Is always Some")
            .checked_sub(1)
            .unwrap_or(1001);
        self.view_list_state.select(Some(buffer));
    }

    pub fn set_to_minimal(&mut self) {
        self.view_list_state.select(Some(self.minimal + 2));
    }
}

mod tests {
    #![allow(unused_imports)]
    use super::*;
}
