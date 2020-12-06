mod changelog;
mod cli;
mod git;
mod npm;

use changelog::Changelog;
use cli::Version;

fn main() {
    let args = cli::args();
    let r = args.value_of("releaser").unwrap();
    let i = args.value_of("info").unwrap();
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

    let ver = v.to_owned();
    let chlog = Changelog::new(v, i.to_string(), r.to_string());
    let c = chlog.create();

    chlog.write(c).expect("Can not create changelog file");

    match git::commit(&ver) {
        Ok(msg) => println!("{}", msg),
        Err(err) => eprintln!("{}", err),
    }
}
