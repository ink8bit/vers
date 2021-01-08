use std::error::Error;
use std::process::Command;
use std::str;
use std::str::Utf8Error;

pub fn version(ver_type: &str) -> Result<String, Box<dyn Error>> {
    let out = Command::new("npm")
        .args(&[
            "version",
            ver_type,
            "--no-git-tag-version",
            "--no-commit-hooks",
        ])
        .output()?;

    let stdout = parse_std_out(&out.stdout)?;
    Ok(stdout)
}

fn parse_std_out(out: &[u8]) -> Result<String, Utf8Error> {
    let std_out = str::from_utf8(&out)?;
    let v = std_out.trim();
    Ok(v.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_stdout_empty() {
        let out: Vec<u8> = Vec::new();
        assert_eq!(parse_std_out(&out).unwrap(), String::from(""));
    }

    #[test]
    fn parse_stdout_valid_data() {
        let out: Vec<u8> = Vec::from("v1.3.0");
        assert_eq!(parse_std_out(&out).unwrap(), String::from("v1.3.0"));
    }

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
