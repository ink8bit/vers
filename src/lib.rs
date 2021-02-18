mod changelog;
mod git;
mod npm;

use changelog::{Changelog, Entry};

/// Update version with provided info
pub fn update(version: &str, info: &str, no_commit: bool) -> String {
    let v = match npm::version(version) {
        Ok(v) => v,
        Err(err) => panic!("Error: {}", err),
    };

    let releaser = git::user_name().expect("Could not get git user name");

    let changes = git::has_changes().expect("Could not execute git status");
    if !changes {
        panic!("Nothing to commit, working tree clean");
    }

    let commits = git::log().expect("Unable to collect your commits");

    let e = Entry {
        version: &v,
        changes: info.to_string(),
        releaser,
        commits,
    };
    let log = Changelog { entry: e };
    match log.update() {
        Ok(v) => println!("{}", v),
        Err(e) => eprintln!("{}", e),
    }

    if no_commit {
        return v;
    }

    match git::commit(&v) {
        Ok(msg) => println!("{}", msg),
        Err(err) => eprintln!("{}", err),
    }

    match git::tag(&v) {
        Ok(msg) => println!("{}", msg),
        Err(err) => eprintln!("{}", err),
    }

    match git::push() {
        Ok(msg) => println!("{}", msg),
        Err(err) => eprintln!("{}", err),
    }

    v
}

/// Commit and tag changes
pub fn save_changes(v: &str) -> Result<(), std::io::Error> {
    let commit_msg = git::commit(&v)?;
    println!("{}", commit_msg);

    let tag_msg = git::tag(&v)?;
    println!("{}", tag_msg);

    Ok(())
}

/// Push changes to the remote
pub fn push_changes() -> Result<(), Box<dyn std::error::Error>> {
    let msg = git::push()?;
    println!("{}", msg);

    Ok(())
}
