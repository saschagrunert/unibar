use crate::{color::ColorScheme, state::State};
use amethyst::{
    assets::{AssetStorage, Loader},
    ecs::{Entities, Entity, Read, ReadExpect},
    renderer::{PngFormat, Texture, TextureHandle, TextureMetadata},
    ui::{
        Anchor, FontAsset, FontHandle, TtfFormat, UiButtonBuilder,
        UiButtonBuilderResources,
    },
};
use failure::Fallible;
use i3ipc::reply::Workspace as I3Workspace;

#[derive(Default)]
pub struct Workspace {
    name: String,
    entity: Option<Entity>,
}

impl Workspace {
    /// Create a new workspace instance
    pub fn new() -> Self {
        Self {
            name: "".to_owned(),
            entity: None,
        }
    }

    /// Update the workspace
    pub fn update<'s>(
        &mut self,
        i3_workspace: &I3Workspace,
        loader: &ReadExpect<'s, Loader>,
        texture_storage: &Read<'s, AssetStorage<Texture>>,
        font_storage: &Read<'s, AssetStorage<FontAsset>>,
        button_builder_resources: UiButtonBuilderResources<'s, u8>,
        entities: Entities<'s>,
    ) -> Fallible<()> {
        // Remove the entity if available
        if let Some(e) = self.entity {
            entities.delete(e)?;
        }

        // Load the font
        let font = self.load_font(loader, font_storage);

        // Prepare workspace name
        let name = i3_workspace
            .name
            .split(':')
            .last()
            .unwrap_or(&i3_workspace.name);

        // Create the button
        let mut button_builder = UiButtonBuilder::new(
            format!("ws_button_{}", i3_workspace.num),
            name,
        )
        .with_anchor(Anchor::TopLeft)
        .with_font(font.clone())
        .with_font_size(State::font_size())
        .with_position((20 * (i3_workspace.num - 1) + 10) as f32, -10.)
        .with_size(20., 20.);

        match (i3_workspace.visible, i3_workspace.focused) {
            (true, true) => {
                button_builder = button_builder
                    .with_image(self.load_texture(
                        "images/purple.png",
                        loader,
                        texture_storage,
                    ))
                    .with_text_color(ColorScheme::black())
            }
            (true, false) => {
                button_builder = button_builder
                    .with_image(self.load_texture(
                        "images/selection.png",
                        loader,
                        texture_storage,
                    ))
                    .with_text_color(ColorScheme::foreground())
            }
            (false, _) => {
                button_builder = button_builder
                    .with_image(self.load_texture(
                        "images/background.png",
                        loader,
                        texture_storage,
                    ))
                    .with_text_color(ColorScheme::foreground())
            }
        }

        // Build the entity and add to the world
        self.entity = Some(button_builder.build(button_builder_resources));
        self.name = i3_workspace.name.to_owned();
        Ok(())
    }

    /// Retrieve the name of the workspace
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Retrieve the entity
    pub fn has_entity(&self, entity: Entity) -> bool {
        if let Some(e) = self.entity {
            e == entity
        } else {
            false
        }
    }

    fn load_texture<'s, N>(
        &self,
        name: N,
        loader: &ReadExpect<'s, Loader>,
        storage: &Read<'s, AssetStorage<Texture>>,
    ) -> TextureHandle
    where
        N: Into<String>,
    {
        loader.load(
            name,
            PngFormat,
            // Screen pixel will be taken from nearest pixel of texture
            TextureMetadata::srgb_scale(),
            (),
            storage,
        )
    }

    fn load_font<'s>(
        &self,
        loader: &ReadExpect<'s, Loader>,
        storage: &Read<'s, AssetStorage<FontAsset>>,
    ) -> FontHandle {
        loader.load("font/meslo.ttf", TtfFormat, (), (), storage)
    }
}
