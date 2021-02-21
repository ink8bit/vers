mod changelog;
mod git;
mod npm;

use changelog::{Changelog, Entry};
use std::fmt;

#[derive(Debug)]
pub enum VersError {
    GitName,
    GitStatus,
    GitLog,
    GitCommit,
    GitTag,
    GitPush,
    DirtyWorkingArea,
    LogUpdate,
    VersionUpdate,
}

impl fmt::Display for VersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VersError::GitName => write!(f, "Could not get git user name"),
            VersError::GitStatus => write!(f, "Could not execute git status"),
            VersError::GitLog => write!(f, "Unable to collect your commits"),
            VersError::GitCommit => write!(f, "Unable to commit your changes"),
            VersError::GitTag => write!(f, "Unable to create git tag"),
            VersError::GitPush => write!(f, "Unable to push your changes to the remote"),
            VersError::DirtyWorkingArea => write!(f, "Working area has changes to commit.\nYou can update version only if working area is clean."),
            VersError::LogUpdate => write!(f, "Could not update changelog file"),
            VersError::VersionUpdate => write!(f, "Could not update version"),
        }
    }
}

/// Update version with provided info
pub fn update(version: &str, info: &str, no_commit: bool) -> Result<String, VersError> {
    let v = npm::version(version).map_err(|_| VersError::VersionUpdate)?;
    let releaser = git::user_name().map_err(|_| VersError::GitName)?;

    let changes = git::has_changes().map_err(|_| VersError::GitStatus)?;
    if changes {
        return Err(VersError::DirtyWorkingArea);
    }

    let commits = git::log().map_err(|_| VersError::GitStatus)?;

    let e = Entry {
        version: &v,
        changes: info.to_string(),
        releaser,
        commits,
    };

    let log = Changelog { entry: e };

    let update_msg = log.update().map_err(|_| VersError::LogUpdate)?;
    println!("{}", update_msg);

    if no_commit {
        return Ok(v);
    }

    let commit_msg = git::commit(&v).map_err(|_| VersError::GitCommit)?;
    println!("{}", commit_msg);

    let tag_msg = git::tag(&v).map_err(|_| VersError::GitTag)?;
    println!("{}", tag_msg);

    let push_msg = git::push().map_err(|_| VersError::GitPush)?;
    println!("{}", push_msg);

    Ok(v)
}

/// Commit and tag changes
pub fn save_changes(v: &str) -> Result<(), VersError> {
    let commit_msg = git::commit(&v).map_err(|_| VersError::GitCommit)?;
    println!("{}", commit_msg);

    let tag_msg = git::tag(&v).map_err(|_| VersError::GitTag)?;
    println!("{}", tag_msg);

    Ok(())
}

/// Push changes to the remote
pub fn push_changes() -> Result<(), VersError> {
    let msg = git::push().map_err(|_| VersError::GitPush)?;
    println!("{}", msg);

    Ok(())
}
