A proc macro and a macro attribute to quickly time funcitions. Uses `std::time::Instant` so relies on std to work. 

### \#\[time_this\]

This macro can be used to time any function you want using `std::time::Instant`. It may not work
correctly with `async fn` and it definitely doesn't work with `const fn` (even if called in a non-const
context. You can write a small wrapping fn if you need to time a `const fn`).
It will print:
* the time in ns if the function took less than 1μs.
* the time in μs if the function took less than 1ms.
* the time in ms if the function took longer than 1ms, but less than 1s.
* the time in s if the function took more than a second, with two decimal digits.

```rust
# use crate::time_this::time_this;

#[time_this]
fn add(a: u32, b: u32) -> u32 {
    a + b
}

fn main() {
    let result = add(3, 5);
    // function 'add()' took 37ns
}
```

### time!()

This macro can be used to time any expression you want using `std::time::Instant`. It returns the
result of the expression, similar to `dbg!()`. It may not work correctly with `async fn`.
Instead of printing the function name, it will print the file/line the expression that was timed at.

```rust
# use crate::time_this::time;

fn add(a: u32, b: u32) -> u32 {
    a + b
}

fn main() {
    let result = time!(add(3, 5));
    // expression at [tests/tests.rs:33] took 28ns
}
```