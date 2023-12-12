use std::fmt::{Display, Formatter, Error};

pub type GFResult = Result<GF, &'static str>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct GF {
    p: i64,
   value: i64,
}

fn extended_gcd(a: i64, b: i64) -> Result<(i64, i64, i64), &'static str> { // r, s, t
    if a == 0 && b == 0 {
        return Err("Tryied to calculate extended gcd with zeros");
    }

    let mut last_two_r = (a, b);
    let mut last_two_s = (1, 0);
    let mut last_two_t = (0, 1);

    while last_two_r.1 != 0 {
        let q = last_two_r.0 / last_two_r.1;

        last_two_r = (last_two_r.1, last_two_r.0 - q * last_two_r.1);
        last_two_s = (last_two_s.1, last_two_s.0 - q * last_two_s.1);
        last_two_t = (last_two_t.1, last_two_t.0 - q * last_two_t.1);
    }

    Ok((last_two_r.0, last_two_s.0, last_two_t.0))
}

pub fn pow(a: GFResult, b: GFResult) -> GFResult {
    match (a, b) {
        (Ok(a), Ok(b)) => {
            if a.p != b.p + 1 {
                return Err("Tryied to raise to power with wrong field");
            }

            let mut n = b.value;
            let mut result = 1;

            while n > 0 {
                result = (result * a.value).rem_euclid(a.p);
                n -= 1;
            }

            GF::try_new(result, a.p)
        },
        (a @ Err(_), _) => a,
        (_, b @ Err(_)) => b,
    }
}

pub fn inv(a: GFResult) -> GFResult {
    match a {
        Ok(a) => {
            if a.value == 0 {
                Err("Tryied to invert zero")
            }
            else {
                if let Ok((1, s, _)) = extended_gcd(a.value, a.p) {
                    GF::try_new(s.rem_euclid(a.p), a.p)
                } else {
                    Err("Tryied to invert non-invertible element")
                }
            }
        },
        a @ Err(_) => a,
    }
}

pub fn add(a: GFResult, b: GFResult) -> GFResult {
    match (a, b) {
        (Ok(a), Ok(b)) => {
            if a.p == b.p {
                GF::try_new(a.value + b.value, a.p)
            } else {
                Err("Tryied to add elements from different Galios Fields")
            }
        },
        (a @ Err(_), _) => a,
        (_, b @ Err(_)) => b,
    }
}

pub fn sub(a: GFResult, b: GFResult) -> GFResult {
    match (a, b) {
        (Ok(a), Ok(b)) => {
            if a.p == b.p {
                GF::try_new(a.value - b.value, a.p)
            } else {
                Err("Tryied to subtract elements from different Galios Fields")
            }
        },
        (a @ Err(_), _) => a,
        (_, b @ Err(_)) => b,
    }
}

pub fn mul(a: GFResult, b: GFResult) -> GFResult { 
    match (a, b) {
        (Ok(a), Ok(b)) => {
            if a.p == b.p {
                GF::try_new(a.value * b.value, a.p)
            } else {
                Err("Tryied to multiplay elements from different Galios Fields")
            }
        },
        (a @ Err(_), _) => a,
        (_, b @ Err(_)) => b,
    }
}

pub fn neg(a: GFResult) -> GFResult {
    match a {
        Ok(a) => {
            GF::try_new(a.p - a.value, a.p)
        },
        a @ Err(_) => a,
    }
}

pub fn div(a: GFResult, b: GFResult) -> GFResult {
    mul(a, inv(b))
}

impl GF {
    pub fn try_new(x: i64, p: i64) -> Result<GF, &'static str> {
        if p < 2 {
            return Err("Tryied to create galios field with too few elements");
        }

        Ok(GF { p, value: x.rem_euclid(p)})
    }
}

impl Display for GF {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.value)
    }
}

