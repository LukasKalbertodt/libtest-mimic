# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.6.0] - 2022-11-05
### Changed
- **Breaking**: Updated `clap` to version 4 (thanks @msrd0)
- **Breaking**: Bump MSRV to 1.60 (due to the clap update)

### Removed
- **Breaking**: Remove `FromStr` impls for `args::{ColorSetting, FormatSetting}` (use `clap::ValueEnum` instead).

## [0.5.2] - 2022-08-14
### Added
- Re-add `--nocapture` as a noop argument [#18](https://github.com/LukasKalbertodt/libtest-mimic/pull/18) (thanks @sunshowers)

### Fixed
- Link in documentation

## [0.5.1] - 2022-08-13
### Added
- `Trial::{name, kind, has_ignored_flag, is_test, is_bench}` getters

## [0.5.0] - 2022-08-13

Most parts of this library have been rewritten and the API has changed a lot.
You might be better of just reading the new docs instead of this change log.
I do think the new API is better in many regards.
Apart from an improved API, changes that motivated the rewrite are marked with ⭐.

### Changed
- **Breaking**: bump MSRV to 1.58
- **Breaking**: Rename `Test` to `Trial`
- **Breaking**: Rename `run_tests` to `run`
- ⭐ **Breaking**: Make every `Trial` have a runner function instead of `data` + a
  global runner function. Thus, the third parameter of `run` is no more. I think
  this model is more intuitive.
- **Breaking**: Add `Trial::{test, bench}` constructor functions, use builder
  pattern, and make fields private.
- **Breaking**: rename `Args::num_threads` to `test_threads`
- **Breaking**: make fields of `Conclusion` public and remove getter methods
- **Breaking**: remove `RunnerEvent`. This should not have been public.
- ⭐ Tests are now run in main thread when `--test-threads=1` is specified
- ⭐ Reduce number of indirect dependencies considerably
- Fix `rust-version` field in `Cargo.toml` (thanks @hellow554)
- Fix `--ignored` behavior
- Fix some CLI error messages

### Added
- ⭐Panics in test runners are caught and treated as failure
- ⭐ Lots of integration tests (should make any future development of this library way easier)
- Add `must_use` message for `Conclusion`
- Print total execution time at the end of the run
- Allow benchmarks to run in test mode
- `--include-ignored`

### Removed
- **Breaking**: remove unsupported CLI options. They were ignored anyway, but
  the CLI would accept them.


## [0.4.1] - 2022-06-07

- Add `rust = "1.56"` to `Cargo.toml`, stating the existing MSRV.
- Update `crossbeam-channel` to deduplicate some indirect dependencies.

## [0.4.0] - 2022-05-13
- **Breaking**: Update to Rust 2021, bumping MSRV to 1.56
- Fix `--list --ignored` behavior


## [0.3.0] - 2020-06-28
### Added
- Add support for running tests in parallel #4
- Add `Arguments::from_iter` #5

## [0.2.0] - 2019-10-02
### Changed
- Upgrade dependencies #3
- Flush stdout after printing test name 4a36b3318b69df233b0db7d1af3caf276e6bb070

### Fixed
- Fix overflow bug when calculating number of passed tests 264fe6f8a986ab0c02f4a85e64e42ee17596923c

## 0.1.0 - 2018-07-23
### Added
- Everything.


[Unreleased]: https://github.com/LukasKalbertodt/libtest-mimic/compare/v0.6.0...HEAD
[0.6.0]: https://github.com/LukasKalbertodt/libtest-mimic/compare/v0.5.2...v0.6.0
[0.5.2]: https://github.com/LukasKalbertodt/libtest-mimic/compare/v0.5.1...v0.5.2
[0.5.1]: https://github.com/LukasKalbertodt/libtest-mimic/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/LukasKalbertodt/libtest-mimic/compare/v0.4.1...v0.5.0
[0.4.1]: https://github.com/LukasKalbertodt/libtest-mimic/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/LukasKalbertodt/libtest-mimic/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/LukasKalbertodt/libtest-mimic/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/LukasKalbertodt/libtest-mimic/compare/v0.1.0...v0.2.0
[0.1.1]: https://github.com/LukasKalbertodt/libtest-mimic/compare/v0.1.0...v0.1.1
