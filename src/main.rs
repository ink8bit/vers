mod cli;

fn main() {
    let args = cli::args();
    let info = args.value_of("info").unwrap_or_default();
    let version = args.value_of("version").unwrap();
    let no_commit = args.is_present("no_commit");

    let v = vers::update(version, info, no_commit);
    println!("Updated version: {}", v);

    if let Err(e) = vers::save_changes(&v) {
        panic!(e);
    }

    if let Err(e) = vers::push_changes() {
        eprintln!("{}", e);
    }
}
