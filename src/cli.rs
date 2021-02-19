use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg, ArgMatches};

pub(crate) fn args() -> ArgMatches {
    App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!())
        .arg(
            Arg::new("version_type")
                .index(1)
                .possible_values(&["major", "minor", "patch"])
                .required(true),
        )
        .arg(
            Arg::new("info")
                .short('i')
                .long("info")
                .value_name("string")
                .about("Sets info value")
                .takes_value(true),
        )
        .arg(
            Arg::new("no_commit")
                .short('n')
                .long("no-commit")
                .about("Prevents committing your changes"),
        )
        .get_matches()
}
