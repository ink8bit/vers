use std::error::Error;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

use chrono::prelude::*;

const CHANGELOG_FILE_NAME: &str = "CHANGELOG.md";

pub(crate) struct Changelog<'a> {
    pub entry: Entry<'a>,
}

pub(crate) struct Entry<'a> {
    pub version: &'a str,
    pub changes: &'a str,
    pub releaser: &'a str,
    pub commits: &'a str,
}

impl Changelog<'_> {
    pub(crate) fn update(&self) -> Result<String, Box<dyn Error>> {
        let formatted = self.format(&self.entry);
        let _res = self.write(formatted)?;

        Ok(format!("{} updated", CHANGELOG_FILE_NAME))
    }

    fn format(&self, e: &Entry) -> String {
        let dt = Local::now();
        let date = dt.to_rfc2822();
        let mut formatted = format!(
            "
## {version}

### {date}

**Released by:** {releaser}
",
            version = e.version,
            date = date,
            releaser = e.releaser
        );

        if !e.changes.is_empty() {
            formatted.push_str(&format!(
                "
**Changes:** {}
",
                e.changes
            ));
        }

        if !e.commits.is_empty() {
            formatted.push_str(&format!(
                "
```
{}
```
",
                e.commits
            ));
        }

        formatted
    }

    fn write(&self, data: String) -> Result<(), Box<dyn Error>> {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(CHANGELOG_FILE_NAME)?;
        let current = fs::read_to_string(CHANGELOG_FILE_NAME)?;
        let contents = format!("{}\n{}", data, current);
        file.write_all(contents.trim().as_bytes())?;

        Ok(())
    }
}
