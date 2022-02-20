use clap::Arg;

pub(crate) const COMMIT_ARG: &str = "no_commit";

pub(crate) fn commit() -> Arg<'static> {
    Arg::new(COMMIT_ARG)
        .short('n')
        .long("no-commit")
        .help("Don't commit your changes")
}
