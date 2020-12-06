use std::fs;
use std::fs::OpenOptions;
use std::io::{Error, Write};
use std::path::Path;

use chrono::prelude::*;

const NAME: &str = "CHANGELOG.md";

#[derive(Debug)]
pub struct Changelog {
    date: String,
    version: String,
    info: String,
    releaser: String,
}

impl Changelog {
    pub fn new(version: String, info: String, releaser: String) -> Self {
        let dt = Local::now();
        let date = dt.to_rfc2822();
        Self {
            date,
            version,
            info,
            releaser,
        }
    }

    pub fn create(&self) -> String {
        format!(
            "
## {version}

- **Date:** `{date}`
- **Releaser:** @{releaser}
- **Info:** {info}
",
            version = self.version,
            date = self.date,
            info = self.info,
            releaser = self.releaser,
        )
    }

    pub fn write(self, data: String) -> Result<(), Error> {
        if !Path::new(NAME).exists() {
            println!("Created {}", NAME);
            return fs::write(NAME, data);
        }

        let mut file = OpenOptions::new().append(true).open(NAME)?;
        file.write_all(data.as_bytes())?;
        println!("Successfully added new entry to {}", NAME);

        Ok(())
    }
}
