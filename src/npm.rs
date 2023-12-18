use std::error::Error;
use std::process::Command;
use std::str;

/// Updates `package.json` and `package-lock.json` _version_ field
///
/// Uses `npm version` command under the hood
pub(crate) fn version(ver_type: &str) -> Result<String, Box<dyn Error>> {
    let out = Command::new("npm")
        .args([
            "version",
            ver_type,
            "--no-git-tag-version",
            "--no-commit-hooks",
        ])
        .output()?;

    let std_out = str::from_utf8(&out.stdout)?.trim();
    Ok(std_out.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn version_major() {
        assert!(version("major").is_ok());
    }

    #[test]
    #[ignore]
    fn version_minor() {
        assert!(version("minor").is_ok());
    }

    #[test]
    #[ignore]
    fn version_patch() {
        assert!(version("patch").is_ok());
    }
}
