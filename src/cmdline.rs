use clap::{App, Arg, ArgMatches};

pub(crate) fn parse_args() -> ArgMatches {
    App::new("gitcheck-rs")
        .version("0.0.1")
        .author("Maxim Zhukov")
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
        // .arg( // TODO
        //     Arg::with_name("quiet")
        //         .short('q')
        //         .long("quiet")
        //         .about("Display info only when repository needs action"),
        // )
        // .arg(
        //     Arg::with_name("email")
        //         .short('e')
        //         .long("email")
        //         .about("Send an email with result as html, using mail.properties parameters"),
        // )
        // .arg(
        //     Arg::with_name("all-branch")
        //         .short('a')
        //         .long("all-branch")
        //         .about("Show the status of all branches"),
        // )
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
        .get_matches()
}
