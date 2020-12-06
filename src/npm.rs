use std::error::Error;
use std::process::Command;
use std::str;
use std::str::Utf8Error;

pub fn version(ver_type: &str) -> Result<String, Box<dyn Error>> {
    let out = Command::new("npm")
        .arg("version")
        .arg(ver_type)
        .arg("--no-git-tag-version")
        .arg("--no-commit-hooks")
        .output()?;

    let stdout = parse_std_out(&out.stdout)?;
    Ok(stdout)
}

fn parse_std_out(out: &Vec<u8>) -> Result<String, Utf8Error> {
    let std_out = str::from_utf8(&out)?;
    let v = remove_newline(std_out);
    Ok(v.to_string())
}

fn remove_newline(version: &str) -> String {
    version.replace("\n", "")
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
    fn version_major() {
        assert!(version("major").is_ok());
    }

    #[test]
    fn version_minor() {
        assert!(version("minor").is_ok());
    }

    #[test]
    fn version_patch() {
        assert!(version("patch").is_ok());
    }

    #[test]
    fn successfuly_remove_newline() {
        assert_eq!(remove_newline("new_line\n"), "new_line");
    }

    #[test]
    fn do_not_change_string_without_newline() {
        assert_eq!(remove_newline("without_new_line"), "without_new_line");
    }
}
