//! Bar definitions and functions
use failure::Fallible;
use i3ipc::I3Connection;
use log::debug;
use winit::{
    dpi::LogicalSize,
    os::unix::{WindowBuilderExt, XWindowType},
    EventsLoop, WindowBuilder,
};

#[allow(dead_code)]
/// The root bar structure
pub struct Bar {
    events_loop: EventsLoop,
}

impl Bar {
    /// Create a new Bar instance
    pub fn new() -> Fallible<Self> {
        debug!("Connecting to i3");
        let mut i3ipc = I3Connection::connect()?;
        debug!("Found i3 version {}", i3ipc.get_version()?.human_readable);

        debug!("Creating events loop");
        let events_loop = EventsLoop::new();

        debug!("Building window");
        let _ = WindowBuilder::new()
            .with_dimensions(LogicalSize::new(20.0, 20.0))
            .with_x11_window_type(XWindowType::Dock)
            .with_resizable(false)
            .with_decorations(false)
            .with_always_on_top(true)
            .build(&events_loop)?;

        debug!("Initialization done");
        Ok(Bar { events_loop })
    }

    /// Run the bar
    pub fn run(&mut self) -> Fallible<()> {
        Ok(())
    }
}
