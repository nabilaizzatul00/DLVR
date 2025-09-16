use ndarray;
use ndarray::{Array2, arr2};

fn main() {
    let a: Array2<f64> = arr2(&[[1., 2.], [3., 4.]]);
    let b: Array2<f64> = arr2(&[[5., 6.], [7., 8.]]);
    let c = a.dot(&b);  // Matrix multiplication
    println!("Matrix product:\n{}", c);
}

