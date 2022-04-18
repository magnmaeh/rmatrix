
use std::fmt::{Display, Formatter, Result};
use std::clone::Clone;
use std::ops::{Add, Sub};

struct Matrix<T> {
    n: usize,
    m: usize,
    data: Vec<Vec<T>>,
}

impl<T> Display for Matrix<T> where T: Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "n: {}, m: {}\ndata:\n", self.n, self.m)?;
        for i in 0..self.n {
            for j in 0..self.m {
                write!(f, "{} ", self.data[i][j])?
            }
            write!(f, "\n")?
        }
        Ok(())
    }
}

trait Zero {
    fn zero() -> Self;
}

impl Zero for usize {
    fn zero () -> Self {
        0
    }
}

#[derive(Clone)]
struct Complex {
    real: usize,
    img: usize
}

impl Zero for Complex {
    fn zero() -> Self {
        Complex { real: 0, img: 0 }
    }
}

impl Display for Complex {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}, {})", self.real, self.img)
    }
}

impl<T: Clone> Matrix<T> {
    fn new(n: usize, m: usize) -> Matrix<T> {
        Matrix {
            n, m, data: Vec::new()
        }
    }

    fn new_zeros(n: usize, m: usize) -> Matrix<T> where T: Zero {
        Matrix {
            n, m, data: vec![vec![Zero::zero(); m]; n]
        } 
    }

    fn new_square(n: usize) -> Matrix<T> {
        Matrix::new(n, n)
    }

    fn new_ident(n: usize) -> Matrix<T> {
        let mut m = Matrix::new_square(n);
        // change diagonal
        m
    }
}

impl<T: Clone + Zero + Copy + Add + Add<Output = T>> Add for Matrix<T> {
    type Output = Self;
    
    fn add(self, op: Self) -> Self::Output {
        let mut mat = Matrix::new_zeros(self.n, self.m);
        for i in 0..self.n {
            for j in 0..self.m {
                mat.data[i][j] = mat.data[i][j] + op.data[i][j];
            }
        }
        mat
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_usize() {
        let m: Matrix<usize> = Matrix::new_zeros(5, 6);
        println!("M: {}", m);    
    }

    #[test]
    fn create_complex() {
        let m: Matrix<Complex> = Matrix::new_zeros(7, 8);
        println!("M: {}", m);
    }

    #[test]
    fn add() {
        let mut m1: Matrix<usize> = Matrix::new_zeros(6, 6);
        let mut m2: Matrix<usize> = Matrix::new_zeros(6, 6);
        m1.data[5][2] = 66;
        m2.data[5][2] = 21;
        let result = m1 + m2;
        println!("M: {}", result);
    }
}
