extern crate test_cli;
extern crate structopt;

use test_cli::{Arguments, Test, Outcome, run_tests};


fn main() {
    let args = Arguments::from_args();

    let tests = vec![
        Test::from_name("toph"),
        Test::from_name("sokka"),
        Test {
            name: "long_computation".into(),
            kind: "".into(),
            is_ignored: true,
            data: (),
        },
        Test {
            name: "lifetime".into(),
            kind: "compile-fail".into(),
            is_ignored: false,
            data: (),
        },
        Test::from_name("katara"),
    ];

    run_tests(&args, tests, |test| {
        if test.name == "sokka" {
            Outcome::Failed
        } else {
            Outcome::Passed
        }
    }).exit();
}
