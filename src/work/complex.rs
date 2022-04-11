use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
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
        Self {
            real: self.real * other.real - self.imag * other.imag,
            imag: self.real.mul_add(other.imag, self.imag * other.real),
        }
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

impl Display for Complex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}+{}i)", self.real, self.imag)
    }
}



