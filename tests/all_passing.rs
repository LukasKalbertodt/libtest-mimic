use common::{args, check};
use libtest_mimic::{Test, Conclusion};

#[macro_use]
mod common;


fn tests() -> Vec<Test> {
    vec![
        Test::test("foo", || Ok(())),
        Test::test("bar", || Ok(())),
        Test::test("baz", || Ok(())),
    ]
}

#[test]
fn normal() {
    check(args([]), tests, 3,
        Conclusion {
            num_filtered_out: 0,
            num_passed: 3,
            num_failed: 0,
            num_ignored: 0,
            num_measured: 0,
        },
        "
            test foo ... ok
            test bar ... ok
            test baz ... ok
        "
    );
}

#[test]
fn filter_one() {
    check(args(["foo"]), tests, 1,
        Conclusion {
            num_filtered_out: 2,
            num_passed: 1,
            num_failed: 0,
            num_ignored: 0,
            num_measured: 0,
        },
        "test foo ... ok",
    );
}

#[test]
fn filter_two() {
    check(args(["ba"]), tests, 2,
        Conclusion {
            num_filtered_out: 1,
            num_passed: 2,
            num_failed: 0,
            num_ignored: 0,
            num_measured: 0,
        },
        "
            test bar ... ok
            test baz ... ok
        ",
    );
}
