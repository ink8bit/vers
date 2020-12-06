use chrono::prelude::*;

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

    pub fn create(self) -> String {
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

    fn write() {
        todo!();
    }

    fn exists() {
        todo!();
    }
}
