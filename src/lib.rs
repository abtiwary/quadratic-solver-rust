struct Quadratic {
    coeff_a: f64,
    coeff_b: f64,
    coeff_c: f64,
}

impl Quadratic {
    fn new(a: f64, b: f64, c: f64) -> Self {
        Quadratic { coeff_a: a, coeff_b: b, coeff_c: c }
    }
}

impl Quadratic {
    fn solve(&self) -> (f64, f64) {
        let discriminant: f64 = (self.coeff_b * self.coeff_b) - (4f64 * self.coeff_a * self.coeff_c);
        let x_0 = (-self.coeff_b + discriminant.sqrt()) / (self.coeff_a * 2f64);
        let x_1 = (-self.coeff_b - discriminant.sqrt()) / (self.coeff_a * 2f64);
        (x_0, x_1)
    }
}


#[cfg(test)]
mod tests {
    use crate::Quadratic;

    #[test]
    fn happy_case_1() {
        let q = Quadratic::new(-2f64, 2f64, 1f64);
        let (x0, x1) = q.solve();
        //println!("{}", x0);
        //println!("{}", x1);
        assert_eq!((x0.abs() - (-0.3660254f64).abs() < 0.000001), true);
        assert_eq!((x1.abs() - (1.3660254f64).abs() < 0.000001), true);
    }
}
