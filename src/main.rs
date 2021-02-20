mod cli;

fn main() {
    let args = cli::args();
    let info = args.value_of("info").unwrap_or_default();
    let version = args.value_of("version_type").unwrap();
    let no_commit = args.is_present("no_commit");

    match vers::update(version, info, no_commit) {
        Ok(v) => println!("Updated version: {}", v),
        Err(e) => eprintln!("{}", e),
    }
}
