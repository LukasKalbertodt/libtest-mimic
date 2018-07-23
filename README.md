libtest-mimic
=============

Write your own test harness that looks and behaves like the built-in test harness (used by `rustc --test`)!

This is a simple and small testing framework that mimics the original `libtest` (used by `rustc --test`). That means: all output looks pretty much like `cargo test` and most CLI arguments are understood and used. With that plumbing work out of the way, your test runner can concentrate on the actual testing.

**See it in action** (with the `tidy` example):

[![asciicast](https://asciinema.org/a/ZBQ5vkwW5VaQCn7VGohuNFxr2.png)](https://asciinema.org/a/ZBQ5vkwW5VaQCn7VGohuNFxr2)


# Example

```
extern crate libtest_mimic;

use libtest_mimic::{Arguments, Test, Outcome, run_tests};


// Parse command line arguments
let args = Arguments::from_args();

// Create a list of tests (in this case: three dummy tests)
let tests = vec![
    Test::test("toph"),
    Test::test("sokka"),
    Test {
        name: "long_computation".into(),
        kind: "".into(),
        is_ignored: true,
        is_bench: false,
        data: (),
    },
];

// Run all tests and exit the application appropriatly (in this case, the
// test runner is a dummy runner which does nothing and says that all s
// passed).
run_tests(&args, tests, |test| Outcome::Passed).exit();
```

For more examples, see [`examples/`](https://github.com/LukasKalbertodt/libtest-mimic/tree/master/examples).

---

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
