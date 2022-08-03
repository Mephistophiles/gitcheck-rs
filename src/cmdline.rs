/*
 *  Copyright (C) 2020 Maxim Zhukov <mussitantesmortem@gmail.com>
 *
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
use clap::Parser;
use regex::Regex;
use std::path::PathBuf;

#[derive(Clone, Parser, Debug)]
#[clap(about, version, author)]
pub(crate) struct Options {
    /// Show the status of all branches
    #[clap(short, long)]
    pub(crate) all_branch: bool,

    /// Show debug message
    #[clap(long)]
    pub(crate) debug: bool,

    /// Display info only when repository needs action
    #[clap(short, long)]
    pub(crate) quiet: bool,

    /// Force remote update (slow)
    #[clap(short, long)]
    pub(crate) remote: bool,

    /// Show untracked files
    #[clap(short, long)]
    pub(crate) untracked: bool,

    /// ignore branches matching the regex <re>
    #[clap(short, long)]
    pub(crate) ignore_branch_regex: Option<Regex>,

    /// Search <dir> for repositories (can be used multiple times)
    #[clap(short = 'd', long = "dir")]
    pub(crate) working_directories: Vec<PathBuf>,

    /// Limit the depth of repositories search
    #[clap(short, long)]
    pub(crate) max_depth: Option<usize>,

    /// Specifies  the  number  of jobs (commands) to run simultaneously. (Default $(nprocs))
    #[clap(short, long, default_value_t = num_cpus::get())]
    pub(crate) jobs: usize,
}

pub(crate) fn parse_args() -> Options {
    let mut options = Options::parse();

    if options.working_directories.is_empty() {
        options
            .working_directories
            .push(std::env::current_dir().unwrap());
    }

    options
}
