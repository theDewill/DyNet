// src/matrix.rs

pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<f64>,
}

pub struct Lattice {
    skeleton : Vec<usize>,
    dimension : usize,
    data : Vec<Vec<Vec<f32>>>

}

let mut tst = Lattice { vec![2,5,6]};

impl Lattice {
    pub fn new(sk : Vec<usize>) -> Self {

        let invec : Vec<{unknown}> = Vec::with_capacity(sk[0]);
        




        Lattice { 
            skeleton : sk,
            dimension : sk.len()
         }
    
    }
 }






impl Matrix {
    pub fn new(rows: usize, cols: usize, data: Vec<f64>) -> Self {
        assert_eq!(rows * cols, data.len());
        Matrix { rows, cols, data }
    }

    pub fn show(&self) -> () {
        let art : String = "-----".repeat(self.cols);
        println!("+{}+",art);
        for i in 0..self.rows {
            print!("| ");
            for j in 0..self.cols {
                print!("{:?}, ", self.data[i * self.cols + j]);
                }
                print!("|");
                println!();
            println!("+{}+",art);
                }
    }

    pub fn zero(rows: usize, cols: usize) -> Self {
        Matrix {
            rows,
            cols,
            data: vec![0.0; rows * cols],
        }
    }

    pub fn multiply(&self, other: &Matrix) -> Matrix {
        assert_eq!(self.cols, other.rows);
        let mut data = vec![0.0; self.rows * other.cols];
        for i in 0..self.rows {
            for j in 0..other.cols {
                for k in 0..self.cols {
                    data[i * other.cols + j] += self.data[i * self.cols + k] * other.data[k * other.cols + j];
                }
            }
        }
        Matrix::new(self.rows, other.cols, data)
    }

    // Add an element-wise application of a function
    pub fn apply<F>(&mut self, f: F)
    where
        F: Fn(f64) -> f64,
    {
        for i in 0..self.data.len() {
            self.data[i] = f(self.data[i]);
        }
    }
}