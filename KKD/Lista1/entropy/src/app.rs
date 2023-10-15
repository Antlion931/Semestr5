const BYTE_POSSIBILITIES: usize = 256;

fn count(counts: &[u64]) -> Result<u128, AppError> {
    Ok(counts.iter().try_fold(0u128, |acc, x| acc.checked_add(*x as u128)).ok_or(AppError::Overflow)?)
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

#[derive(Debug)]
pub enum AppError {
    Overflow,
    ZeroBytes,
    NaN
}

/// Application.
#[derive(Debug)]
pub struct App {
    /// should the application exit?
    pub should_quit: bool,

    single_byte_counts: [u64; BYTE_POSSIBILITIES],

    last_byte: u8,
    double_byte_counts: [u64; BYTE_POSSIBILITIES * BYTE_POSSIBILITIES]
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self { should_quit: false, single_byte_counts: [0; BYTE_POSSIBILITIES], last_byte: 0, double_byte_counts: [0; BYTE_POSSIBILITIES * BYTE_POSSIBILITIES] }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn get_single_byte_counts(&self) -> &[u64] {
        &self.single_byte_counts
    }

    pub fn get_double_byte_counts(&self) -> &[u64] {
        &self.double_byte_counts
    }

    pub fn read_byte(&mut self, byte: u8) -> Result<(), AppError> {
        self.single_byte_counts[byte as usize] = self.single_byte_counts[byte as usize].checked_add(1).ok_or(AppError::Overflow)?;
        let double_byte = u16::from_be_bytes([self.last_byte, byte]);

        self.double_byte_counts[double_byte as usize] = self.double_byte_counts[double_byte as usize].checked_add(1).ok_or(AppError::Overflow)?;
        self.last_byte = byte;
    
        Ok(())
    }


    pub fn entropy(&self) -> Result<f64, AppError> {
        entropy_in(&self.single_byte_counts)
    }

    pub fn conditional_entropy(&self) -> Result<f64, AppError> {
        let mut sum = 0.0;
        let global_count = count(&self.single_byte_counts)?;

        for chunk in self.double_byte_counts.chunks(BYTE_POSSIBILITIES) {
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
