use crate::segment::{Cpu, Date, Segment};
use amethyst::{
    core::{timing::Time, transform::Transform},
    ecs::{Read, System, Write, WriteStorage},
    ui::{UiFinder, UiText},
};

#[derive(Default)]
pub struct SegmentSystem {
    delta: f64,
}

impl SegmentSystem {
    fn button_txt(&self, id: &str) -> String {
        format!("{}_btn_txt", id)
    }
}

impl<'s> System<'s> for SegmentSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, UiText>,
        UiFinder<'s>,
        Read<'s, Time>,
        Write<'s, Date>,
        Write<'s, Cpu>,
    );

    fn run(
        &mut self,
        (mut _transforms, mut texts, finder, time, mut date, mut cpu): Self::SystemData,
    ) {
        // Update all segments every second and on startup
        if time.absolute_time_seconds() - self.delta >= 1. || self.delta == 0. {
            if let Some(t) = finder
                .find(&self.button_txt(date.id()))
                .and_then(|e| texts.get_mut(e))
            {
                date.update(t);
            }
            if let Some(t) = finder
                .find(&self.button_txt(cpu.id()))
                .and_then(|e| texts.get_mut(e))
            {
                cpu.update(t);
            }
            self.delta = time.absolute_time_seconds();
        }
    }
}
