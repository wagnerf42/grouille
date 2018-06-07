use std::borrow::Borrow;
use std::cmp::Ordering;

pub mod iterators;

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

pub(crate) fn min_max<C: PartialOrd + Clone, T: Borrow<C>, U: Borrow<C>>(a: T, b: U) -> [C; 2] {
    match a.borrow()
        .partial_cmp(b.borrow())
        .expect("failed comparing partially ordered things")
    {
        Ordering::Greater => [b.borrow().clone(), a.borrow().clone()],
        _ => [a.borrow().clone(), b.borrow().clone()],
    }
}
