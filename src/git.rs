use std::error::Error;
use std::process::Command;
use std::str;

pub(crate) fn commit<'a>(
    version: &str,
    releaser: &str,
    info: &str,
) -> Result<&'a str, std::io::Error> {
    let comment = create_comment(&version, &releaser, &info, false);
    let out = Command::new("git")
        .args(&["commit", "-n", "--cleanup=strip", "-m", &comment])
        .output();

    match out {
        Ok(_) => Ok("Successfully committed changes"),
        Err(e) => Err(e),
    }
}

pub(crate) fn add_all() -> Result<(), Box<dyn Error>> {
    Command::new("git").args(&["add", "--all"]).output()?;
    Ok(())
}

fn branch() -> Result<String, Box<dyn Error>> {
    let current_branch = branch_name()?;
    if !current_branch.is_empty() {
        return Ok(current_branch);
    }
    let current_branch_fallback = branch_name_fallback()?;
    Ok(current_branch_fallback)
}

/// Returns current `git` branch name
fn branch_name() -> Result<String, Box<dyn Error>> {
    let out = Command::new("git")
        .args(&["branch", "--show-current"])
        .output()?;
    let stdout = str::from_utf8(&out.stdout)?.trim();

    Ok(stdout.to_string())
}

/// Returns current git branch name.
/// It's only to support previous `git` versions
fn branch_name_fallback() -> Result<String, Box<dyn Error>> {
    let out = Command::new("git")
        .args(&["rev-parse", "--abbrev-ref", "HEAD"])
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

fn by_msg<'a>(is_tag: bool) -> &'a str {
    if is_tag {
        return "Tagged";
    }
    "Released"
}

fn create_comment(v: &str, releaser: &str, info: &str, is_tag: bool) -> String {
    let mut comment = format!("Version bump: {}\n", v);
    let msg = by_msg(is_tag);
    if !releaser.is_empty() {
        comment.push_str(&format!("\n{m} by: {r}", m = msg, r = releaser));
    }
    if !info.is_empty() {
        comment.push_str(&format!("\nInfo: {}", info));
    }

    comment.trim().to_string()
}

pub(crate) fn tag(version: &str, releaser: &str, info: &str) -> Result<String, std::io::Error> {
    let comment = create_comment(&version, &releaser, &info, true);
    let tag_cmd = Command::new("git")
        .args(&["tag", "-a", version, "-m", &comment])
        .output();

    match tag_cmd {
        Ok(_) => Ok(format!("Successfully created new tag {}", version)),
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

pub(crate) fn user_email() -> Result<String, Box<dyn Error>> {
    let out = Command::new("git")
        .args(&["config", "user.email"])
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

pub(crate) fn log() -> Result<String, Box<dyn Error>> {
    let out = Command::new("git")
        .args(&[
            "log",
            "--pretty=format:%h | %an: %s",
            "--no-merges",
            "master..HEAD",
            "--reverse",
        ])
        .output()?;

    let stdout = str::from_utf8(&out.stdout)?.trim();
    Ok(stdout.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_by_msg_tag() {
        let actual = by_msg(true);

        assert_eq!(actual, "Tagged");
    }

    #[test]
    fn test_by_msg_commit() {
        let actual = by_msg(false);

        assert_eq!(actual, "Released");
    }

    #[test]
    fn test_create_comment_tag() {
        let fake_version = "v1.2.3";
        let fake_user_name = "Fake username";
        let fake_info = "Fake info";

        let actual = create_comment(fake_version, fake_user_name, fake_info, true);
        let expected = format!(
            "Version bump: {v}

Tagged by: {u}
Info: {i}",
            v = fake_version,
            u = fake_user_name,
            i = fake_info,
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_create_comment_commit() {
        let fake_version = "v1.2.3";
        let fake_user_name = "Fake username";
        let fake_info = "Fake info";

        let actual = create_comment(fake_version, fake_user_name, fake_info, false);
        let expected = format!(
            "Version bump: {v}

Released by: {u}
Info: {i}",
            v = fake_version,
            u = fake_user_name,
            i = fake_info,
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_create_comment_tag_empty_username_and_info() {
        let fake_version = "v1.2.3";
        let fake_user_name = "";
        let fake_info = "";

        let actual = create_comment(fake_version, fake_user_name, fake_info, true);
        let expected = format!("Version bump: {v}", v = fake_version,);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_create_comment_commit_empty_username_and_info() {
        let fake_version = "v1.2.3";
        let fake_user_name = "";
        let fake_info = "";

        let actual = create_comment(fake_version, fake_user_name, fake_info, false);
        let expected = format!("Version bump: {v}", v = fake_version,);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_create_comment_tag_empty_username() {
        let fake_version = "v1.2.3";
        let fake_user_name = "";
        let fake_info = "Fake info";

        let actual = create_comment(fake_version, fake_user_name, fake_info, true);
        let expected = format!(
            "Version bump: {v}

Info: {i}",
            v = fake_version,
            i = fake_info,
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_create_comment_commit_empty_username() {
        let fake_version = "v1.2.3";
        let fake_user_name = "";
        let fake_info = "Fake info";

        let actual = create_comment(fake_version, fake_user_name, fake_info, false);
        let expected = format!(
            "Version bump: {v}

Info: {i}",
            v = fake_version,
            i = fake_info,
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_create_comment_tag_empty_info() {
        let fake_version = "v1.2.3";
        let fake_user_name = "Fake username";
        let fake_info = "";

        let actual = create_comment(fake_version, fake_user_name, fake_info, true);
        let expected = format!(
            "Version bump: {v}

Tagged by: {u}",
            v = fake_version,
            u = fake_user_name,
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_create_comment_commit_empty_info() {
        let fake_version = "v1.2.3";
        let fake_user_name = "Fake username";
        let fake_info = "";

        let actual = create_comment(fake_version, fake_user_name, fake_info, false);
        let expected = format!(
            "Version bump: {v}

Released by: {u}",
            v = fake_version,
            u = fake_user_name,
        );

        assert_eq!(actual, expected);
    }
}
