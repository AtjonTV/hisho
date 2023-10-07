# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [unreleased]

<!--
### Added
### Changed
### Deprecated
### Removed
### Fixed
### Security
-->

## [0.2.2] - 2023-10-07

### Fixed
- Cyclic dependencies in environments
- Cyclic dependencies in build steps

## [0.2.1] - 2023-10-06

### Changed
- Print message when reading or parsing env file failed

### Fixed
- `Service: Could not find environment:\n` is not printed when environment is empty

## [0.2.0] - 2023-10-06

### Changed
* **BREAKING**: Templates are now scoped, with `arg.` for command line arguments and `env.` for defined environment variables.

## [0.1.2] - 2023-10-06

### Added
* `sources` string array on `Environment` to define .env files to be loaded

### Changed
* If no shell is defined, Commands exit after running build steps.

## [0.1.1] - 2023-10-06

### Changed
- Code improvements

## [0.1.0] - 2023-10-04

### Added
- Build Targets that need to succeed
- Containers that need to run
- Commands
