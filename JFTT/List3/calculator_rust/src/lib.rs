use std::fmt::{Display, Formatter, Error};
use std::ops::{Add, Sub, Mul, Neg, Div};

const P: i64 = 1234577;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct GF {
    value: i64,
    error: bool,
}

impl GF {
    pub fn new(x: i64) -> GF {
        GF { value: x.rem_euclid(P), error: false }
    }

    // FIXME: Too slow
    pub fn pow(self, n: GF) -> GF {
        let n = n.value;
        let mut result = GF::new(1);

        for _ in 0..n {
            result = result * self;
        }

        result
    }

    fn inv(self) -> GF {
        if self.value == 0 {
            GF { value: 0, error: true }
        } else {
            self.pow(GF::new(P - 2))
        }
    }
}

impl Add for GF {
    type Output = GF;

    fn add(self, rhs: GF) -> GF {
        if self.error || rhs.error {
            GF { value: 0, error: true }
        } else {
            GF::new((self.value + rhs.value).rem_euclid(P))
        }
    }
}

impl Sub for GF {
    type Output = GF;

    fn sub(self, rhs: GF) -> GF {
        if self.error || rhs.error {
            GF { value: 0, error: true }
        } else {
            GF::new((self.value - rhs.value).rem_euclid(P))
        }
    }
}

impl Mul for GF {
    type Output = GF;

    fn mul(self, rhs: GF) -> GF {
        if self.error || rhs.error {
            GF { value: 0, error: true }
        } else {
            GF::new((self.value * rhs.value).rem_euclid(P))
        }
    }
}

impl Neg for GF {
    type Output = GF;

    fn neg(self) -> GF {
        if self.error {
            GF { value: 0, error: true }
        } else {
            GF::new(P - self.value)
        }
    }
}

impl Div for GF {
    type Output = GF;

    fn div(self, rhs: GF) -> GF {
        self * rhs.inv()
    }
}

impl Display for GF {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        if self.error {
            write!(f, "Error")
        } else {
            write!(f, "{}", self.value)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_one() {
        let result = GF::new(2) + GF::new(3) * (GF::new(4) - GF::new(5));
        assert_eq!(result, GF::new(1234576));
    }

    #[test]
    fn test_two() {
        let result = GF::new(2).pow(GF::new(100));
        assert_eq!(result, GF::new(295422));
    }

    #[test]
    fn test_three() {
        let result = GF::new(2) - GF::new(3) - GF::new(2);
        assert_eq!(result, GF::new(1234574));
    }

    #[test]
    fn test_four() {
        let result = GF::new(269164) / GF::new(123456);
        assert_eq!(result, GF::new(567890));
    }

    #[test]
    fn test_five() {
        let result = -GF::new(2) - -GF::new(1);
        assert_eq!(result, GF::new(1234576));
    }

    #[test]
    fn test_six() {
        let result = GF::new(1) / -GF::new(580978);
        assert_eq!(result, GF::new(123456));
    }

    #[test]
    fn test_seven() {
        let result = GF::new(123456789);
        assert_eq!(result, GF::new(1233666));
    }

    #[test]
    fn test_eight() {
        let result = GF::new(2).pow(GF::new(123));
        assert_eq!(result, GF::new(594706));
    }

    #[test]
    fn division_by_zero() {
        let result = GF::new(1) / GF::new(0);
        assert_eq!(result.error, true);
    }

    #[test]
    fn test_nine() {
        let result = GF::new(2).pow(GF::new(1234574));
        assert_eq!(result, GF::new(925933));
    }
}
