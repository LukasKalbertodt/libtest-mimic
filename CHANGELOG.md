# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

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


[Unreleased]: https://github.com/LukasKalbertodt/stable-vec/compare/v0.4.0...HEAD
[0.4.0]: https://github.com/LukasKalbertodt/stable-vec/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/LukasKalbertodt/stable-vec/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/LukasKalbertodt/stable-vec/compare/v0.1.0...v0.2.0
[0.1.1]: https://github.com/LukasKalbertodt/stable-vec/compare/v0.1.0...v0.1.1
