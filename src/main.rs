mod cli;

fn main() {
    let args = cli::args();
    let info = args.value_of("info").unwrap_or_default();
    let version = args.value_of_t("type").unwrap_or_else(|e| e.exit());
    let no_commit = args.is_present("no_commit");

    let v = vers::update(version, info, no_commit);
    println!("Updated version: {}", v);
}
