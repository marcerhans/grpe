use std::ops::Mul;

use crate::vector::VectorRow;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Default, Clone)]
pub struct Quaternion<T> (
    pub T,
    pub T,
    pub T,
    pub T,
);

impl Quaternion<f64> {
    pub fn inverse(&self) -> Self {
        Self (
            self.0,
            -self.1,
            -self.2,
            -self.3,
        )
    }
}

impl Mul for &Quaternion<f64> {
    type Output = Quaternion<f64>;

    fn mul(self, rhs: Self) -> Self::Output {
        Quaternion (
            self.0 * rhs.0 - self.1 * rhs.1 - self.2 * rhs.2 - self.3 * rhs.3,
            self.0 * rhs.1 + self.1 * rhs.0 + self.2 * rhs.3 - self.3 * rhs.2,
            self.0 * rhs.2 - self.1 * rhs.3 + self.2 * rhs.0 + self.3 * rhs.1,
            self.0 * rhs.3 + self.1 * rhs.2 - self.2 * rhs.1 + self.3 * rhs.0,
        )
    }
}

impl Into<VectorRow<f64, 4>> for Quaternion<f64> {
    fn into(self) -> VectorRow<f64, 4> {
        VectorRow::from([self.0, self.1, self.2, self.3])
    }
}

impl Into<VectorRow<f64, 3>> for Quaternion<f64> {
    fn into(self) -> VectorRow<f64, 3> {
        VectorRow::from([self.1, self.2, self.3])
    }
}

impl From<&VectorRow<f64, 3>> for Quaternion<f64> {
    fn from(value: &VectorRow<f64, 3>) -> Self {
        Self (
            0.0,
            value[0],
            value[1],
            value[2],
        )
    }
}

pub fn rotate(
    point: &VectorRow<f64, 3>,
    q: &Quaternion<f64>,
    q_prim: &Quaternion<f64>,
) -> VectorRow<f64, 3> {
    (&(q * &point.into()) * q_prim).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mul_test() {
        let q1 = Quaternion (
            1.0,
            2.0,
            3.0,
            4.0,
        );
        let q2 = Quaternion (
            5.0,
            6.0,
            7.0,
            8.0,
        );
        let q3 = Quaternion (
            -60.0,
            12.0,
            30.0,
            24.0,
        );
        assert!(&q1 * &q2 == q3, "Actual: {:?}", &q1 * &q2);
    }

    #[test]
    fn rotation_test() {
        let r1 = Quaternion (
            (std::f64::consts::FRAC_PI_2 / 2.0).cos(),
            0.0,
            0.0,
            (std::f64::consts::FRAC_PI_2 / 2.0).sin(),
        );
        let r2 = Quaternion (
            (std::f64::consts::FRAC_PI_2 / 2.0).cos(),
            (std::f64::consts::FRAC_PI_2 / 2.0).sin(),
            0.0,
            0.0,
        );
        let r = &r1 * &r2;
        println!("{:?}", r);
        let r_prim = Quaternion (
            r.0,
            -r.1,
            -r.2,
            -r.3,
        );
        let p = Quaternion (
            0.0,
            1.0,
            0.0,
            0.0,
        );
        println!("{:?}", &(&r * &p) * &r_prim);
    }

    #[test]
    fn pure_vector_test() {
        let v = Quaternion (
            0.0,
            1.0,
            2.0,
            3.0,
        );

        let r = Quaternion (
            (std::f64::consts::FRAC_PI_2 / 2.0).cos(),
            0.0,
            0.0,
            (std::f64::consts::FRAC_PI_2 / 2.0).sin(),
        );
        let r_prim = Quaternion (
            r.0,
            -r.1,
            -r.2,
            -r.3,
        );

        let a = &(&r * &v) * &r_prim;
        let b = &r * &v;

        println!("{a:?} and {b:?}");
    }
}
