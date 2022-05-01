use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Copy, Clone, Serialize, Deserialize, PartialEq)]
pub struct Complex {
    real: f64,
    imag: f64,
}

impl Complex {
    pub fn new(real: f64, imag: f64) -> Self {
        Self { real, imag }
    }

    #[inline]
    pub fn real(&self) -> f64 {
        self.real
    }
    #[inline]
    pub fn imag(&self) -> f64 {
        self.imag
    }
    #[allow(dead_code)]
    #[inline]
    pub fn set_real(&mut self, real: f64) {
        self.real = real;
    }
    #[allow(dead_code)]
    #[inline]
    pub fn set_imag(&mut self, imag: f64) {
        self.imag = imag;
    }
    #[inline]
    pub fn square_length(&self) -> f64 {
        self.real.mul_add(self.real, self.imag * self.imag)
    }
    #[inline]
    pub fn norm(&self) -> f64 {
        f64::sqrt(self.square_length())
    }

    #[inline]
    pub fn mul_by(&self, other: &Complex) -> Complex {
        Self {
            real: self.real * other.real - self.imag * other.imag,
            imag: self.real.mul_add(other.imag, self.imag * other.real),
        }
    }

    pub fn powi(&self, power: u32) -> Complex {
        // a ^ (b + c) = a ^ b * a ^ c
        match power {
            0 => Complex {
                real: 1.0,
                imag: 0.0,
            },
            1 => self.clone(),
            2 => self.mul_by(self),
            3 => self.mul_by(self).mul_by(self),
            _ => {
                let mut res = Complex {
                    real: 1.0,
                    imag: 0.0,
                };
                let mut curr = self.clone();
                let mut curr_pow = power;
                loop {
                    if curr_pow & 0x1 == 0x1 {
                        res *= curr;
                    }
                    curr_pow >>= 1;
                    if curr_pow == 0 {
                        break;
                    }
                    curr *= curr;
                }
                res
            }
        }
    }

    /*
    pub fn powi(&self, power: u32) -> Complex {
        // recursive approach
        match power {
            1 => self.clone(),
            2 => self.mul_by(&self),
            _ => {
                let tmp = self.powi(power / 2);
                if power & 0x1 == 0x0 {
                    // self ^ (power/2) * self ^ (power/2)
                    tmp.mul_by(&tmp)
                } else {
                    // self ^ (power/2) * self ^ (power/2) * self
                    tmp.mul_by(&tmp).mul_by(&self)
                }
            }
        }
    }
     */
}

impl Add for Complex {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}

impl AddAssign for Complex {
    fn add_assign(&mut self, other: Self) {
        self.real = self.real + other.real;
        self.imag = self.imag + other.imag;
    }
}

impl Sub for Complex {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            real: self.real - other.real,
            imag: self.imag - other.imag,
        }
    }
}

impl SubAssign for Complex {
    fn sub_assign(&mut self, other: Self) {
        self.real = self.real - other.real;
        self.imag = self.imag - other.imag;
    }
}

impl Mul for Complex {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        self.mul_by(&other)
    }
}

impl MulAssign for Complex {
    fn mul_assign(&mut self, other: Self) {
        let real = self.real * other.real - self.imag * other.imag;
        let imag = self.real.mul_add(other.imag, self.imag * other.real);
        self.real = real;
        self.imag = imag;
    }
}

impl Mul<f64> for Complex {
    type Output = Self;
    fn mul(self, other: f64) -> Self::Output {
        Self {
            real: self.real * other,
            imag: self.imag * other,
        }
    }
}

impl MulAssign<f64> for Complex {
    fn mul_assign(&mut self, other: f64) {
        let real = self.real * other;
        let imag = self.imag * other;
        self.real = real;
        self.imag = imag;
    }
}

impl Display for Complex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}+i{})", self.real, self.imag)
    }
}

impl Debug for Complex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}+i{})", self.real, self.imag)
    }
}

#[cfg(test)]
pub mod test {
    use super::Complex;

    #[test]
    fn test_powi() {
        let c = Complex::new(2.0, 2.0);
        let res = c.powi(0);
        assert_eq!(res, Complex::new(1.0, 0.0));
        let res = c.powi(1);
        assert_eq!(res, c);
        let res = c.powi(2);
        assert_eq!(res, c.mul_by(&c));
        let res = c.powi(3);
        assert_eq!(res, c.mul_by(&c).mul_by(&c));
        let res = c.powi(4);
        assert_eq!(res, c.mul_by(&c).mul_by(&c).mul_by(&c));
        let res = c.powi(5);
        assert_eq!(res, c.mul_by(&c).mul_by(&c).mul_by(&c).mul_by(&c));
    }
}
