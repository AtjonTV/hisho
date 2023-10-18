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

## [0.5.0] - 2023-10-18

### Added
* Git commit_sha, commit_sha_short, commit_date (in ISO8601 format), commit_author_name and commit_author_email available in templates via `git`

### Changed
* Container names can now be templated based on the command environment
* Hisho exists when a container does not exist or can not be started
* **BREAKING**: BuildStep now takes a list of Processes for its `shell` field
* Resolve Git repository relative to the project file, even when `--hisho:file=` is given
* Resolve `~` in a given `--hisho:file=` path to the users home directory
* Resolve environment sources relative to project file

### Fixed
* BuildStep commands are not passed through the template engine twice

## [0.4.0] - 2023-10-07

### Changed
* **BREAKING**: Renamed service_helper to Hisho
* **BREAKING**: Renamed default file from `service.ron` to `hisho.ron`
* **BREAKING**: Renamed `Service` to `Project` in configuration files.

### Fixed
* when a container had an empty name, all the existing and stopped containers where started


## [0.3.0] - 2023-10-07

### Added
* `input_files` vector on BuildStep to get a list of files by glob pattern as `{{build.input_files}}`
* `name` of BuildStep as `{{build.name}}`
* `--service:file` argument can be used to specify the service ron file to load, defaults to `service.ron`


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
