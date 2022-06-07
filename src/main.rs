use std::error::Error;

use clap::{Arg, Command};

use quadratic_solver::Quadratic;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("Quadratic Solver!")
            .version("0.1")
            .author("Ab Tiwary")
            .about("Solves a Quadratic equation")
            .arg(Arg::new("coeff_a"))
                .short_flag('a')
                .long_flag("coeff_a")
            .arg(Arg::new("coeff_b"))
                .short_flag('b')
                .long_flag("coeff_b")
            .arg(Arg::new("coeff_c"))
                .short_flag('c')
                .long_flag("coeff_c")
            .get_matches();

    let coeff_a = matches.value_of("coeff_a").unwrap().parse::<f64>().unwrap();
    let coeff_b = matches.value_of("coeff_b").unwrap().parse::<f64>().unwrap();
    let coeff_c = matches.value_of("coeff_c").unwrap().parse::<f64>().unwrap();
    println!("{}", coeff_a);
    println!("{}", coeff_b);
    println!("{}", coeff_c);

    //let q = Quadratic::new(-2f64, 2f64, 1f64);
    // for example, run with ./quadratic-solver -- -2 2 1
    let q = Quadratic::new(coeff_a, coeff_b, coeff_c);
    let (x0, x1) = q.solve();
    println!("First root: {}", x0);
    println!("Second root: {}", x1);
    println!("\tfor the equation: {}", q.equation_as_string());

    Ok(())
}
