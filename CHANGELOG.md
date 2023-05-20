# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
## [0.2.1] - 2023-05-20
### Changed
- Updated all dependencies to latest except for Wayland due to drastically different API.
- Slight code changes to follow clippy recommendations.

## [0.2.0] - 2023-05-20
### Added
- Support for changing the preview color.
- Support for edges.

### Changed
- Updated Wayland dependencies to fix coredump.
- Updated compatible dependencies.

## [0.1.4] - 2021-08-30
### Added
- Minimum timeout of 5ms to prevent an infinite loop as it would continuously check for updates.

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
