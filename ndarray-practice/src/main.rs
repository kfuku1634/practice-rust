use ndarray::prelude::*;

fn main() {
    let a = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]];
    let a = Array::from_shape_vec((3,3),  a.concat()).unwrap();
    eprintln!("a = {:?}", a);
}
