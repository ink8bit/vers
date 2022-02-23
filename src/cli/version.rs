use clap::Arg;

pub(crate) const VERSION_ARG: &str = "version_type";

/// Sets version type to update: major, minor, or patch.
pub(crate) fn version() -> Arg<'static> {
    Arg::new(VERSION_ARG)
        .possible_values(&["major", "minor", "patch"])
        .required(true)
}
