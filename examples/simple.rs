extern crate libtest_mimic;

use std::{thread, time};
use libtest_mimic::{Arguments, Test, run_tests, Failed};


fn main() {
    let args = Arguments::from_args();

    let tests = vec![
        Test::test("check_toph", check_toph),
        Test::test("check_sokka", check_sokka),
        Test::test("long_computation", long_computation).with_ignored_flag(true),
        Test::test("foo", compile_fail_dummy).with_kind("compile-fail"),
        Test::test("check_katara", check_katara),
    ];

    run_tests(&args, tests).exit();
}


// Tests

fn check_toph() -> Result<(), Failed> {
    Ok(())
}
fn check_katara() -> Result<(), Failed> {
    Ok(())
}
fn check_sokka() -> Result<(), Failed> {
    Err("Sokka tripped and fell :(".into())
}
fn long_computation() -> Result<(), Failed> {
    thread::sleep(time::Duration::from_secs(1));
    Ok(())
}
fn compile_fail_dummy() -> Result<(), Failed> {
    Ok(())
}
