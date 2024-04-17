use super::*;

fn assert_v_is(v1: Vec, x: f64, y: f64, z: f64) {
    assert_eq!(v1.x, x);
    assert_eq!(v1.y, y);
    assert_eq!(v1.z, z);
}

#[test]
fn new() {
    let v = Vec::new(3., 4., 5.);
    assert_eq!(v.x, 3.);
    assert_eq!(v.y, 4.);
    assert_eq!(v.z, 5.);
}

#[test]
fn add() {
    let v1 = Vec::new(3., 4., 5.);
    let v2 = Vec::new(4., 2., 1.);
    let v = v1 + v2;
    assert_v_is(v, 7., 6., 6.);
}

#[test]
fn sub() {
    let v1 = Vec::new(1., 2., 3.);
    let v2 = Vec::new(2., 0., -1.);
    let v = v1 - v2;
    assert_v_is(v, -1., 2., 4.);
}

#[test]
fn mul_vec() {
    let v1 = Vec::new(2., 1., 3.);
    let v2 = Vec::new(4., 2., 0.);
    let v = v1 * v2;
    assert_v_is(v, 8., 2., 0.);
}

#[test]
fn mul_f() {
    let v1 = Vec::new(2., 3., 1.);
    let v = v1 * 3.;
    assert_v_is(v, 6., 9., 3.);
}

#[test]
fn dot() {
    let v1 = Vec::new(2., 3., 1.);
    let v2 = Vec::new(3., 5., 0.5);
    let d = v1 & v2;
    assert_eq!(d, 21.5);
}
