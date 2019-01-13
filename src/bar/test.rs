#![cfg(test)]
use crate::bar::Bar;
use failure::Fallible;
use log::LevelFilter;

#[test]
fn succeed_to_create_bar() -> Fallible<()> {
    // Given
    // When
    let bar = Bar::run(LevelFilter::Error, true);

    // Then
    assert!(bar.is_ok());
    Ok(())
}
