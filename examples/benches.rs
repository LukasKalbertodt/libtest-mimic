extern crate libtest_mimic;

use libtest_mimic::{Arguments, Test, Outcome, run_tests};


fn main() {
    let args = Arguments::from_args();

    let tests = vec![
        Test {
            name: "foo::bar".into(),
            kind: "".into(),
            is_ignored: false,
            is_bench: true,
            data: (1274, 23),
        },
        Test {
            name: "zhu_li::do_the_thing".into(),
            kind: "".into(),
            is_ignored: false,
            is_bench: true,
            data: (73, 6),
        },
        Test {
            name: "ferris::run".into(),
            kind: "".into(),
            is_ignored: false,
            is_bench: true,
            data: (19082, 99),
        },
    ];

    run_tests(&args, tests, |test| {
        let (avg, variance) = test.data;
        Outcome::Measured { avg, variance }
    }).exit();
}
