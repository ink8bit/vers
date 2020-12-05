use std::process::Command;
use std::str;

fn main() {
    let output = Command::new("npm")
        .arg("-v")
        .output()
        .expect("failed to execute command");

    let status = output.status;
    if !status.success() {
        panic!("Error: can't execute `npm -v`");
    }

    let stdout = str::from_utf8(&output.stdout).unwrap();
    dbg!(stdout);
}
