mod version;

fn run() {
    match version::get_parsed() {
        Ok(v) => println!("Version: {}", v),
        Err(e) => println!("Package.json: {}", e),
    }
}

fn main() {
    run();
}
