use rand::Rng;

#[derive(Debug, Clone)]
pub struct Filter<T, F> 
where
    T: Copy,
    F: Fn(T, T) -> T,
{
    func: F,
    pub elements: Vec<T>,
    last_number: Option<T>,
}

impl<T, F> Filter<T, F>
where
    T: Copy + std::ops::Sub<Output = T>,
    F: Fn(T, T) -> T,
{
    pub fn new(func: F) -> Self {
        Self {
            func,
            elements: Vec::new(),
            last_number: None,
        }
    }

    pub fn update(&mut self, new_element: T) {
        let last_element = self.last_number.unwrap_or(new_element);

        self.last_number = Some(new_element);

        self.elements.push((self.func)(new_element, last_element));
    }

    pub fn update_with_last(&mut self) {
        self.elements.push((self.func)(self.last_number.unwrap(), self.last_number.unwrap()));
    }

    pub fn get_elements(self) -> Vec<T> {
        self.elements
    }

    pub fn get_diffrences_of_elements(mut self) -> Vec<T> {
        for i in (1..self.elements.len()).rev() {
            self.elements[i] = self.elements[i] - self.elements[i - 1];
        }

        self.elements
    }
}

pub fn averages_from_blocks(values: &[f64], value_to_block: &[usize], blocks: usize) -> Vec<f64> {
    let mut sums_in_blocks = vec![0.0; blocks];
    let mut counts_in_blocks = vec![0; blocks];

    for (v, b) in values.iter().zip(value_to_block.iter()) {
        sums_in_blocks[*b] += *v;
        counts_in_blocks[*b] += 1;
    }

    let mut rng = rand::thread_rng();

    sums_in_blocks
        .into_iter()
        .zip(counts_in_blocks.into_iter())
        .map(|(sum, count)|if count != 0 {sum / count as f64} else {rng.gen_range(-255.0..=255.0)})
        .collect()
}

pub fn blocks_from_averages(values: &[f64], averages: &[f64], old_blocks: &[usize]) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    let new_averages: Vec<_> = averages
        .iter()
        .map(|c| {
            let mut new_c = *c;

            if rng.gen_bool(0.5) {
                new_c += rng.gen_range(0.1..1.0);
            } else {
                new_c -= rng.gen_range(0.1..1.0);
            }

            new_c
        })
        .collect();

    values
        .iter()
        .enumerate()
        .map(|(n, v)| {
            let current_distance = (v - averages[old_blocks[n]]).abs();
            let new_distance = (v - new_averages[old_blocks[n]]).abs();
            if current_distance <= new_distance {
                old_blocks[n]
            } else {
                old_blocks[n] + averages.len()
            }
        })
        .collect()
}

pub fn quantize(values: &[f64], times: usize) -> Vec<f64> {
    let mut blocks = vec![0; values.len()];
    let mut averages = averages_from_blocks(&values, &blocks, 1);

    for _ in 0..times {
        blocks = blocks_from_averages(&values, &averages, &blocks);
        averages = averages_from_blocks(&values, &blocks, averages.len() * 2);
    }

    averages
}

#[cfg(test)]
mod test {
    use approx::relative_eq;
    use super::*;

    #[test]
    fn test_filter_y() {
        let mut y = Filter::new(|x: f64, y: f64| (x + y) / 2.0);

        let tab_x = [10.0, 14.0, 10.0, 12.0, 14.0, 8.0, 14.0, 12.0, 10.0, 8.0, 10.0, 12.0];

        for x in tab_x {
            y.update(x);
        }
        
        let result = [10.0, 12.0, 12.0, 11.0, 13.0, 11.0, 11.0, 13.0, 11.0, 9.0, 9.0, 11.0];

        for (x, y) in y.get_elements().iter().zip(result.iter()) {
            println!("{} {}", x, y);
            assert!(relative_eq!(x, y, epsilon = f64::EPSILON));
        }
    }

    #[test]
    fn test_diffrences_on_filter_y() {
        let mut y = Filter::new(|x: f64, y: f64| (x + y) / 2.0);

        let tab_x = [10.0, 14.0, 10.0, 12.0, 14.0, 8.0, 14.0, 12.0, 10.0, 8.0, 10.0, 12.0];

        for x in tab_x {
            y.update(x);
        }
        
        let result = [10.0, 2.0, 0.0, -1.0, 2.0, -2.0, 0.0, 2.0, -2.0, -2.0, 0.0, 2.0];

        for (x, y) in y.get_diffrences_of_elements().iter().zip(result.iter()) {
            println!("{} {}", x, y);
            assert!(relative_eq!(x, y, epsilon = f64::EPSILON));
        }
    }

    #[test]
    fn test_filter_z() {
        let mut z = Filter::new(|x: f64, y: f64| (x - y) / 2.0);

        let tab_x = [10.0, 14.0, 10.0, 12.0, 14.0, 8.0, 14.0, 12.0, 10.0, 8.0, 10.0, 12.0];

        for x in tab_x {
            z.update(x);
        }
        
        let result = [0.0, 2.0, -2.0, 1.0, 1.0, -3.0, 3.0, -1.0, -1.0, -1.0, 1.0, 1.0];

        for (x, z) in z.get_elements().iter().zip(result.iter()) {
            println!("{} {}", x, z);
            assert!(relative_eq!(x, z, epsilon = f64::EPSILON));
        }
    }
}
