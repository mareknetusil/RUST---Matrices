pub mod algebra {
    extern crate num_cpus;
    use std::thread;
    use std::cmp;
    use std::sync::{Arc, RwLock};

    #[derive(Debug)]
    pub struct Matrix {
        data: Arc<RwLock<Vec<Vec<f64>>>>,
        pub m: usize,
        pub n: usize,
    }

    impl PartialEq for Matrix {
        fn eq(&self, other: &Matrix) -> bool {
            *self.data.read().unwrap() == *other.data.read().unwrap()
        }
    }

    impl Matrix {
        // METHODS
        pub fn new(v: Vec<Vec<f64>>) -> Matrix {
            let m = v.len();
            let n = v[0].len();
            Matrix {
                data: Arc::new(RwLock::new(v)),
                m,
                n,
            }
        }

        pub fn print(self: &Matrix) {
            println!("Matice {}x{}:", self.m, self.n);
            let data = self.data.read().unwrap();
            for row in data.iter() {
                for item in row.iter() {
                    print!("{} ", &item);
                }
                println!("");
            }
        }

        pub fn times(self: &Matrix, b: &Matrix) -> Matrix {
            assert_eq!(self.n, b.m);
            let c = Matrix::zero(self.n, self.m);
            let a_rows = self.m;
            let a_cols = self.n;
            let b_cols = b.n;
            let mut threads = vec![];
            let cpus = cmp::min(a_rows, num_cpus::get());

            let rest_size = a_rows % cpus;
            let batch_size = (a_rows + cpus - rest_size) / cpus;
            println!("Pocet cpu: {}. Pocet radek na cpu: {}", cpus, batch_size);
            for cpu in 0..cpus {
                let a_clone = self.data.clone();
                let b_clone = b.data.clone();
                let c_clone = c.data.clone();
                threads.push(thread::spawn(move || {
                    let a_data = a_clone.read().unwrap();
                    let b_data = b_clone.read().unwrap();
                    let lower = cpu * batch_size;
                    let upper = cmp::min(lower + batch_size, a_rows);
                    for i in lower..upper {
                        let mut val;
                        for j in 0..b_cols {
                            val = 0f64;
                            for k in 0..a_cols {
                                val += a_data[i][k] * b_data[k][j];
                            }
                            let mut c_data = c_clone.write().unwrap();
                            c_data[i][j] = val;
                        }
                    }
                }));
            }
            for handle in threads { handle.join().unwrap(); }
            return c;
        }

        pub fn times_normal(self: &Matrix, b: &Matrix) -> Matrix {
            assert_eq!(self.n, b.m);
            let c = Matrix::zero(self.n, self.m);
            let a_rows = self.m;
            let a_cols = self.n;
            let b_cols = b.n;
            let a_data = self.data.read().unwrap();
            let b_data = b.data.read().unwrap();

            for i in 0..a_rows {
                let mut c_data = c.data.write().unwrap();
                for j in 0..b_cols {
                    for k in 0..a_cols {
                        c_data[i][j] += a_data[i][k] * b_data[k][j];
                    }
                }
            }
            return c;
        }

        pub fn power(self: &Matrix, p: u16) -> Matrix {
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
        pub fn zero(m: usize, n: usize) -> Matrix {
            Matrix::hom(0.0, m, n)
        }

        pub fn hom(val: f64, m: usize, n: usize) -> Matrix {
            Matrix {
                data: Arc::new(RwLock::new(vec![vec![val; n]; m])),
                m,
                n,
            }
        }

        pub fn identity(m: usize) -> Matrix {
            let c = Matrix::zero(m, m);
            {
                let mut c_lock = c.data.write().unwrap();
                for i in 0..m {
                    c_lock[i][i] = 1.0;
                }
            }
            c
        }
    }
}

#[cfg(test)]
mod tests {
    use super::algebra::*;

    #[test]
    fn multi() {
        let test_size = 101;
        let n = Matrix::hom(1., test_size, test_size);
        let _c1 = n.times(&n);
        assert_eq!(_c1, Matrix::hom(test_size as f64, test_size, test_size));
    }

    #[test]
    fn power() {
        let m = Matrix::new(vec![vec![0.0, 1.0], vec![1.0, 0.0]]);
        m.print();
        assert_eq!(m.power(2), Matrix::identity(2));
    }
}
