# Changelog

This file is a copy of the CHANGELOG.md file from the Git repository of Hisho.

## [1.0.0] - 2023-11-01

Release v1.0

## [1.0.0-rc.5] - 2023-10-30

* **Added**
  * Core: `build_tool::ensure_steps_are_build` for building a custom set of steps
  * Cli2: `build` (`b`, `make`) subcommand for building a build step

* **Removed**
  * Cli: `hisho_cli` has been removed.

## [1.0.0-rc.4] - 2023-10-29

* **Added**
  * Core: `git::fetch_repo_vars` now includes `branch`, `commit_commiter_name` and `commit_committer_email`.
  * Core: `arg_parse::parse` a simple argument parser for flags and options
  * Cli2: Start implementation of a new Clap based command line interface
  * Cli2: Support option parsing for `arg.` templates
  * Cli2: Print project commands in help output
  * Cli2: `run` now as the aliases `r` and `cmd`

* **Deprecated**
  * Cli: hisho_cli is now deprecated, please use hisho_cli2 instead

## [1.0.0-rc.3] - 2023-10-28

* **Added**
  * Process now has `cwd` to specify the current working directory where the command is executed

* **Changed**
  * Split Hisho into `hisho_core` and `hisho_cli` crates
  * Core: Moved `impl` blocks from `config` module to `config_models` module
  * Core: Renamed `config` module to `environment` module
  * Core: Renamed `build` module to `build_tool` module
  * Core: Make `resolve_files_from_globs` in `build_tool` module public
  * Core: Make `get_home_dir` in `files` module public

* **Removed**
  * Core: Make `render_environment_value` in `template` module private

## [1.0.0-rc.2] - 2023-10-20

* **Changed**
  * **BREAKING**: `capture_all` as removed from Command and `[[argv]]` was added for Process argument templates

## [1.0.0-rc.1] - 2023-10-18

Tag v0.5.0 as v1.0.0-rc.1 in preparation for a stable release.  
There might still be breaking changes before 1.0 stable!

## [0.5.0] - 2023-10-18

* **Added**
  * Git commit_sha, commit_sha_short, commit_date (in ISO8601 format), commit_author_name and commit_author_email available in templates via `git`

* **Changed**
  * Container names can now be templated based on the command environment
  * Hisho exists when a container does not exist or can not be started
  * **BREAKING**: BuildStep now takes a list of Processes for its `shell` field
  * Resolve Git repository relative to the project file, even when `--hisho:file=` is given
  * Resolve `~` in a given `--hisho:file=` path to the users home directory
  * Resolve environment sources relative to project file

* **Fixed**
  * BuildStep commands are not passed through the template engine twice

## [0.4.0] - 2023-10-07

* **Changed**
  * **BREAKING**: Renamed service_helper to Hisho
  * **BREAKING**: Renamed default file from `service.ron` to `hisho.ron`
  * **BREAKING**: Renamed `Service` to `Project` in configuration files.

* **Fixed**
  * when a container had an empty name, all the existing and stopped containers where started

## [0.3.0] - 2023-10-07

* **Added**
  * `input_files` vector on BuildStep to get a list of files by glob pattern as `{{build.input_files}}`
  * `name` of BuildStep as `{{build.name}}`
  * `--service:file` argument can be used to specify the service ron file to load, defaults to `service.ron`


## [0.2.2] - 2023-10-07

* **Fixed**
  - Cyclic dependencies in environments
  - Cyclic dependencies in build steps

## [0.2.1] - 2023-10-06

* **Changed**
  - Print message when reading or parsing env file failed

* **Fixed**
  - `Service: Could not find environment:\n` is not printed when environment is empty

## [0.2.0] - 2023-10-06

* **Changed**
  * **BREAKING**: Templates are now scoped, with `arg.` for command line arguments and `env.` for defined environment variables.

## [0.1.2] - 2023-10-06

* **Added**
  * `sources` string array on `Environment` to define .env files to be loaded

* **Changed**
  * If no shell is defined, Commands exit after running build steps.

## [0.1.1] - 2023-10-06

* **Changed**
  - Code improvements

## [0.1.0] - 2023-10-04

* **Added**
  - Build Targets that need to succeed
  - Containers that need to run
  - Commands
