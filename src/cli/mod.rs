use clap::{crate_authors, crate_description, crate_name, crate_version, ArgMatches, Command};

pub mod commit;
pub mod info;
pub mod version;

use commit::commit;
use info::info;
use version::version;

pub(crate) fn args() -> ArgMatches {
    Command::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!())
        .arg(version())
        .arg(info())
        .arg(commit())
        .get_matches()
}
