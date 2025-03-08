# catr
cat command for rust

## Install

```shell
cargo install --path . --force
```

## Uninstall

```shell
cargo uninstall echor
```

## Usage
Usage: catr [OPTIONS] [FILE]...

### Arguments:
[FILE]...  Input file(s) [default: -]

### Options:
- -n, --number           Number of lines to print
- -b, --number-nonblock  Number non-block lines
- -h, --help             Print help
- -V, --version          Print version

## test
create output files
```shell
./mk-outs.sh
```

run tests
```shell
cargo test
```