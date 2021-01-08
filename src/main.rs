mod changelog;
mod cli;
mod git;
mod npm;

use changelog::Changelog;
use cli::Version;

fn main() {
    let args = cli::args();
    let info = args.value_of("info").unwrap();
    let t = args.value_of_t("type").unwrap_or_else(|e| e.exit());
    let ver_type = match t {
        Version::Major => "major",
        Version::Minor => "minor",
        Version::Patch => "patch",
    };

    let v = match npm::version(ver_type) {
        Ok(v) => v,
        Err(err) => panic!("Error: {}", err),
    };

    let releaser = git::user_name().expect("Could not get git user name");
    let chlog = Changelog::new(&v, info.to_string(), releaser);
    let c = chlog.create();

    let changes = git::has_changes().expect("Could not execute git status");
    if !changes {
        return println!("Nothing to commit, working tree clean.");
    }

    chlog.write(c).expect("Can not create changelog file");

    match git::commit(&v) {
        Ok(msg) => println!("{}", msg),
        Err(err) => eprintln!("{}", err),
    }

    match git::tag(&v) {
        Ok(msg) => println!("{}", msg),
        Err(err) => eprintln!("{}", err),
    }
}
