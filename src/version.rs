use std::{fs, io};

use serde::Deserialize;

#[derive(Deserialize)]
struct Package {
    version: String,
}

#[allow(dead_code)]
pub enum VersionType {
    Major,
    Minor,
    Patch,
}

// Returns patch version
#[allow(dead_code)]
fn get_patch() -> u8 {
    0
}

// Returns minor version
#[allow(dead_code)]
fn get_minor() -> u8 {
    0
}

// Returns major version
#[allow(dead_code)]
fn get_major() -> u8 {
    0
}

// Returns parsed version from package.json
pub fn get_parsed() -> Result<String, io::Error> {
    let f = fs::read_to_string("package.json")?;
    let package: Package = serde_json::from_str(&f)?;

    Ok(package.version)
}

// Updates version in package.json by a given one
#[allow(dead_code)]
fn update(_version: VersionType) {}

#[derive(Debug)]
pub struct Version {
    major: String,
    minor: String,
    patch: String,
}

impl Version {
    pub fn new(major: String, minor: String, patch: String) -> Version {
        Version {
            major,
            minor,
            patch,
        }
    }

    fn to_number(self) -> u8 {
        let mj = self.major.into_bytes();
        mj[0]
    }

    pub fn update(self, kind: VersionType) -> Version {
        match kind {
            VersionType::Major => Version::new(
                (self.major.into_bytes()[0] + 1).to_string(),
                self.minor,
                self.patch,
            ),
            VersionType::Minor => Version::new(
                self.major,
                (self.minor.into_bytes()[0] + 1).to_string(),
                self.patch,
            ),
            VersionType::Patch => Version::new(
                self.major,
                self.minor,
                (self.patch.into_bytes()[0] + 1).to_string(),
            ),
        }
    }

    fn to_string() -> String {
        "some".to_owned()
    }
}
