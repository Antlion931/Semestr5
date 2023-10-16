use ratatui::widgets::ListState;
use std::fs::File;
use std::io::{BufWriter, Write};

const BYTE_POSSIBILITIES: usize = 256;

fn count(counts: &[u64]) -> Result<u128, AppError> {
    Ok(counts
        .iter()
        .try_fold(0u128, |acc, x| acc.checked_add(*x as u128))
        .ok_or(AppError::Overflow)?)
}

fn entropy_in(counts: &[u64]) -> Result<f64, AppError> {
    let mut sum = 0.0;
    let count: u128 = count(counts)?;

    if count == 0 {
        return Err(AppError::ZeroBytes);
    }

    for c in counts {
        if *c == 0 {
            continue;
        }

        let probability = *c as f64 / count as f64;
        let information = -probability.log2();

        sum += probability * information
    }

    if sum.is_finite() {
        Ok(sum)
    } else {
        Err(AppError::NaN)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Screen {
    Entropy,
    ConditionalEntropy,
    Exiting,
    Saving(SavingMode),
}

#[derive(Debug, Clone, Copy)]
pub enum SavingMode {
    Entropy,
    ConditionalEntropy,
    Results,
}

#[derive(Debug)]
pub enum AppError {
    Overflow,
    ZeroBytes,
    NaN,
    SavingProblem,
}

impl From<std::io::Error> for AppError {
    fn from(_value: std::io::Error) -> Self {
        Self::SavingProblem
    }
}

/// Application.
#[derive(Debug)]
pub struct App {
    /// should the application exit?
    pub file_name: String,
    pub should_quit: bool,
    pub entropy_saved: bool,
    pub conditional_entropy_saved: bool,
    pub results_saved: bool,

    pub entropy_list_state: ListState,
    pub conditional_entropy_list_state: ListState,

    current_screen: Screen,
    previous_screen: Screen,

    single_byte_counts: [u64; BYTE_POSSIBILITIES],

    last_byte: u8,
    double_byte_counts: [u64; BYTE_POSSIBILITIES * BYTE_POSSIBILITIES],

    amount_of_non_zero_single_bytes: usize,
    amount_of_non_zero_double_bytes: usize,
    count: u128,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self {
            file_name: String::new(),
            should_quit: false,
            entropy_saved: false,
            results_saved: false,
            entropy_list_state: list_state.clone(),
            conditional_entropy_list_state: list_state,
            conditional_entropy_saved: false,
            current_screen: Screen::Entropy,
            previous_screen: Screen::Entropy,
            single_byte_counts: [0; BYTE_POSSIBILITIES],
            last_byte: 0,
            double_byte_counts: [0; BYTE_POSSIBILITIES * BYTE_POSSIBILITIES],
            amount_of_non_zero_single_bytes: 0,
            amount_of_non_zero_double_bytes: 0,
            count: 0,
        }
    }

    pub fn change_screen(&mut self, new: Screen) {
        self.previous_screen = self.current_screen;
        self.current_screen = new;
    }

    pub fn save(&mut self) -> Result<(), AppError> {
        let mut out = BufWriter::new(File::create(self.file_name.clone())?);

        match self.current_screen {
            Screen::Saving(SavingMode::Entropy) => {
                for (key, value) in self.get_single_byte_counts().iter().enumerate() {
                    if *value == 0 {
                        continue;
                    }

                    writeln!(out, "{:#04X};{}", key, value)?;
                }

                self.entropy_saved = true;
            }
            Screen::Saving(SavingMode::ConditionalEntropy) => {
                for (key, value) in self.get_double_byte_counts().iter().enumerate() {
                    if *value == 0 {
                        continue;
                    }

                    writeln!(out, "{:#06X};{}", key, value)?;
                }

                self.conditional_entropy_saved = true;
            }
            Screen::Saving(SavingMode::Results) => {
                writeln!(out, "Entropy;{}", self.entropy()?)?;
                writeln!(out, "Conditional Entropy;{}", self.conditional_entropy()?)?;
                writeln!(out, "difference;{}", self.difference()?)?;

                self.results_saved = true;
            }
            _ => unreachable!()
        }

        out.flush()?;

        Ok(())
    }

    pub fn toggle_screen(&mut self) {
        match self.current_screen {
            Screen::Entropy => self.current_screen = Screen::ConditionalEntropy,
            Screen::ConditionalEntropy => self.current_screen = Screen::Entropy,
            _ => {}
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.

    pub fn soft_quit(&mut self) {
        match self.current_screen {
            Screen::Entropy => self.change_screen(Screen::Exiting),
            Screen::ConditionalEntropy => self.change_screen(Screen::Exiting),
            _ => unreachable!(),
        };

        if self.entropy_saved && self.conditional_entropy_saved && self.results_saved {
            self.should_quit = true;
        }
    }

    pub fn hard_quit(&mut self) {
        self.should_quit = true;
    }


    pub fn get_single_byte_counts(&self) -> &[u64] {
        &self.single_byte_counts
    }

    pub fn get_double_byte_counts(&self) -> &[u64] {
        &self.double_byte_counts
    }

    pub fn get_current_screen(&self) -> &Screen {
        &self.current_screen
    }

    pub fn get_previous_screen(&self) -> &Screen {
        &self.previous_screen
    }

    pub fn difference(&mut self) -> Result<f64, AppError> {
        Ok((self.entropy()? - self.conditional_entropy()?).abs())
    }

    pub fn read_byte(&mut self, byte: u8) -> Result<(), AppError> {
        self.count = self.count.checked_add(1).ok_or(AppError::Overflow)?;
        self.single_byte_counts[byte as usize] = self.single_byte_counts[byte as usize]
            .checked_add(1)
            .ok_or(AppError::Overflow)?;

        if self.single_byte_counts[byte as usize] == 1 {
            self.amount_of_non_zero_single_bytes += 1;
        }

        let double_byte = u16::from_be_bytes([self.last_byte, byte]);

        self.double_byte_counts[double_byte as usize] = self.double_byte_counts
            [double_byte as usize]
            .checked_add(1)
            .ok_or(AppError::Overflow)?;

        if self.double_byte_counts[double_byte as usize] == 1 {
            self.amount_of_non_zero_double_bytes += 1;
        }

        self.last_byte = byte;

        Ok(())
    }

    pub fn entropy(&self) -> Result<f64, AppError> {
        entropy_in(&self.single_byte_counts)
    }

    pub fn conditional_entropy(&self) -> Result<f64, AppError> {
        let mut sum = 0.0;
        let global_count = self.count;

        for chunk in self.double_byte_counts.chunks_exact(BYTE_POSSIBILITIES) {
            let count = count(chunk)?;

            if count == 0 {
                continue;
            }

            sum += count as f64 / global_count as f64 * entropy_in(chunk)?;
        }

        if sum.is_finite() {
            Ok(sum)
        } else {
            Err(AppError::NaN)
        }
    }

    pub fn scrol_down(&mut self) {
        match self.current_screen {
            Screen::Entropy => {
                let mut buffer = self.entropy_list_state.selected().expect("Is always Some") + 1;
                if buffer >= self.amount_of_non_zero_single_bytes {
                    buffer = 0;
                }
                self.entropy_list_state.select(Some(buffer));
            }
            Screen::ConditionalEntropy => {
                let mut buffer = self
                    .conditional_entropy_list_state
                    .selected()
                    .expect("Is always Some")
                    + 1;
                if buffer >= self.amount_of_non_zero_double_bytes {
                    buffer = 0;
                }
                self.conditional_entropy_list_state.select(Some(buffer));
            }
            _ => {}
        }
    }

    pub fn scrol_up(&mut self) {
        match self.current_screen {
            Screen::Entropy => {
                let buffer = self
                    .entropy_list_state
                    .selected()
                    .expect("Is always Some")
                    .checked_sub(1)
                    .unwrap_or(self.amount_of_non_zero_single_bytes - 1);
                self.entropy_list_state.select(Some(buffer));
            }
            Screen::ConditionalEntropy => {
                let buffer = self
                    .conditional_entropy_list_state
                    .selected()
                    .expect("Is always Some")
                    .checked_sub(1)
                    .unwrap_or(self.amount_of_non_zero_double_bytes - 1);
                self.conditional_entropy_list_state.select(Some(buffer));
            }
            _ => {}
        }
    }
}

mod tests {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn test_app_single_byte_countes_updated_correctly() {
        const BYTE: u8 = 0b_1111_1111;
        let mut app = App::new();

        for x in &app.single_byte_counts {
            assert_eq!(*x, 0);
        }

        app.read_byte(BYTE).unwrap();

        for (n, x) in app.single_byte_counts.iter().enumerate() {
            match n as u8 {
                BYTE => assert_eq!(*x, 1),
                _ => assert_eq!(*x, 0),
            }
        }
    }

    #[test]
    fn test_app_double_byte_countes_update_correctly() {
        const FIRST_BYTE: u8 = 0b_1111_1111;
        const FIRST_DOUBLE_BYTE: u16 = FIRST_BYTE as u16;
        const SECOND_BYTE: u8 = 0b_1010_1010;
        const SECOND_DOUBLE_BYTE: u16 = 0b_1111_1111_1010_1010;
        let mut app = App::new();

        for x in &app.double_byte_counts {
            assert_eq!(*x, 0);
        }

        app.read_byte(FIRST_BYTE).unwrap();

        for (n, x) in app.double_byte_counts.iter().enumerate() {
            match n as u16 {
                FIRST_DOUBLE_BYTE => assert_eq!(*x, 1),
                _ => assert_eq!(*x, 0),
            }
        }

        app.read_byte(SECOND_BYTE).unwrap();

        for (n, x) in app.double_byte_counts.iter().enumerate() {
            match n as u16 {
                FIRST_DOUBLE_BYTE => assert_eq!(*x, 1),
                SECOND_DOUBLE_BYTE => assert_eq!(*x, 1),
                _ => assert_eq!(*x, 0),
            }
        }
    }

    #[test]
    fn test_app_correct_entropy() {
        let mut app = App::new();
        app.read_byte(1).unwrap();
        app.read_byte(2).unwrap();
        app.read_byte(3).unwrap();
        app.read_byte(2).unwrap();
        app.read_byte(3).unwrap();
        app.read_byte(4).unwrap();
        app.read_byte(5).unwrap();
        app.read_byte(4).unwrap();
        app.read_byte(5).unwrap();
        app.read_byte(6).unwrap();
        app.read_byte(7).unwrap();
        app.read_byte(8).unwrap();
        app.read_byte(9).unwrap();
        app.read_byte(8).unwrap();
        app.read_byte(9).unwrap();
        app.read_byte(10).unwrap();

        assert!((app.entropy().unwrap() - 3.25).abs() < 0.0001)
    }

    #[test]
    fn test_app_correct_conditional_entropy() {
        let mut app = App::new();
        app.read_byte(1).unwrap();
        app.read_byte(2).unwrap();
        app.read_byte(3).unwrap();
        app.read_byte(2).unwrap();
        app.read_byte(3).unwrap();
        app.read_byte(4).unwrap();
        app.read_byte(1).unwrap();
        app.read_byte(3).unwrap();
        app.read_byte(2).unwrap();
        app.read_byte(0).unwrap();

        assert!((app.conditional_entropy().unwrap() - 0.7509775).abs() < 0.0001)
    }
}
