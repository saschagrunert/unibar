//! The main library interface
#![deny(missing_docs)]

extern crate failure;
extern crate gfx_backend_gl as backend;
extern crate gfx_hal;
extern crate log;
extern crate winit;

mod bar;

pub use crate::bar::Bar;
