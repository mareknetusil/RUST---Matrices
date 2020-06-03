struct Matrix {
    data: Vec<Vec<f64>>,
    m: usize,
    n: usize,
}

fn print_matrix(m: &Matrix) {
    println!("Matice {}x{}:", m.m, m.n);
    for row in (m.data).iter() {
        for item in row.iter() {
            print!("{} ", &item);
        }
        println!("");
    }
}

fn mul(a: &Matrix, b: &Matrix) -> Matrix {
    // TODO: run in parallel
    assert_eq!(a.n, b.m);
    let mut c = zero_matrix(a.m, b.n);
    for i in 0..a.m {
        for j in 0..b.n {
            for k in 0..a.n {
                c.data[i][j] += a.data[i][k] * b.data[k][j]
            }
        }
    }
    c
}

fn power(m: &Matrix, p: u16) -> Matrix {
    assert_eq!(m.m, m.n);
    if p == 0 {
        identity_matrix(m.m)    
    } else if p % 2 == 0 {
        let c = power(m, p / 2);
        mul(&c, &c)
    } else {
        let c = power(m, p - 1);
        mul(&m, &c)
    }
}

fn zero_matrix(m: usize, n: usize) -> Matrix {
    hom_matrix(0.0, m, n)
}

fn hom_matrix(val: f64, m: usize, n: usize) -> Matrix {
    Matrix {
        data: vec![vec![val; n]; m],
        m,
        n,
    }
}

fn identity_matrix(m: usize) -> Matrix {
    let mut c = zero_matrix(m, m);
    for i in 0..m {
        c.data[i][i] = 1.0;
    }
    c
}

fn main() {
    let mut m = hom_matrix(0.0, 2, 2);
    m.data = vec![vec![1.0, 1.0], vec![1.0, 0.0]];
    let c = power(&m, 50);
    print_matrix(&c);
}
