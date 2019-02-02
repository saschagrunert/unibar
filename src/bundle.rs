use crate::system::{SegmentSystem, WorkspaceSystem};
use amethyst::{core::bundle::SystemBundle, ecs::DispatcherBuilder, Error};

/// This bundle prepares the world for the whole bar
pub struct Bundle;

impl<'a, 'b> SystemBundle<'a, 'b> for Bundle {
    fn build(
        self,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(WorkspaceSystem::new()?, "workspace_system", &[]);
        builder.add(SegmentSystem::default(), "segment_system", &[]);
        Ok(())
    }
}
