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

    let stdout = parse_std_out(out)?;
    Ok(stdout)
}

fn parse_std_out(out: std::process::Output) -> Result<String, Utf8Error> {
    let std_out = str::from_utf8(&out.stdout)?;
    Ok(std_out.to_string())
}
