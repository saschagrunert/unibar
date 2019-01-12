use clap::{crate_version, load_yaml, App};
use env_logger::Builder;
use failure::Fallible;
use gbar::Bar;
use log::{info, LevelFilter};

fn main() -> Fallible<()> {
    // Load the CLI parameters from YAML
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).version(crate_version!()).get_matches();

    // Vary the output based on how many times the user used the "verbose" flag
    let verbosity = match matches.occurrences_of("verbose") {
        0 => LevelFilter::Error,
        1 => LevelFilter::Warn,
        2 => LevelFilter::Info,
        3 => LevelFilter::Debug,
        4 | _ => LevelFilter::Trace,
    };

    // Set the logging verbosity
    Builder::new().filter_level(verbosity).try_init()?;
    info!("Set logging verbosity to: {}", verbosity);

    // Init the bar
    let mut gbar = Bar::new()?;

    // Run the bar
    gbar.run()?;

    Ok(())
}
