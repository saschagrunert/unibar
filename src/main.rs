use clap::{crate_version, load_yaml, App};
use failure::Fallible;
use log::LevelFilter;
use unibar::Bar;

fn main() -> Fallible<()> {
    // Load the CLI parameters from YAML
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).version(crate_version!()).get_matches();

    // Vary the output based on how many times the user used the "verbose" flag
    let level_filter = match matches.occurrences_of("verbose") {
        0 => LevelFilter::Error,
        1 => LevelFilter::Warn,
        2 => LevelFilter::Info,
        3 => LevelFilter::Debug,
        4 | _ => LevelFilter::Trace,
    };

    // Init and start the bar
    Bar::run(level_filter)?;

    Ok(())
}
