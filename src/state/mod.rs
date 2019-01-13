//! Global state handling for the bar
use amethyst::{input::is_key_down, prelude::*};
use winit::VirtualKeyCode;

/// The state representation of the bar
pub struct BarState;

impl SimpleState for BarState {
    fn handle_event(
        &mut self,
        _: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                Trans::Quit
            } else {
                Trans::None
            }
        } else {
            Trans::None
        }
    }
}
