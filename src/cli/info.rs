use clap::Arg;

pub(crate) const INFO_ARG: &str = "info";

pub(crate) fn info() -> Arg<'static> {
    Arg::new(INFO_ARG)
        .short('i')
        .long("info")
        .value_name("string")
        .help("Set info value")
        .takes_value(true)
}
