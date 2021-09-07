# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [1.1.1] - 2021-09-07

### Added

- Add custom code for UTF-8 en/decode error
- Add Bash error codes
- Fix typo

## [1.1.0] - 2021-04-28

### Added

- `PosixError` implement `From<std::io:Error>`
- `PosixError` implement `From<std::process::Output>`

### Changed

- Deprecate `error_from_output`
- Deprecate `to_posix_error`

### Fixed

- docs: Fix `clippy::doc-markdown`
