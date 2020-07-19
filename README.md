# gitcheck-rs

![Rust](https://github.com/Mephistophiles/gitcheck-rs/workflows/Rust/badge.svg)

This is a Rust replacement for gitcheck written by Python (https://github.com/badele/gitcheck)

# Installation

```
cargo install --git https://github.com/Mephistophiles/gitcheck-rs
```

# Repository status

This repository is currently in WIP (Work-In-Progress) state. At this moment released:
* Basic functionality
* Setup maximum depth
* Select directory `--dir`
* Select ignored git branches `--ignore-branch`

# Options

```
gitcheck-rs 0.0.1
Maxim Zhukov
rust gitcheck. Check multiple git repository in one pass

USAGE:
    gitcheck [FLAGS] [OPTIONS]

FLAGS:
        --debug      Show debug message
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --dir <dir>              Search <dir> for repositories (can be used multiple times)
    -i, --ignore-branch <re>     ignore branches matching the regex <re>
    -m, --maxdepth <maxdepth>    Limit the depth of repositories search
```
