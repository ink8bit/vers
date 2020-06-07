mod version;
use version::{Version, VersionType};

fn run() {
    let ver = match version::get_parsed() {
        Ok(v) => v,
        Err(e) => panic!(e),
    };

    let parts: Vec<&str> = ver.split(".").collect();
    let version = Version::new(
        parts[0].to_owned(),
        parts[1].to_owned(),
        parts[2].to_owned(),
    );

    let v = Version::update(version, VersionType::Major);
    dbg!(v);
}

fn main() {
    run();
}
