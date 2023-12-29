use libtest_mimic::{Arguments, Trial};

#[test]
fn check_test_on_main_thread() {
    let outer_thread = std::thread::current().id();

    #[allow(unused_mut)]
    let mut args = Arguments::default();

    #[cfg(feature = "multithreaded")]
    {
        args.test_threads = Some(1);
    }
    let conclusion = libtest_mimic::run(
        &args,
        vec![Trial::test("check", move || {
            assert_eq!(outer_thread, std::thread::current().id());
            Ok(())
        })],
    );

    assert_eq!(conclusion.num_passed, 1);
}
