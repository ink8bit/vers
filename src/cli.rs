use std::str::FromStr;

use clap::{crate_authors, crate_description, crate_version, App, Arg, ArgMatches};

pub enum Version {
    Major,
    Minor,
    Patch,
}

impl FromStr for Version {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "major" => Ok(Version::Major),
            "minor" => Ok(Version::Minor),
            "patch" => Ok(Version::Patch),
            _ => Err("no match"),
        }
    }
}

pub(crate) fn args() -> ArgMatches {
    App::new("vers")
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!())
        .arg(Arg::from("<type> 'Version type'").possible_values(&["major", "minor", "patch"]))
        .arg(
            Arg::new("info")
                .short('i')
                .long("info")
                .value_name("string")
                .about("Sets info value.")
                .required(true)
                .takes_value(true),
        )
        .get_matches()
}
