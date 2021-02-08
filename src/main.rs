mod cli;

fn main() {
    let args = cli::args();
    let info = args.value_of("info").unwrap();
    let t = args.value_of_t("type").unwrap_or_else(|e| e.exit());

    vers::update(t, info);
}
