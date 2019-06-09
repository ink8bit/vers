#[macro_use]
extern crate structopt;

#[macro_use]
extern crate serde_derive;
use structopt::StructOpt;

mod version;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "vers",
    about = "vers is a CLI app for updating your app's version and creating changelog file."
)]
struct Opt {
    /// Update patch version
    #[structopt(short = "p", long = "patch")]
    patch: bool,
    /// Update minor version
    #[structopt(short = "m", long = "minor")]
    minor: bool,
    /// Update major version
    #[structopt(short = "j", long = "major")]
    major: bool,
}

fn main() {
    let opt = Opt::from_args();
    // println!("{:?}", opt);

    if opt.patch {
        println!("PATCH: {}", opt.patch);
    }
    if opt.minor {
        println!("MINOR: {}", opt.minor);
    }
    if opt.major {
        println!("MAJOR: {}", opt.major);
    }

    println!("VERSION: {}", version::get_parsed());
}
