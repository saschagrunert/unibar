//! Global state handling for the bar

use crate::color::ColorScheme;
use amethyst::{
    assets::{AssetStorage, Loader},
    core::Transform,
    ecs::World,
    input::{is_close_requested, is_key_down},
    prelude::*,
    renderer::{
        Camera, PngFormat, Projection, Texture, TextureHandle, TextureMetadata,
    },
    ui::{
        Anchor, FontHandle, TtfFormat, UiButtonBuilder, UiImage, UiText,
        UiTransform,
    },
    winit::VirtualKeyCode,
};
use i3ipc::I3Connection;
use log::warn;

/// The state representation of the bar
pub struct BarState;

impl BarState {
    fn init_workspace_buttons(&self, world: &mut World) {
        let workspaces = ["1", "2", "3", "4", "5"];

        // Load the images
        let image = self.load_texture("images/background.png", world);
        let hover_image = self.load_texture("images/selection.png", world);
        let press_image = self.load_texture("images/foreground.png", world);

        // Load the font
        let font = self.load_font(world);

        // Create the buttons
        for (index, workspace) in workspaces.iter().enumerate() {
            let button = UiButtonBuilder::new(
                format!("ws_button_{}", workspace),
                workspace,
            )
            .with_anchor(Anchor::TopLeft)
            .with_font(font.clone())
            .with_font_size(Self::font_size())
            .with_hover_image(hover_image.clone())
            .with_hover_text_color(ColorScheme::foreground())
            .with_image(image.clone())
            .with_position((20 * index + 10) as f32, -10.)
            .with_press_image(press_image.clone())
            .with_press_text_color(ColorScheme::background())
            .with_size(20., 20.)
            .with_text_color(ColorScheme::foreground())
            .build_from_world(world);

            // Add the entity to the world
            world.add_resource(button);
        }
    }

    fn init_date_text(&self, world: &mut World) {
        // Load the font
        let font = self.load_font(world);

        // Build the transform
        let transform = UiTransform::new(
            "date_time_text".to_string(),
            Anchor::TopRight,
            -100.,
            -10.,
            1.,
            200.,
            20.,
            0,
        );

        // Create the text entity
        let mut text = UiText::new(
            font,
            "".to_owned(),
            ColorScheme::foreground(),
            Self::font_size(),
        );
        text.align = Anchor::MiddleRight;

        // Add the entity to the world
        world.create_entity().with(transform).with(text).build();

        // Add a separator
        self.add_separator(world, -200.);
    }

    fn add_separator(&self, world: &mut World, x: f32) {
        // Load the image
        let image = self.load_texture("images/purple.png", world);

        // Build the transform
        let transform = UiTransform::new(
            "separator".to_string(),
            Anchor::TopRight,
            x,
            -10.,
            1.,
            20.,
            20.,
            0,
        );

        // Add the entity to the world
        world
            .create_entity()
            .with(transform)
            .with(UiImage { texture: image })
            .build();
    }

    fn init_camera(&self, world: &mut World) {
        let mut transform = Transform::default();
        transform.set_z(1.0);
        world
            .create_entity()
            .with(Camera::from(Projection::orthographic(0., 200., 0., 200.)))
            .with(transform)
            .build();
    }

    fn load_texture<N>(&self, name: N, world: &World) -> TextureHandle
    where
        N: Into<String>,
    {
        let loader = world.read_resource::<Loader>();
        loader.load(
            name,
            PngFormat,
            // Screen pixel will be taken from nearest pixel of texture
            TextureMetadata::srgb_scale(),
            (),
            &world.read_resource::<AssetStorage<Texture>>(),
        )
    }

    fn load_font(&self, world: &mut World) -> FontHandle {
        world.read_resource::<Loader>().load(
            "font/meslo.ttf",
            TtfFormat,
            (),
            (),
            &world.read_resource(),
        )
    }

    const fn font_size() -> f32 {
        14.
    }
}

impl SimpleState for BarState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // Add the I3 connection to the world
        world.add_resource(I3Connection::connect());

        // Initialize the workspace component
        self.init_workspace_buttons(world);

        // Initialize further components
        self.init_date_text(world);

        // Initialize the camera
        self.init_camera(world);
    }

    fn handle_event(
        &mut self,
        _: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        match &event {
            StateEvent::Window(event) => {
                if is_close_requested(&event)
                    || is_key_down(&event, VirtualKeyCode::Escape)
                {
                    Trans::Quit
                } else {
                    Trans::None
                }
            }
            StateEvent::Ui(ui_event) => {
                warn!("Got UI event: {:?}", ui_event);
                Trans::None
            }
        }
    }
}
