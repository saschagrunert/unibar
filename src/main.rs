extern crate env_logger;
extern crate failure;
extern crate gbar;

use failure::Fallible;
use gbar::Bar;
use std::env::set_var;

fn main() -> Fallible<()> {
    // Set the logging verbosity
    set_var("RUST_LOG", "gbar=debug");

    // Setup the logger
    env_logger::init();

    // Init the bar
    let mut bar = Bar::new(200, 50)?;

    // Run the bar
    bar.run()?;

    Ok(())
}
