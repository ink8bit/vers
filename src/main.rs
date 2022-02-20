use colored::*;
use terminal_spinners::{SpinnerBuilder, DOTS};

mod cli;

use cli::{commit::COMMIT_ARG, info::INFO_ARG, version::VERSION_ARG};

fn main() {
    let args = cli::args();
    let info = args.value_of(INFO_ARG).unwrap_or_default();
    let version = args.value_of(VERSION_ARG).unwrap();
    let no_commit = args.is_present(COMMIT_ARG);

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
