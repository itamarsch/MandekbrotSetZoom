use std::ops::{Add, Mul};

#[derive(Clone, Copy)]
pub struct Complex {
    pub re: f64,
    pub img: f64,
}

impl Complex {
    pub fn mag(self) -> f64 {
        self.re.hypot(self.img)
    }
}

impl Add<Complex> for Complex {
    fn add(self, other: Complex) -> Self::Output {
        Complex {
            re: self.re + other.re,
            img: self.img + other.img,
        }
    }

    type Output = Complex;
}

impl Mul<f64> for Complex {
    type Output = Complex;

    //Scale
    fn mul(self, n: f64) -> Self::Output {
        Complex {
            re: self.re * n,
            img: self.img * n,
        }
    }
}

impl Mul<Complex> for Complex {
    type Output = Complex;

    //Complex multiplication
    fn mul(self, other: Complex) -> Self::Output {
        Complex {
            re: self.re * other.re - self.img * other.img,
            img: self.re * other.img + self.img * other.re,
        }
    }
}
