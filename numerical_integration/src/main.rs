fn main() {

}


fn trapezoid(f: impl Fn(f64) -> f64, n: u32, a: f64, b: f64) -> f64 {
    let nf = n as f64;
    let h: f64 = (b - a)/nf;
    let mut sum: f64 = 0.5 * f(a) + f(b);

    for i in 1..(n-1) {
        let ifl = i as f64;
        let x = a + ifl*h;
        sum = sum + f(x);
    }

    sum = sum*h;

    sum
}

fn simpson(f: impl Fn(f64) -> f64, ) {
    todo!()
}

fn test1(x: f64) -> f64 {
    let res: f64 = x*x + 4.0 * x;
}    

#[test1]{ 

    let lower: f64 = 2.2;
    let upper: f64 = 2.5;

    let res: f64 = trapezoid(test1, 100, 0.0, 1.0);
    assert!((lower..upper).contains(res))
}




