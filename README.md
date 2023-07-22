# nocommit

A commandline utility to search for the word "nocommit" in git-staged files.

## Features

- case-insensitive

## Planned Features

- option to specify alternative strings to search for
- option to search full diff with HEAD instead of only staged files
- option to restrict search path
- publishing on both crates.io and npm for easy inclusion as a pre-commit hook

## Usage

Clone this repository and run `cargo build --release`, then run it as `./target/release/nocommit` or copy it to your local bin directory.

