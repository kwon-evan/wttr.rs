# wttr.rs

**WIP**
A Rust Wrapper for wttr.in for the command line.
Currently only supports one line format only.
If you find the project useful, please consider giving it a star!

```bash
$ wttr
Nam-gu, South Korea: ⛅️  -9°C
```

## Install

```bash
cargo build --release
cargo install --path .
```

## Usage

```
➜ wttr --help
Usage: wttr [OPTIONS]

Options:
  -l, --location <LOCATION>
  -f, --format <FORMAT>      [default: 3]
  -h, --help                 Print help
  -V, --version              Print version
```

## License

WTFPL
