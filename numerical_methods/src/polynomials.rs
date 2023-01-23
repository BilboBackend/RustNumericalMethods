// Library for polynomials with integer coefficients.

#[derive(Debug, Clone, PartialEq)]
pub struct Poly<I32> {
    pub coeffs: Vec<I32>,
}

//Evaluate a polynomial in x
pub fn eval_pol(x: i32, pol: Poly<i32>) -> i32 {
    let n = pol.coeffs.len();

    match pol.coeffs.get(0) {
        Some(v) => {
            v * x.pow(n.try_into().unwrap())
                + eval_pol(
                    x,
                    Poly {
                        coeffs: pol.coeffs[1..n].to_vec(),
                    },
                )
        }
        None => 0,
    }
}

pub fn mult_const_pol(c: i32, pol: Poly<i32>) -> Vec<i32> {
    let mut res = vec![];
    let n = pol.coeffs.len();

    match pol.coeffs.get(0) {
        Some(v) => {
            res.push(c * v);
            mult_const_pol(
                c,
                Poly {
                    coeffs: pol.coeffs[1..n].to_vec(),
                },
            )
        }
        None => res,
    }
}
