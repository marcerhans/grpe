use std::ops::Mul;

use crate::vector::VectorRow;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Quaternion<T> {
    q0: T,
    q1: T,
    q2: T,
    q3: T,
}

impl Mul for &Quaternion<f64> {
    type Output = Quaternion<f64>;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            q0: self.q0 * rhs.q0 - self.q1 * rhs.q1 - self.q2 * rhs.q2 - self.q3 * rhs.q3,
            q1: self.q0 * rhs.q1 + self.q1 * rhs.q0 + self.q2 * rhs.q3 - self.q3 * rhs.q2,
            q2: self.q0 * rhs.q2 - self.q1 * rhs.q3 + self.q2 * rhs.q0 + self.q3 * rhs.q1,
            q3: self.q0 * rhs.q3 + self.q1 * rhs.q2 - self.q2 * rhs.q1 + self.q3 * rhs.q0,
        }
    }
}

impl Into<VectorRow<f64, 4>> for Quaternion<f64> {
    fn into(self) -> VectorRow<f64, 4> {
        VectorRow::from([
            self.q0,
            self.q1,
            self.q2,
            self.q3,
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mul_test() {
        let q1 = Quaternion {
            q0: 1.0,
            q1: 2.0,
            q2: 3.0,
            q3: 4.0,
        };
        let q2 = Quaternion {
            q0: 5.0,
            q1: 6.0,
            q2: 7.0,
            q3: 8.0,
        };
        let q3 = Quaternion {
            q0: -60.0,
            q1: 12.0,
            q2: 30.0,
            q3: 24.0,
        };
        assert!(&q1 * &q2 == q3, "Actual: {:?}", &q1 * &q2);
    }
}