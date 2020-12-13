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

#[allow(dead_code)]
pub fn tag() -> Result<String, Box<dyn Error>> {
    todo!();
}

pub fn user_name() -> Result<String, Box<dyn Error>> {
    let out = Command::new("git")
        .args(&["config", "user.name"])
        .output()?;

    let stdout = str::from_utf8(&out.stdout)?;
    Ok(stdout.to_string())
}
