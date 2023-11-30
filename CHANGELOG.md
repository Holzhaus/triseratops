# Changelog

All notable changes to this project will be documented in this file.

## [0.0.3] - 2023-11-30

### Bug Fixes

- Also derive `Eq` for `Color` struct
- Remove needless borrow on `File::create()` argument
- Use variables directly in `format!` string
- Fix remaining clippy warnings
- Fix some clippy warnings

### Documentation

- Fix some phrasing issues in README.md

### Miscellaneous Tasks

- Update pre-commit hooks
- Fix false-positive in codespell hook
- Add markdownlint-cli2 hook
- Add prettier hook for YAML auto-formatting
- Add sourceheaders hook for automatic license headers
- Add `gitlint` hook
- Update pre-commit hooks
- Update dependencies to latest versions

### Styling

- Apply automatic changes from rustfmt
- Reformat YAML files in a consistent style
- Reformat Markdown files to fix various formatting issues
- Add source code headers to all source files

## [v0.0.2] - 2022-02-19

- Add initial support for reading the Serato Library #7
- Split MP4_ATOM string into mean and name parts #8
- Add methods to write tags to tag container #9
- Add native GEOB tag support to reader example #10
- Various code cleanups and improvements #13 #14 #15 #16

## [v0.0.1] - 2021-02-04

Initial release.
