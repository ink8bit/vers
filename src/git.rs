use std::error::Error;
use std::process::Command;
use std::str;

pub(crate) fn commit(version: &str) -> Result<&str, std::io::Error> {
    let ver_str = format!("Version bump: {}", version);
    let out = Command::new("git")
        .args(&["commit", "-n", "-a", "--cleanup=strip", "-m", &ver_str])
        .output();

    match out {
        Ok(_) => Ok("Successfully committed changes"),
        Err(e) => Err(e),
    }
}

fn branch() -> Result<String, Box<dyn Error>> {
    let out = Command::new("git")
        .args(&["branch", "--show-current"])
        .output()?;
    let stdout = str::from_utf8(&out.stdout)?.trim();

    Ok(stdout.to_string())
}

pub(crate) fn push() -> Result<String, Box<dyn Error>> {
    let branch_name = branch()?;
    let origin = remote()?;

    let _out = Command::new("git")
        .args(&["push", "--follow-tags", origin.as_str(), &branch_name])
        .output()?;

    Ok(format!(
        "Successfully pushed changes to remote branch '{}'",
        branch_name
    ))
}

fn remote() -> Result<String, Box<dyn Error>> {
    let out = Command::new("git").arg("remote").output()?;
    let stdout = str::from_utf8(&out.stdout)?.trim();

    Ok(stdout.to_string())
}

pub(crate) fn tag(v: &str) -> Result<String, std::io::Error> {
    let version = format!("Version: {}", v);
    let tag_cmd = Command::new("git")
        .args(&["tag", "-a", v, "-m", &version])
        .output();

    match tag_cmd {
        Ok(_) => Ok(format!("Successfully created new tag {}", v)),
        Err(e) => Err(e),
    }
}

pub(crate) fn user_name() -> Result<String, Box<dyn Error>> {
    let out = Command::new("git")
        .args(&["config", "user.name"])
        .output()?;

    let stdout = str::from_utf8(&out.stdout)?.trim();
    Ok(stdout.to_string())
}

fn status() -> Result<String, Box<dyn Error>> {
    let out = Command::new("git").args(&["status", "-s"]).output()?;
    let stdout = str::from_utf8(&out.stdout)?.trim();
    Ok(stdout.to_string())
}

pub(crate) fn has_changes() -> Result<bool, Box<dyn Error>> {
    match status() {
        Ok(v) => {
            if v.is_empty() {
                Ok(false)
            } else {
                Ok(true)
            }
        }
        Err(e) => Err(e),
    }
}
