

use std::marker::PhantomData;
use pyo3::types::{PyDict, IntoPyDict};
use pyo3_macros::IntoPyDict;

pub trait TestTrait<'a> {

}

#[derive(IntoPyDict)]
pub struct Test1 {
    x: u8,
    y: u8,
}

#[derive(IntoPyDict)]
pub struct Test<T> {
    j: Test1,
    h: u8,
    i: u8,
    x: T
}