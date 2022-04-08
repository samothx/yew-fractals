#![allow(clippy::missing_const_for_fn)]
#![allow(dead_code)]

use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

use simple_big_int::Rational;

#[derive( Clone, PartialEq)]
pub struct ComplexRational {
    real: Rational,
    imag: Rational,
}

impl<'a> ComplexRational {
    pub fn new() -> Self {
        Self { real: 0u32.into(), imag: 0u32.into() }
    }

    pub fn from_f64(real: f64, imag: f64) -> Self {
        Self {
            real: Rational::from_f64(real),
            imag: Rational::from_f64(imag),
        }
    }

    pub fn from_rational(real: Rational, imag: Rational) -> Self {
        Self { real, imag }
    }


    #[inline]
    pub fn real(&'a self) -> &'a Rational {
        &self.real
    }
    #[inline]
    pub fn imag(&'a self) -> &'a Rational {
        &self.imag
    }
    #[inline]
    pub fn set_real(&mut self, real: Rational) {
        self.real = real;
    }
    #[inline]
    pub fn set_imag(&mut self, imag: Rational) {
        self.imag = imag;
    }
    #[inline]
    pub fn square_length(&self) -> Rational {
        self.real.mul_by(&self.real).add_to(&self.imag.mul_by(&self.imag))
    }
    #[inline]
    pub fn norm(&self) -> Rational {
        self.square_length().sqrt()
    }
}

impl Add for ComplexRational {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            real: self.real.add_to(&other.real),
            imag: self.imag.add_to(&other.imag),
        }
    }
}

impl AddAssign for ComplexRational {
    fn add_assign(&mut self, other: Self) {
        self.real.add_into(&other.real);
        self.imag.add_into(&other.imag);
    }
}

impl Sub for ComplexRational {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            real: self.real.sub_from(&other.real),
            imag: self.imag.sub_from(&other.imag),
        }
    }
}

impl SubAssign for ComplexRational {
    fn sub_assign(&mut self, other: Self) {
        self.real.sub_into(&other.real);
        self.imag.sub_into(&other.imag);
    }
}


impl Mul for ComplexRational {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self {
            real: self.real.mul_by(&other.real).sub_from(&self.imag.mul_by(&other.imag)),
            imag: self.real.mul_by(&other.imag).add_to(&self.imag.mul_by(&other.real)),
        }
    }
}

impl MulAssign for ComplexRational {
    fn mul_assign(&mut self, other: Self) {
        let real = self.real.mul_by(&other.real).sub_from( &self.imag.mul_by(&other.imag));
        let imag = self.real.mul_by(&other.imag).add_to( &self.imag.mul_by(&other.real));
        self.real = real;
        self.imag = imag;
    }
}

impl Display for ComplexRational {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}+{}i)", self.real, self.imag)
    }
}
