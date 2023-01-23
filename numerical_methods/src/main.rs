mod num_integration;
mod polynomials;
use crate::polynomials::Poly;

fn main() {
    print!("Hello, hello, good morning, good morning!");

    let pol1 = Poly {
        coeffs: vec![1, 2, 3],
    };
    let res = polynomials::eval_pol(1, pol1);

    print!("{}", res)
}
