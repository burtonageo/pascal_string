# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/) 
and this project adheres to [Semantic Versioning](http://semver.org/).

## Unreleased
### Added
- Implement `std::error::Error` and std::`fmt::Display` for `AsciiError`, `PascalStringAppendError`, and
  `PascalStringCreateError`.
- Add crate-level documentation.

### Changed
- Move the `PascalString::get_unchecked()` method onto `PascalStr`.

## [0.1.0] - 2016-10-12
### Added
- First release.
