use algebra::algebra::Matrix;
use std::time::Instant;

fn main() {
    let test_size = 101;
    let n = Matrix::hom(1., test_size, test_size);
    let start = Instant::now();
    let _c1 = n.times(&n);
    let time1 = Instant::now();
    let _c2 = n.times_normal(&n);
    let time2 = Instant::now();
    println!("Paralelni vypocet: {}ms", time1.duration_since(start).as_millis());
    println!("Normalni vypocet: {}ms", time2.duration_since(time1).as_millis());
    assert_eq!(_c1, Matrix::hom(test_size as f64, test_size, test_size));
}

