use clap::{App, AppSettings, Arg};
use regex::Regex;
use std::env;
use std::path::{Path, PathBuf};

#[derive(Clone)]
pub(crate) struct Options {
    pub(crate) all_branch: bool,
    pub(crate) debug: bool,
    pub(crate) quiet: bool,
    pub(crate) ignore_branch_regex: Option<Regex>,
    pub(crate) working_directory: PathBuf,

    pub(crate) max_depth: usize,
    pub(crate) jobs: usize,
}

pub(crate) fn parse_args() -> Options {
    let matches = App::new("gitcheck-rs")
        .version("0.0.1")
        .author("Maxim Zhukov")
        .setting(AppSettings::ColoredHelp)
        .about("rust gitcheck. Check multiple git repository in one pass")
        // .arg(
        //     Arg::with_name("verbose")
        //         .short('v')
        //         .long("verbose")
        //         .about("Show files & commits"),
        // )
        .arg(
            Arg::with_name("debug")
                .long("debug")
                .about("Show debug message"),
        )
        // .arg(
        //     Arg::with_name("remote")
        //         .short('r')
        //         .long("remote")
        //         .about("force remote update (slow)"),
        // )
        // .arg(
        //     Arg::with_name("untracked")
        //         .short('u')
        //         .long("untracked")
        //         .about("Show untracked files"),
        // )
        // .arg(
        //     Arg::with_name("bell")
        //         .short('b')
        //         .long("bell")
        //         .about("bell on action needed"),
        // )
        // .arg(
        //     Arg::with_name("watch")
        //         .short('w')
        //         .long("watch")
        //         .value_name("sec")
        //         .takes_value(true)
        //         .about("after displaying, wait <sec> and run again"),
        // )
        .arg(
            Arg::with_name("ignore-branch")
                .short('i')
                .long("ignore-branch")
                .value_name("re")
                .takes_value(true)
                .about("ignore branches matching the regex <re>"),
        )
        .arg(
            Arg::with_name("dir")
                .short('d')
                .long("dir")
                .value_name("dir")
                .takes_value(true)
                .about("Search <dir> for repositories (can be used multiple times)"),
        )
        .arg(
            Arg::with_name("maxdepth")
                .short('m')
                .long("maxdepth")
                .value_name("maxdepth")
                .takes_value(true)
                .about("Limit the depth of repositories search"),
        )
        .arg(
            Arg::with_name("quiet")
                .short('q')
                .long("quiet")
                .about("Display info only when repository needs action"),
        )
        // .arg(
        //     Arg::with_name("email")
        //         .short('e')
        //         .long("email")
        //         .about("Send an email with result as html, using mail.properties parameters"),
        // )
        .arg(
            Arg::with_name("all-branch")
                .short('a')
                .long("all-branch")
                .about("Show the status of all branches"),
        )
        // .arg(
        //     Arg::with_name("localignore")
        //         .short('l')
        //         .long("localignore")
        //         .value_name("re")
        //         .takes_value(true)
        //         .about("ignore changes in local files which match the regex <re>"),
        // )
        // .arg(
        //     Arg::with_name("init-email").long("init-email").about(
        //         "Initialize mail.properties file (has to be modified by user using JSON Format",
        //     ),
        // )
        .arg(
            Arg::with_name("jobs")
                .short('j')
                .long("jobs")
                .value_name("jobs")
                .takes_value(true)
                .about("Specifies  the  number  of jobs (commands) to run simultaneously. (Default $(nprocs))"),
        )
        .get_matches();

    Options {
        all_branch: matches.is_present("all-branch"),
        debug: matches.is_present("debug"),
        quiet: matches.is_present("quiet"),

        ignore_branch_regex: matches
            .value_of("ignore-branch")
            .and_then(|v| Regex::new(v).ok()),

        working_directory: matches
            .value_of("dir")
            .and_then(|d| Path::new(d).canonicalize().ok())
            .unwrap_or_else(|| env::current_dir().unwrap()),
        max_depth: matches
            .value_of("maxdepth")
            .and_then(|m| m.parse().ok())
            .unwrap_or(usize::MAX),
        jobs: matches
            .value_of("jobs")
            .and_then(|j| j.parse().ok())
            .unwrap_or_else(num_cpus::get),
    }
}
