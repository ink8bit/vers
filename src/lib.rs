mod changelog;
mod error;
mod git;
mod npm;

use std::{env, path::Path};

use changelog::{Changelog, Entry};
use error::VersError;

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
/// commits and pushes your changes to the remote,
/// creates git tag for your release. Returns an updated version value.
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
    log.update().map_err(|_| VersError::LogUpdate)?;

    if no_commit {
        return Ok(v);
    }

    git::add_all().map_err(|_| VersError::GitAddAll)?;
    git::commit(&v, &r, info).map_err(|_| VersError::GitCommit)?;
    git::tag(&v, &r, info).map_err(|_| VersError::GitTag)?;

    let current_branch = git::branch().map_err(|_| VersError::GitBranch)?;
    if current_branch.is_empty() {
        return Err(VersError::GitBranch);
    }

    let remote_name = git::remote().map_err(|_| VersError::GitRemote)?;
    if remote_name.is_empty() {
        return Err(VersError::GitRemote);
    }

    git::push(&current_branch, &remote_name).map_err(|_| VersError::GitPush)?;

    Ok(v)
}

/// Performs a commit and creates tag for your changes,
/// returns a newly created tag value.
///
/// # Arguments
///
/// - `version`- version string
/// - `releaser_name` - releaser name which will be included in commit and tag message
/// - `info` - additional info you want to provide
pub fn save_changes(version: &str, releaser_name: &str, info: &str) -> Result<String, VersError> {
    git::add_all().map_err(|_| VersError::GitAddAll)?;
    git::commit(&version, &releaser_name, &info).map_err(|_| VersError::GitCommit)?;
    let tag = git::tag(&version, &releaser_name, &info).map_err(|_| VersError::GitTag)?;

    Ok(tag)
}

/// Pushes changes to the remote, returns current git branch
pub fn push_changes() -> Result<String, VersError> {
    let current_branch = git::branch().map_err(|_| VersError::GitBranch)?;
    if current_branch.is_empty() {
        return Err(VersError::GitBranch);
    }

    let remote_name = git::remote().map_err(|_| VersError::GitRemote)?;
    if remote_name.is_empty() {
        return Err(VersError::GitRemote);
    }

    let branch = git::push(&current_branch, &remote_name).map_err(|_| VersError::GitPush)?;

    Ok(branch)
}

/// Returns current `git` branch value
pub fn current_branch_name() -> Result<String, VersError> {
    let branch_name = git::branch().map_err(|_| VersError::GitBranch)?;
    if branch_name.is_empty() {
        return Err(VersError::GitBranch);
    }
    Ok(branch_name)
}
