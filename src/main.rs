use colored::*;
use terminal_spinners::{SpinnerBuilder, DOTS};

mod cli;

fn main() {
    let args = cli::args();
    let info = args.value_of("info").unwrap_or_default();
    let version = args.value_of("version_type").unwrap();
    let no_commit = args.is_present("no_commit");

    let sp = SpinnerBuilder::new()
        .spinner(&DOTS)
        .text(" Updating...")
        .start();

    match vers::update(version, info, no_commit) {
        Ok(v) => {
            let msg = format!("Successfully created version {}", v)
                .green()
                .to_string();
            sp.text(msg);
            sp.done();
        }
        Err(e) => {
            let err_msg = format!("Error: {}", e).red().to_string();
            sp.text(err_msg);
            sp.error();
        }
    }
}
