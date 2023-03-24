#![feature(stmt_expr_attributes)]
#![feature(proc_macro_hygiene)]

use time_this::{time, time_this};

#[time_this]
pub fn try_compile() {}

#[time_this]
fn add_for_compile(a: usize, b: usize) -> usize {
    a + b
}

#[time_this]
#[inline]
fn recursive(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => recursive(n - 1) + recursive(n - 2),
    }
}

fn add_for_time(a: usize, b: usize) -> usize {
    a + b
}

#[test]
fn test() {
    dbg!(recursive(5));

    assert_eq!(time!(add_for_time(3, 5)), add_for_time(3, 5));
    assert_eq!(add_for_time(3, 5), 8);
}
