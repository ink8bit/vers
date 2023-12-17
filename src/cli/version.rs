use clap::{Arg, ArgAction};

pub(crate) const VERSION_ARG: &str = "version_type";

/// Sets version type to update: major, minor, or patch.
pub(crate) fn version() -> Arg {
    Arg::new(VERSION_ARG)
        .required(true)
        .value_parser(["major", "minor", "patch"])
        .action(ArgAction::Set)
}
