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

    fn to_number(version_type: String) -> u8 {
        let number: u8 = version_type.parse().unwrap();
        number
    }

    pub fn update(self, kind: VersionType) -> Version {
        match kind {
            VersionType::Major => Version::new(
                (Version::to_number(self.major) + 1).to_string(),
                self.minor,
                self.patch,
            ),
            VersionType::Minor => Version::new(
                self.major,
                (Version::to_number(self.minor) + 1).to_string(),
                self.patch,
            ),
            VersionType::Patch => Version::new(
                self.major,
                self.minor,
                (Version::to_number(self.patch) + 1).to_string(),
            ),
        }
    }

    fn combine(self, ver: Version) -> String {
        self.major + &self.minor + &self.patch
    }
}
