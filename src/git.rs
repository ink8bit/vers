use std::error::Error;
use std::process::Command;
use std::str;

pub fn commit(version: &String) -> Result<String, Box<dyn Error>> {
    let ver_str = format!("Version bump: {}", version);
    let out = Command::new("git")
        .args(&["commit", "-n", "-a", "--cleanup=strip", "-m", &ver_str])
        .output()?;

    let stdout = str::from_utf8(&out.stdout)?;
    Ok(stdout.to_string())
}

#[allow(dead_code)]
pub fn push() -> Result<String, Box<dyn Error>> {
    todo!();
}

pub fn tag(v: &String) -> Result<String, std::io::Error> {
    let version = format!("Version: {}", v);
    let tag_cmd = Command::new("git")
        .args(&["tag", "-a", v, "-m", &version])
        .output();

    match tag_cmd {
        Ok(_) => Ok(format!("Successfully created new tag {}", v)),
        Err(e) => Err(e),
    }
}

pub fn user_name() -> Result<String, Box<dyn Error>> {
    let out = Command::new("git")
        .args(&["config", "user.name"])
        .output()?;

    let stdout = str::from_utf8(&out.stdout)?;
    Ok(stdout.to_string())
}

fn status() -> Result<String, Box<dyn Error>> {
    let out = Command::new("git").args(&["status", "-s"]).output()?;
    let stdout = str::from_utf8(&out.stdout)?;
    Ok(stdout.to_string())
}

pub fn has_changes() -> Result<bool, Box<dyn Error>> {
    match status() {
        Ok(v) => {
            if v.is_empty() {
                Ok(true)
            } else {
                Ok(false)
            }
        }
        Err(e) => Err(e),
    }
}
