# nocommit

A command line utility to search for the word "nocommit" in git-staged files.

## Features

- case-insensitive
- prints match count and location of each match
- option to search full diff with HEAD instead of only staged files
- option to restrict search path

## Planned Features

- option to specify alternative strings to search for
- publishing on both crates.io and npm for easy inclusion as a pre-commit hook

## Usage

Clone this repository and run `cargo build --release`, then run it as `./target/release/nocommit` or copy it to your local bin directory.

```
Usage: nocommit [OPTIONS]

Options:
  -a, --all-files                  Search unstaged files as well
  -p, --search-path <SEARCH_PATH>  Path to restrict search to
  -h, --help                       Print help
```
