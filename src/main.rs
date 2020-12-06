mod cli;
use cli::Version;

mod changelog;
mod npm;

use changelog::Changelog;

fn main() {
    let args = cli::args();
    // dbg!(args);

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

    let chlog = Changelog::new(v, String::from("some info"), String::from("someone"));
    let c = chlog.create();
    println!("{}", c);
}
