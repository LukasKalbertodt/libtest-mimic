extern crate test_cli;
extern crate structopt;

use test_cli::{Arguments, Test, Outcome, run_tests};


fn main() {
    let args = Arguments::from_args();

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
        } else {
            Outcome::Passed
        }
    }).exit();
}
