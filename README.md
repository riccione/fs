![release](https://github.com/github/docs/actions/workflows/main.yml/badge.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

<!-- ABOUT THE PROJECT -->
## About fs

CLI app that stdout list of files from a given path, for each file it counts hash

### Installation

Does not required, portable CLI app

## Build from source

`cargo build --release --locked`

## Usage
`fs --path <dir_path> [--limit <bytes> --format <csv|json>]`

- `--path <dir_path>`
- `--limit <bytes>` - show files only size above provided
- `--format <csv|json>` - output format, default is csv

<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE.txt` for more information.