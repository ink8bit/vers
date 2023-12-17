use clap::{Arg, ArgAction};

pub(crate) const COMMIT_ARG: &str = "no_commit";

/// Prevents creating a commit while updating version.
pub(crate) fn commit() -> Arg {
    Arg::new(COMMIT_ARG)
        .short('n')
        .long("no-commit")
        .help("Don't commit your changes")
        .action(ArgAction::SetTrue)
}
