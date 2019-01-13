//! Global state handling for the bar
mod test;

use amethyst::{input::is_close_requested, prelude::*};
use i3ipc::I3Connection;
use log::info;

/// The state representation of the bar
pub struct BarState {
    /// The i3 connection
    pub i3_conn: I3Connection,
}

impl SimpleState for BarState {
    fn handle_event(
        &mut self,
        _: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        match &event {
            StateEvent::Window(event) => {
                // Close the bar when requested
                if is_close_requested(&event) {
                    Trans::Quit
                } else {
                    Trans::None
                }
            }
            StateEvent::Ui(ui_event) => {
                info!("Got UI event: {:?}", ui_event);
                Trans::None
            }
        }
    }
}
