pub struct Filter<T, F> 
where
    T: Copy,
    F: Fn(T, T) -> T,
{
    func: F,
    elements: Vec<T>,
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
