mod changelog;
mod git;
mod npm;
mod version;

use changelog::{Changelog, Entry};
use version::Version;

pub fn update(version: Version, info: &str, no_commit: bool) -> String {
    let ver_type = match version {
        Version::Major => "major",
        Version::Minor => "minor",
        Version::Patch => "patch",
    };

    let v = match npm::version(ver_type) {
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
