use amethyst::{
    core::transform::Transform,
    ecs::prelude::{System, WriteStorage},
    ui::{UiFinder, UiText},
};

pub struct WorkspaceSystem;

impl<'s> System<'s> for WorkspaceSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, UiText>,
        UiFinder<'s>,
    );

    fn run(&mut self, (mut _transforms, mut texts, finder): Self::SystemData) {
        if let Some(t) = finder
            .find("ws_button_1_btn_txt")
            .and_then(|e| texts.get_mut(e))
        {
            t.text = "0".to_owned();
        }
    }
}
