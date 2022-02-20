use clap::{crate_authors, crate_description, crate_name, crate_version, Arg, ArgMatches, Command};

pub(crate) fn args() -> ArgMatches {
    Command::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!())
        .arg(
            Arg::new("version_type")
                .possible_values(&["major", "minor", "patch"])
                .required(true),
        )
        .arg(
            Arg::new("info")
                .short('i')
                .long("info")
                .value_name("string")
                .help("Set info value")
                .takes_value(true),
        )
        .arg(
            Arg::new("no_commit")
                .short('n')
                .long("no-commit")
                .help("Don't commit your changes"),
        )
        .get_matches()
}
