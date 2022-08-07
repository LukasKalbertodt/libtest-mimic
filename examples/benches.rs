extern crate libtest_mimic;

use libtest_mimic::{Arguments, Test, Measurement};


fn main() {
    let args = Arguments::from_args();

    let tests = vec![
        Test::bench("foo::bar", |_| Ok(Some(Measurement { avg: 1274, variance: 23 }))),
        Test::bench("ferris::run", |_| Ok(Some(Measurement { avg: 19082, variance: 99 }))),
        Test::bench("zhu_li::do_the_thing", |_| Ok(Some(Measurement { avg: 73, variance: 6 })))
            .with_ignored_flag(true),
    ];

    libtest_mimic::run(&args, tests).exit();
}
