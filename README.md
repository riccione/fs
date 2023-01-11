# fs
stdout files in a given path

## Build
**stdout all files more than <bytes> bytes in a given <dir_path>**
cargo run -- --path <dir_path> --limit <bytes>

limit is an optional parameter

**stdout all files in a given <dir_path>**
cargo run -- --path <dir_path>

## Usage
`fs --path <dir_path> [--limit <bytes> --format <csv|json>]`

`--path <dir_path>`
`--limit <bytes>` - show files only size above provided
`--format <csv|json>` - output format, default is csv