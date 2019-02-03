use crate::workspace::Workspace;
use amethyst::{
    assets::{AssetStorage, Loader},
    ecs::{Entities, Read, ReadExpect, System, Write},
    error::format_err,
    renderer::Texture,
    shrev::{EventChannel, ReaderId},
    ui::{FontAsset, UiButtonBuilderResources, UiEvent, UiEventType},
    Error,
};
use i3ipc::{event::Event, I3Connection, I3EventListener, Subscription};
use log::{debug, error, warn};
use std::{
    collections::HashMap,
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

pub struct WorkspaceSystem {
    i3_connection: I3Connection,
    event_receiver: Receiver<Event>,
    number_of_workspaces: usize,
    workspace_to_draw: usize,
    update_workspaces: bool,
    workspaces: HashMap<usize, Workspace>,
    reader_id: Option<ReaderId<UiEvent>>,
}

impl WorkspaceSystem {
    /// Create a new workspace system
    pub fn new() -> Result<Self, Error> {
        let mut event_listener = I3EventListener::connect()
            .map_err(|_| format_err!("unable to establish i3 connection"))?;

        event_listener
            .subscribe(&[Subscription::Workspace])
            .map_err(|_| format_err!("unable to subscribe to i3 events"))?;

        let (tx, rx): (Sender<Event>, Receiver<Event>) = mpsc::channel();

        thread::spawn(move || {
            for event in event_listener.listen() {
                match event {
                    Ok(e) => {
                        debug!("Received i3 event: {:?}", e);
                        if let Err(err) = tx.send(e) {
                            error!("Unable to send i3 event: {}", err);
                        }
                    }
                    Err(e) => warn!("Unable to get i3 event: {}", e),
                }
            }
        });

        let i3_connection = I3Connection::connect()
            .map_err(|_| format_err!("unable to establish i3 connection"))?;

        Ok(Self {
            i3_connection,
            event_receiver: rx,
            number_of_workspaces: 0,
            workspace_to_draw: 0,
            update_workspaces: true,
            workspaces: HashMap::default(),
            reader_id: None,
        })
    }
}

impl<'s> System<'s> for WorkspaceSystem {
    type SystemData = (
        Write<'s, EventChannel<UiEvent>>,
        ReadExpect<'s, Loader>,
        Read<'s, AssetStorage<Texture>>,
        Read<'s, AssetStorage<FontAsset>>,
        UiButtonBuilderResources<'s, u8>,
        Entities<'s>,
    );

    fn run(
        &mut self,
        (
            mut events,
            loader,
            texture_storage,
            font_storage,
            button_builder_resources,
            entities,
        ): Self::SystemData,
    ) {
        // Process UI events
        let reader_id = self
            .reader_id
            .get_or_insert_with(|| events.register_reader());
        for event in events.read(reader_id) {
            // Switch the workspace
            if let UiEventType::Click = event.event_type {
                // Get the corresponding workspace for the target entity
                for workspace in self.workspaces.values() {
                    if workspace.has_entity(event.target) {
                        if let Err(e) = self.i3_connection.run_command(
                            &format!("workspace {}", workspace.name()),
                        ) {
                            error!("Unable to switch workspace: {}", e);
                        }
                        break;
                    }
                }
            }
        }

        // Check if we need to update the workspaces
        for e in self.event_receiver.try_iter() {
            if let Event::WorkspaceEvent(_) = e {
                self.update_workspaces = true
            }
        }

        if self.update_workspaces {
            match self.i3_connection.get_workspaces() {
                Ok(response) => {
                    self.number_of_workspaces = response.workspaces.len();

                    // Stop of no workspaces were found
                    if self.number_of_workspaces == 0 {
                        error!("No workspaces found");
                        return;
                    }

                    // Retrieve the workspace
                    let workspace = self
                        .workspaces
                        .entry(self.workspace_to_draw)
                        .or_insert_with(Workspace::new);

                    // Update the workspace
                    if let Err(e) = workspace.update(
                        &response.workspaces[self.workspace_to_draw],
                        &loader,
                        &texture_storage,
                        &font_storage,
                        button_builder_resources,
                        entities,
                    ) {
                        error!("Unable to update workspace: {}", e)
                    }

                    // Sanitize for index out of bounds checks
                    if self.workspace_to_draw >= self.number_of_workspaces - 1 {
                        self.workspace_to_draw = 0;
                        self.update_workspaces = false;
                    } else {
                        self.workspace_to_draw += 1;
                    }
                }
                Err(e) => error!("Unable to retrieve workspaces: {}", e),
            }
        }
    }
}
