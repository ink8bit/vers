mod changelog;
mod git;
mod npm;

use changelog::{Changelog, Entry};
use std::{env, fmt, path::Path};

#[derive(Debug)]
pub enum VersError {
    GitName,
    GitEmail,
    GitBranch,
    GitRemote,
    GitStatus,
    GitLog,
    GitAddAll,
    GitCommit,
    GitTag,
    GitPush,
    DirtyWorkingArea,
    LogUpdate,
    VersionUpdate,
    PackageNotFound,
}

impl fmt::Display for VersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VersError::GitName => write!(f, "Could not get git user name"),
            VersError::GitEmail => write!(f, "Could not get git user email"),
            VersError::GitBranch => write!(f, "Could not get current git branch value"),
            VersError::GitRemote => write!(f, "Could not get git remote name"),
            VersError::GitStatus => write!(f, "Could not execute git status"),
            VersError::GitLog => write!(f, "Unable to collect your commits"),
            VersError::GitAddAll => write!(f, "Unable to add your changes to staging area"),
            VersError::GitCommit => write!(f, "Unable to commit your changes"),
            VersError::GitTag => write!(f, "Unable to create git tag"),
            VersError::GitPush => write!(f, "Unable to push your changes to the remote"),
            VersError::DirtyWorkingArea => write!(f, "Working area has changes to commit.\nYou can update version only if working area is clean"),
            VersError::LogUpdate => write!(f, "Could not update changelog file"),
            VersError::VersionUpdate => write!(f, "Could not update version"),
            VersError::PackageNotFound => write!(f, "File package.json not found"),
        }
    }
}

/// Returns releaser field with _git_ user name and email
/// or user GitHub handle if env var `VERS_GITHUB_NAME` is set
pub fn releaser() -> Result<String, VersError> {
    let github_name = env::var("VERS_GITHUB_NAME").unwrap_or_default();
    if !github_name.is_empty() {
        return Ok(format!("@{}", github_name));
    }

    let user_name = git::user_name().map_err(|_| VersError::GitName)?;
    let user_email = git::user_email().map_err(|_| VersError::GitEmail)?;
    let releaser = format!("{name} <{email}>", name = user_name, email = user_email);

    Ok(releaser)
}

/// Updates version and creates changelog,
/// commits and pushes your changes to the remote.
/// Also creates git tag for your release.
///
/// # Arguments
///
/// - `version` - version string
/// - `info` - some info you want to provide
/// - `no-commit` - should your changes be committed, tag created and pushed
pub fn update(version: &str, info: &str, no_commit: bool) -> Result<String, VersError> {
    if !Path::new("package.json").exists() {
        return Err(VersError::PackageNotFound);
    }

    let changes = git::has_changes().map_err(|_| VersError::GitStatus)?;
    if changes {
        return Err(VersError::DirtyWorkingArea);
    }

    let v = npm::version(version).map_err(|_| VersError::VersionUpdate)?;
    let r = releaser()?;
    let commits = git::log().map_err(|_| VersError::GitStatus)?;

    let e = Entry {
        version: &v,
        changes: info,
        releaser: &r,
        commits: &commits,
    };

    let log = Changelog { entry: e };

    let update_msg = log.update().map_err(|_| VersError::LogUpdate)?;
    println!("{}", update_msg);

    if no_commit {
        return Ok(v);
    }

    git::add_all().map_err(|_| VersError::GitAddAll)?;

    let commit_msg = git::commit(&v, &r, info).map_err(|_| VersError::GitCommit)?;
    println!("{}", commit_msg);

    let tag_msg = git::tag(&v, &r, info).map_err(|_| VersError::GitTag)?;
    println!("{}", tag_msg);

    let current_branch = git::branch().map_err(|_| VersError::GitBranch)?;
    if current_branch.is_empty() {
        return Err(VersError::GitBranch);
    }
    let remote_name = git::remote().map_err(|_| VersError::GitRemote)?;
    if remote_name.is_empty() {
        return Err(VersError::GitRemote);
    }
    let push_msg = git::push(&current_branch, &remote_name).map_err(|_| VersError::GitPush)?;
    println!("{}", push_msg);

    Ok(v)
}

/// Commits and creates tag for your changes
///
/// # Arguments
///
/// - `version`- version string
/// - `releaser_name` - releaser name which will be included in commit and tag message
/// - `info` - additional info you want to provide
pub fn save_changes(version: &str, releaser_name: &str, info: &str) -> Result<(), VersError> {
    git::add_all().map_err(|_| VersError::GitAddAll)?;

    let commit_msg =
        git::commit(&version, &releaser_name, &info).map_err(|_| VersError::GitCommit)?;
    println!("{}", commit_msg);

    let tag_msg = git::tag(&version, &releaser_name, &info).map_err(|_| VersError::GitTag)?;
    println!("{}", tag_msg);

    Ok(())
}

/// Pushes changes to the remote
pub fn push_changes() -> Result<(), VersError> {
    let current_branch = git::branch().map_err(|_| VersError::GitBranch)?;
    if current_branch.is_empty() {
        return Err(VersError::GitBranch);
    }

    let remote_name = git::remote().map_err(|_| VersError::GitRemote)?;
    if remote_name.is_empty() {
        return Err(VersError::GitRemote);
    }

    let msg = git::push(&current_branch, &remote_name).map_err(|_| VersError::GitPush)?;
    println!("{}", msg);

    Ok(())
}
