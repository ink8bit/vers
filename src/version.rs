use std::str::FromStr;
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
