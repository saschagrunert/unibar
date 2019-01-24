use amethyst::{
    core::transform::Transform,
    ecs::prelude::{System, WriteStorage},
    ui::{UiFinder, UiText},
};
use chrono::Local;

pub struct StatusSystem;

impl<'s> System<'s> for StatusSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, UiText>,
        UiFinder<'s>,
    );

    fn run(&mut self, (mut _transforms, mut texts, finder): Self::SystemData) {
        // Update the date and time
        if let Some(t) =
            finder.find("date_time_text").and_then(|e| texts.get_mut(e))
        {
            t.text = Local::now().format(" %T  %a %e %b W%V").to_string();
        }
    }
}
