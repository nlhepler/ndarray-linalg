
extern crate rand;
extern crate ndarray;
extern crate ndarray_rand;
extern crate ndarray_linalg;

use ndarray::prelude::*;
use ndarray_linalg::prelude::*;
use rand::distributions::*;
use ndarray_rand::RandomExt;

fn all_close(a: Array<f64, (Ix, Ix)>, b: Array<f64, (Ix, Ix)>) {
    if !a.all_close(&b, 1.0e-7) {
        panic!("\nTwo matrices are not equal:\na = \n{:?}\nb = \n{:?}\n",
               a,
               b);
    }
}

#[test]
fn svd_square() {
    let r_dist = Range::new(0., 1.);
    let a = Array::<f64, _>::random((3, 3), r_dist);
    let (u, s, vt) = a.clone().svd().unwrap();
    let mut sm = Array::eye(3);
    for i in 0..3 {
        sm[(i, i)] = s[i];
    }
    all_close(u.dot(&sm).dot(&vt), a);
}
#[test]
fn svd_square_t() {
    let r_dist = Range::new(0., 1.);
    let a = Array::<f64, _>::random((3, 3), r_dist).reversed_axes();
    let (u, s, vt) = a.clone().svd().unwrap();
    let mut sm = Array::eye(3);
    for i in 0..3 {
        sm[(i, i)] = s[i];
    }
    all_close(u.dot(&sm).dot(&vt), a);
}

#[test]
fn svd_4x3() {
    let r_dist = Range::new(0., 1.);
    let a = Array::<f64, _>::random((4, 3), r_dist);
    let (u, s, vt) = a.clone().svd().unwrap();
    let mut sm = Array::eye(3);
    for i in 0..3 {
        sm[(i, i)] = s[i];
    }
    all_close(u.dot(&sm).dot(&vt), a);
}
#[test]
fn svd_4x3_t() {
    let r_dist = Range::new(0., 1.);
    let a = Array::<f64, _>::random((4, 3), r_dist).reversed_axes();
    let (u, s, vt) = a.clone().svd().unwrap();
    let mut sm = Array::eye(3);
    for i in 0..3 {
        sm[(i, i)] = s[i];
    }
    all_close(u.dot(&sm).dot(&vt), a);
}

#[test]
fn svd_3x4() {
    let r_dist = Range::new(0., 1.);
    let a = Array::<f64, _>::random((3, 4), r_dist);
    let (u, s, vt) = a.clone().svd().unwrap();
    let mut sm = Array::eye(3);
    for i in 0..3 {
        sm[(i, i)] = s[i];
    }
    all_close(u.dot(&sm).dot(&vt), a);
}
#[test]
fn svd_3x4_t() {
    let r_dist = Range::new(0., 1.);
    let a = Array::<f64, _>::random((3, 4), r_dist).reversed_axes();
    let (u, s, vt) = a.clone().svd().unwrap();
    let mut sm = Array::eye(3);
    for i in 0..3 {
        sm[(i, i)] = s[i];
    }
    all_close(u.dot(&sm).dot(&vt), a);
}
