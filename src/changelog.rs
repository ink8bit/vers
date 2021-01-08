use std::error::Error;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

use chrono::prelude::*;

const NAME: &str = "CHANGELOG.md";

pub struct Changelog<'a> {
    pub entry: Entry<'a>,
}

pub struct Entry<'a> {
    pub version: &'a str,
    pub changes: String,
    pub releaser: String,
}

impl Changelog<'_> {
    pub fn update(&self) -> Result<String, Box<dyn Error>> {
        let formatted = self.format(&self.entry);
        let _res = self.write(formatted)?;

        Ok(format!("{} updated", NAME))
    }

    fn format(&self, e: &Entry) -> String {
        let dt = Local::now();
        let date = dt.to_rfc2822();
        format!(
            "
## {version}

- **Date:** `{date}`
- **Releaser:** @{releaser}
- **Changes:** {changes}
",
            version = e.version,
            changes = e.changes,
            date = date,
            releaser = e.releaser,
        )
    }

    fn write(&self, data: String) -> Result<(), Box<dyn Error>> {
        let mut file = OpenOptions::new().create(true).write(true).open(NAME)?;
        let current = fs::read_to_string(NAME)?;
        let contents = format!("{}\n{}", data, current);
        file.write_all(contents.trim().as_bytes())?;

        Ok(())
    }
}
