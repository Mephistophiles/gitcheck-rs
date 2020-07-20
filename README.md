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

# TODO

 - [X] Quiet mode (`--quiet`)
 - [ ] Verbose output (`--verbose`)
 - [ ] Fetch repositories (`--remote`)
 - [ ] Check untracked files (`--untracked`)
 - [X] All branch mode (`--all-branch`)
 - [ ] Support ignore list (`--localignore`)
 - [ ] Terminal bell on repository changes (`--bell`)
 - [ ] Watch mode (`--watch`)
 - [ ] Send email on repository changes (`--email`, `--init-email`)

# Options

```
gitcheck-rs 0.0.1
Maxim Zhukov
rust gitcheck. Check multiple git repository in one pass

USAGE:
    gitcheck [FLAGS] [OPTIONS]

FLAGS:
    -a, --all-branch    Show the status of all branches
        --debug         Show debug message
    -h, --help          Prints help information
    -q, --quiet         Display info only when repository needs action
    -V, --version       Prints version information

OPTIONS:
    -d, --dir <dir>              Search <dir> for repositories (can be used multiple times)
    -i, --ignore-branch <re>     ignore branches matching the regex <re>
    -m, --maxdepth <maxdepth>    Limit the depth of repositories search
```
