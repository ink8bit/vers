use colored::*;
use terminal_spinners::{SpinnerBuilder, DOTS};

mod cli;

use cli::{commit::COMMIT_ARG, info::INFO_ARG, version::VERSION_ARG};

fn main() {
    let matches = cli::args();
    let version = matches.get_one::<String>(VERSION_ARG).unwrap();
    let no_commit = matches.get_flag(COMMIT_ARG);
    let default_info = "".to_string();
    let info = matches.get_one::<String>(INFO_ARG).unwrap_or(&default_info);

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
