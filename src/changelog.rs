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
    pub changes: String,
    pub releaser: String,
    pub commits: String,
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
        format!(
            "
## {version}

### {date}

**Releaser:** @{releaser}

**Changes:** {changes}

```
{commits}
```
",
            version = e.version,
            changes = e.changes,
            date = date,
            releaser = e.releaser,
            commits = e.commits,
        )
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
