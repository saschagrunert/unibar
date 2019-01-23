use crate::system::{StatusSystem, WorkspaceSystem};
use amethyst::{
    core::bundle::{Result, SystemBundle},
    ecs::prelude::DispatcherBuilder,
};

/// This bundle prepares the world for the whole bar
pub struct BarBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for BarBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        builder.add(WorkspaceSystem, "workspace_system", &[]);
        builder.add(StatusSystem, "status_system", &[]);
        Ok(())
    }
}
