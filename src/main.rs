mod version;
use version::Version;

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

    dbg!(version);
}

fn main() {
    run();
}
