mod cli;
use cli::Version;

mod npm;
use npm::version;

fn main() {
    let args = cli::args();
    // dbg!(args);

    let t = args.value_of_t("type").unwrap_or_else(|e| e.exit());
    let ver_type = match t {
        Version::Major => "major",
        Version::Minor => "minor",
        Version::Patch => "patch",
    };

    match version(ver_type) {
        Ok(v) => println!("{}", v),
        Err(err) => panic!("Error: {}", err),
    }
}
