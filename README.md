# Gear Adder

An example use case of `dbg!` macro provided by the [gdbg](https://docs.rs/gdbg/) crate

```
$ cargo test
   Compiling gear-adder v0.1.0 (/home/btwiuse/gear-adder)
    Finished test [unoptimized + debuginfo] target(s) in 3.01s
     Running unittests src/lib.rs (target/debug/deps/gear_adder-6ca38b33ea5bcddc)

running 2 tests
test tests::it_works ... ok
[DEBUG it_works] init() is being called with bytes: [105, 110, 105, 116]
[DEBUG it_works] [/home/btwiuse/gear-adder/src/lib.rs:34] (left, right) = (
    1,
    1,
)
[DEBUG it_works] [/home/btwiuse/gear-adder/src/lib.rs:35] add(left, right) = 2
[DEBUG it_works] [/home/btwiuse/gear-adder/src/lib.rs:34] (left, right) = (
    2,
    2,
)
[DEBUG it_works] [/home/btwiuse/gear-adder/src/lib.rs:35] add(left, right) = 4
[DEBUG it_works] [/home/btwiuse/gear-adder/src/lib.rs:34] (left, right) = (
    7,
    9,
)
[DEBUG it_works] [/home/btwiuse/gear-adder/src/lib.rs:35] add(left, right) = 16
test it_works ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.20s

   Doc-tests gear-adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
