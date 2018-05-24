use std::borrow::Borrow;
use std::cmp::Ordering;

pub(crate) fn min<T: Borrow<f64>, U: Borrow<f64>>(a: T, b: U) -> f64 {
    match a.borrow()
        .partial_cmp(b.borrow())
        .expect("failed comparing floats")
    {
        Ordering::Greater => b.borrow().clone(),
        _ => a.borrow().clone(),
    }
}

pub(crate) fn max<T: Borrow<f64>, U: Borrow<f64>>(a: T, b: U) -> f64 {
    match a.borrow()
        .partial_cmp(b.borrow())
        .expect("failed comparing floats")
    {
        Ordering::Less => b.borrow().clone(),
        _ => a.borrow().clone(),
    }
}
