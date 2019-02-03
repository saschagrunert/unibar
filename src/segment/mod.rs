//! All available data segments

mod cpu;
mod date;

pub use self::{cpu::Cpu, date::Date};

pub trait Segment {
    /// The item which has to be updated
    type Item;

    /// Create a new instance
    fn new() -> Self;

    /// Update the segment
    fn update(&mut self, _: &mut Self::Item) {}

    /// Retrieve the unique identifier
    fn id(&self) -> &str;

    /// Do something on click
    fn click(&self, _: &mut Self::Item) {}
}
