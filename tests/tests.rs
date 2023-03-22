#![feature(stmt_expr_attributes)]
#![feature(proc_macro_hygiene)]

use time_this::{time_this, time};

#[time_this]
pub fn try_compile() {}

#[time_this]
fn compile_with_params(a: usize, b: usize) -> usize {
    a + b
}

#[time_this]
#[inline]
fn recursive(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => recursive(n-1) + recursive(n-2),
    }
}

fn for_timing(a: usize, b: usize) -> usize {
    a + b
}

#[test]
fn test() {
    recursive(5);

    assert_eq!(time!(for_timing(3, 5)), for_timing(3, 5));
    assert_eq!(for_timing(3, 5), 8);
}
