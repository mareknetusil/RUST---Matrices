
struct Matrix {
    data: Vec<Vec<f64>>,
    m: usize,
    n: usize,
}

impl Matrix {
    // METHODS
    fn print(self: &Matrix) {
        println!("Matice {}x{}:", self.m, self.n);
        for row in (self.data).iter() {
            for item in row.iter() {
                print!("{} ", &item);
            }
            println!("");
        }
    }

    fn times(self: &Matrix, b: &Matrix) -> Matrix {
        // TODO: run in parallel
        assert_eq!(self.n, self.m);
        let mut c = Matrix::zero(self.m, b.n);
        for i in 0..self.m {
            for j in 0..b.n {
                for k in 0..self.n {
                    c.data[i][j] += self.data[i][k] * b.data[k][j]
                }
            }
        }
        c
    }

    fn power(self: &Matrix, p: u16) -> Matrix {
        assert_eq!(self.m, self.n);
        if p == 0 {
            Matrix::identity(self.m)    
        } else if p % 2 == 0 {
            let c = self.power(p / 2);
            c.times(&c)
        } else {
            let c = self.power(p - 1);
            self.times(&c)
        }
    }

    // CONSTRUCTORS
    fn zero(m: usize, n: usize) -> Matrix {
        Matrix::hom(0.0, m, n)
    }

    fn hom(val: f64, m: usize, n: usize) -> Matrix {
        Matrix {
            data: vec![vec![val; n]; m],
            m,
            n,
        }
    }

    fn identity(m: usize) -> Matrix {
        let mut c = Matrix::zero(m, m);
        for i in 0..m {
            c.data[i][i] = 1.0;
        }
        c
    }
}


fn main() {
    let mut m = Matrix::hom(0.0, 2, 2);
    m.data = vec![vec![1.0, 1.0], vec![1.0, 0.0]];
    let c = m.power(50);
    c.print();
}

