use std::str::FromStr;

use clap::{App, Arg, ArgMatches};

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

fn is_valid_releaser_name(val: &str) -> Result<(), String> {
    if val.contains('@') {
        return Err("you should write releaser nickname without `@` sign.".to_string());
    }
    Ok(())
}

pub fn args() -> ArgMatches {
    App::new("vers")
        .version("0.1.0")
        .about("vers is a CLI app which changes package version in JavaScript projects.")
        .author("ink8bit")
        .arg(Arg::from("<type> 'Version type'").possible_values(&["major", "minor", "patch"]))
        .arg(
            Arg::new("releaser")
                .short('r')
                .long("releaser")
                .value_name("string")
                .about("Sets releaser value.")
                .required(true)
                .takes_value(true)
                .validator(is_valid_releaser_name),
        )
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
