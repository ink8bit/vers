use clap::{Arg, ArgAction};

pub(crate) const INFO_ARG: &str = "info";

/// Adds content to `Changes` section in every changelog entry.
pub(crate) fn info() -> Arg {
    Arg::new(INFO_ARG)
        .short('i')
        .long("info")
        .value_name("string")
        .help("Set info value")
        .num_args(1)
        .action(ArgAction::Set)
}
