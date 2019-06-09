#[macro_use]
extern crate structopt;

#[macro_use]
extern crate serde_derive;
use structopt::StructOpt;

mod version;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    /// Activate debug mode
    #[structopt(short = "d", long = "debug")]
    debug: bool,
    /// Set speed
    #[structopt(short = "s", long = "speed", default_value = "42")]
    speed: f64,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);

    println!("VERSION: {}", version::get_parsed());
}
