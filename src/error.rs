use colored::*;

use std::fmt;

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

fn colored(message: &str) -> ColoredString {
    message.red()
}

impl fmt::Display for VersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VersError::GitName => write!(f, "{}", colored("Could not get git user name")),
            VersError::GitEmail => write!(f, "{}",  colored("Could not get git user email")),
            VersError::GitBranch => write!(f, "{}", colored("Could not get current git branch value")),
            VersError::GitRemote => write!(f, "{}", colored("Could not get git remote name")),
            VersError::GitStatus => write!(f, "{}", colored("Could not execute git status")),
            VersError::GitLog => write!(f, "{}", colored("Unable to collect your commits")),
            VersError::GitAddAll => write!(f, "{}", colored("Unable to add your changes to staging area")),
            VersError::GitCommit => write!(f, "{}", colored("Unable to commit your changes")),
            VersError::GitTag => write!(f, "{}", colored("Unable to create git tag")),
            VersError::GitPush => write!(f, "{}", colored("Unable to push your changes to the remote")),
            VersError::DirtyWorkingArea => write!(f, "{}", colored("Working area has changes to commit.\nYou can update version only if working area is clean")),
            VersError::LogUpdate => write!(f, "{}", colored("Could not update changelog file")),
            VersError::VersionUpdate => write!(f, "{}", colored("Could not update version")),
            VersError::PackageNotFound => write!(f, "{}", colored("File package.json not found")),
        }
    }
}
