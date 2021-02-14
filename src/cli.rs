use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg, ArgMatches};

pub(crate) fn args() -> ArgMatches {
    App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!())
        .arg(Arg::from("<type> 'Version type'").possible_values(&["major", "minor", "patch"]))
        .arg(
            Arg::new("info")
                .short('i')
                .long("info")
                .value_name("string")
                .about("Sets info value")
                .takes_value(true),
        )
        .get_matches()
}
