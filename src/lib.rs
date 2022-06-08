use std::str::FromStr;
use std::num::ParseFloatError;

pub struct Quadratic {
    coeff_a: f64,
    coeff_b: f64,
    coeff_c: f64,
}

impl Quadratic {
    pub fn new(a: f64, b: f64, c: f64) -> Self {
        Quadratic { coeff_a: a, coeff_b: b, coeff_c: c }
    }
}

impl Quadratic {
    pub fn solve(&self) -> (f64, f64) {
        let discriminant: f64 = (self.coeff_b * self.coeff_b) - (4f64 * self.coeff_a * self.coeff_c);
        let x_0 = (-self.coeff_b + discriminant.sqrt()) / (self.coeff_a * 2f64);
        let x_1 = (-self.coeff_b - discriminant.sqrt()) / (self.coeff_a * 2f64);
        (x_0, x_1)
    }

    pub fn equation_as_string(&self) -> String {
        format!("{}x^2 + {}x + {}", self.coeff_a, self.coeff_b, self.coeff_c)
    }
}

impl FromStr for Quadratic {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coeffs: Vec<&str> = s.trim_matches(|p| p == '(' || p == ')')
                                    .trim()
                                    .split(',')
                                    .collect();

        let a = coeffs[0].parse::<f64>()?;
        let b = coeffs[1].parse::<f64>()?;
        let c = coeffs[2].parse::<f64>()?;

        Ok(Quadratic{ coeff_a: a, coeff_b: b, coeff_c: c })
    }
}


#[cfg(test)]
mod tests {
    use std::str::FromStr;
    
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

    #[test]
    fn parse_string_happy_case_1() {
        let q = Quadratic::from_str("-2.0,2.0,1.0").unwrap();
        let (x0, x1) = q.solve();
        //println!("{}", x0);
        //println!("{}", x1);
        assert_eq!((x0.abs() - (-0.3660254f64).abs() < 0.000001), true);
        assert_eq!((x1.abs() - (1.3660254f64).abs() < 0.000001), true);
    }

    #[test]
    fn parse_string_happy_case_2() {
        let q = Quadratic::from_str("(-2.0,2.0,1.0)").unwrap();
        let (x0, x1) = q.solve();
        //println!("{}", x0);
        //println!("{}", x1);
        assert_eq!((x0.abs() - (-0.3660254f64).abs() < 0.000001), true);
        assert_eq!((x1.abs() - (1.3660254f64).abs() < 0.000001), true);
    }

    #[test]
    fn parse_string_turbofish_happy_case_1() {
        let q = "-2.0,2.0,1.0".parse::<Quadratic>().unwrap();
        let (x0, x1) = q.solve();
        //println!("{}", x0);
        //println!("{}", x1);
        assert_eq!((x0.abs() - (-0.3660254f64).abs() < 0.000001), true);
        assert_eq!((x1.abs() - (1.3660254f64).abs() < 0.000001), true);
    }

    #[test]
    fn parse_string_turbofish_happy_case_2() {
        let q = "(-2.0,2.0,1.0)".parse::<Quadratic>().unwrap();
        let (x0, x1) = q.solve();
        //println!("{}", x0);
        //println!("{}", x1);
        assert_eq!((x0.abs() - (-0.3660254f64).abs() < 0.000001), true);
        assert_eq!((x1.abs() - (1.3660254f64).abs() < 0.000001), true);
    }
}
