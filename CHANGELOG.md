# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Changed
- Added support for changing the preview color.
- Added support for edges.

### Changed
- Update Wayland dependencies to fix coredump.
- Updated compatible dependencies.

## [0.1.4] - 2021-08-30
### Changed
- Added a minimum timeout of 5ms to prevent an infinite loop as it would continuously check for updates.

## [0.1.3] - 2021-05-29
### Changed
- Use the top layer instead of the overlay layer. This makes it not lay on top of lock screens like swaylock.
- When coming out of the lock screen, it will ignore the first events if they are too fast as they would indicate the move was not moved while in the lock screen. This would otherwise lead to invalidly running the command again.

## [0.1.2] - 2021-05-29
### Added
- Automatic AUR deployments.

## [0.1.1] - 2021-05-29
### Changed
- Locked down dependencies in order to be reproducible.

## [0.1.0] - 2021-05-23
### Added
- Initial release.
