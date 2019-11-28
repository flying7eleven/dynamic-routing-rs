use chrono::Local;
use clap::{crate_authors, crate_description, crate_name, crate_version};
use clap::{load_yaml, App};
use log::LevelFilter;

fn setup_logger() {
    let _ = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(LevelFilter::Trace)
        .chain(std::io::stdout())
        .apply();
}

fn main() {
    // configure the command line parser
    let cli_parser_config = load_yaml!("cli.yml");
    let matches = App::from_yaml(cli_parser_config)
        .author(crate_authors!())
        .version(crate_version!())
        .name(crate_name!())
        .about(crate_description!())
        .get_matches();

    // initialize the logger
    setup_logger();
}
