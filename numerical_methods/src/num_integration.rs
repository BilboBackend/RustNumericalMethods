pub fn trapezoid(f: impl Fn(f64) -> f64, a: f64, b: f64, n: u32) -> f64 {
    let nf = n as f64;
    let h: f64 = (b - a) / nf;
    let mut sum: f64 = 0.5 * f(a) + f(b);

    for i in 1..(n - 1) {
        let ifl = i as f64;
        let x = a + ifl * h;
        sum = sum + f(x);
    }

    sum = sum * h;

    sum
}

fn encap_adaptive_simpson(
    f: &dyn Fn(f64) -> f64,
    a: f64,
    b: f64,
    e: f64,
    level: u32,
    level_max: u32,
) -> f64 {
    let new_level = level + 1;

    let h = b - a;
    let c = (a + b) / 2.0;

    let one_simp = h / 6.0 * (f(a) + 4.0 * f((a + b) / 2.0) + f(b));
    let two_simp =
        h / 12.0 * (f(a) + 4.0 * f((a + c) / 2.0) + 2.0 * f(c) + 4.0 * f((b + c) / 2.0) + f(b));

    if level >= level_max {
        print!("Maximum level reached");
        two_simp
    } else {
        let diff = two_simp - one_simp;
        match diff < 15.0 * e {
            true => two_simp + (two_simp - one_simp) / 15.0,
            false => {
                let left_simp: f64 =
                    encap_adaptive_simpson(&f, a, c, e / 2.0, new_level, level_max);
                let right_simp: f64 =
                    encap_adaptive_simpson(&f, c, b, e / 2.0, new_level, level_max);
                left_simp + right_simp
            }
        }
    }
}

pub fn adaptive_simpson(f: impl Fn(f64) -> f64, a: f64, b: f64, e: f64) -> f64 {
    let level = 0;
    let level_max = 10;
    encap_adaptive_simpson(&f, a, b, e, level, level_max)
}

fn second_deg_pol(x: f64) -> f64 {
    let res: f64 = x * x + 4.0 * x;
    res
}

#[test]
fn test1() {
    let lower: f64 = 2.2;
    let upper: f64 = 2.5;

    let res: f64 = trapezoid(&second_deg_pol, 0.0, 1.0, 100);
    assert!((lower..upper).contains(&res))
}

#[test]
fn test2() {
    let lower: f64 = 2.28;
    let upper: f64 = 2.35;

    let e = 0.05;
    let res = adaptive_simpson(&second_deg_pol, 0.0, 1.0, e);
    assert!((lower..upper).contains(&res))
}
