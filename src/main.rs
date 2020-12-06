use std::process::Command;
use std::str;

mod cli;
use cli::Version;

fn main() {
    let args = cli::args();
    // dbg!(args);

    let t = args.value_of_t("type").unwrap_or_else(|e| e.exit());
    let ver_type = match t {
        Version::Major => "major",
        Version::Minor => "minor",
        Version::Patch => "patch",
    };

    dbg!(ver_type);

    let output = Command::new("npm")
        .arg("version")
        .arg(ver_type)
        .output()
        .expect("failed to execute command");

    let status = output.status;
    let stderr = str::from_utf8(&output.stderr).unwrap();

    if !status.success() {
        eprintln!("{}", stderr);
    }

    let stdout = str::from_utf8(&output.stdout).unwrap();
    dbg!(stdout);
}
