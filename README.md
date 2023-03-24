A proc macro and a macro attribute to quickly time functions. Uses `std::time::Instant` so relies on std to work. 

### \#\[time_this\]

This macro can be used to time any function you want using `std::time::Instant`. Whenever the function
gets called, its timing information will be passed to stdout (though in the case of recursive calls,
it will only be printed once). It may not work correctly with `async fn`, in particular when a future
is returned but not yet awaited, and it definitely doesn't work with `const fn`, even if called in a
non-const context. If needed, you can write a small wrapping function if you need to time a `const fn`.
It will print:
* the time in ns if the function took less than 1μs.
* the time in μs if the function took less than 1ms.
* the time in ms if the function took longer than 1ms, but less than 1s.
* the time in s if the function took more than a second, with two decimal digits.

```rust
use time_this::time_this;

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

This macro can be used to time any expression you want using `std::time::Instant`. After the expression
evaluates, timing information will immediately be passed to stdout and the result will be returned,
similar to similar to `dbg!()`. Similar to `time_this`, it may not work correctly with `async fn`.
Instead of printing the function name, it will print file/line the expression that was timed at, as well
as the expression itself.

```rust
use time_this::time;

fn add(a: u32, b: u32) -> u32 {
    a + b
}

fn main() {
    let result = time!(add(3, 5));
    // [tests/tests.rs:33] -> add(3, 5) took 28ns
}
```
