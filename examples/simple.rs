extern crate libtest_mimic;

use std::{thread, time};
use libtest_mimic::{Arguments, Test, Outcome, run_tests};


fn main() {
    let args = Arguments::from_args();

    let tests = vec![
        Test::test("toph"),
        Test::test("sokka"),
        Test::test("long_computation"),
        Test {
            name: "longer_computation".into(),
            kind: "".into(),
            is_ignored: true,
            is_bench: false,
            data: (),
        },
        Test {
            name: "lifetime".into(),
            kind: "compile-fail".into(),
            is_ignored: false,
            is_bench: false,
            data: (),
        },
        Test::test("katara"),
    ];

    run_tests(&args, tests, |test| {
        if test.name == "sokka" {
            Outcome::Failed { msg: Some("Sokka tripped and fell :(".into()) }
        } else if test.name == "long_computation" {
            thread::sleep(time::Duration::from_secs(1));
            Outcome::Passed
        } else {
            Outcome::Passed
        }
    }).exit();
}
