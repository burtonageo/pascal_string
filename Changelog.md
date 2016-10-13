# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/) 
and this project adheres to [Semantic Versioning](http://semver.org/).

## Unreleased
### Added
- Implement `std::str::FromStr` for `PascalString`.

### Fixed
- Improve wording of crate-level doc comment.
- Improve doc comment for `IntoChars` struct.

## [0.2.1] - 2016-10-13
### Fixed
- Fixed `PascalString::remove` to avoid panicking with correct input.
- `IntoChars` now iterates in the same order as `Chars` and `CharsMut` (from the start of the string).

## [0.2.0] - 2016-10-13
### Added
- Implement `std::error::Error` and std::`fmt::Display` for `AsciiError`, `PascalStringAppendError`, and
  `PascalStringCreateError`.
- Add crate-level documentation.

### Changed
- Move the `PascalString::get_unchecked()` method onto `PascalStr`.

## [0.1.0] - 2016-10-12
### Added
- First release.
