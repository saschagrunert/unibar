//! Global state handling for the bar

use crate::{
    color::ColorScheme,
    segment::{Cpu, Date, Segment},
};
use amethyst::{
    assets::{AssetStorage, Loader},
    core::Transform,
    ecs::World,
    input::{is_close_requested, is_key_down},
    prelude::{
        Builder, GameData, SimpleState, SimpleTrans, StateData, StateEvent,
        Trans,
    },
    renderer::{
        Camera, PngFormat, Projection, Texture, TextureHandle, TextureMetadata,
    },
    ui::{Anchor, FontHandle, TtfFormat, UiButtonBuilder, UiTransform},
    winit::VirtualKeyCode,
};

/// The state representation of the bar
pub struct State;

impl State {
    fn init_date_segment(&self, world: &mut World) {
        // Create a new date object and add it to the world
        let date = Date::new();
        self.init_button_segment(world, &date, -100, 190);
        world.add_resource(date);

        // Add a separator
        self.add_separator(world, -205.);
    }

    fn init_cpu_segment(&self, world: &mut World) {
        // Create a new cpu object and add it to the world
        let cpu = Cpu::new();
        self.init_button_segment(world, &cpu, -265, 100);
        world.add_resource(cpu);

        // Add a separator
        self.add_separator(world, -325.);
    }

    fn init_button_segment<T>(
        &self,
        world: &mut World,
        segment: &T,
        x: i16,
        width: u16,
    ) where
        T: Segment,
    {
        let builder: UiButtonBuilder<u8> =
            UiButtonBuilder::new(segment.id(), "");
        builder
            .with_anchor(Anchor::TopRight)
            .with_font(self.load_font(world))
            .with_font_size(Self::font_size())
            .with_image(self.load_texture("images/background.png", world))
            .with_position(f32::from(x), -10.)
            .with_size(f32::from(width), 20.)
            .with_text_color(ColorScheme::foreground())
            .build_from_world(world);
    }

    fn add_separator(&self, world: &mut World, x: f32) {
        // Load the image
        let image = self.load_texture("images/separator.png", world);

        // Build the transform
        let transform = UiTransform::new(
            "separator".to_string(),
            Anchor::TopRight,
            x,
            -10.,
            1.,
            10.,
            20.,
        );

        // Add the entity to the world
        world.create_entity().with(transform).with(image).build();
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

    pub fn load_texture<N>(&self, name: N, world: &World) -> TextureHandle
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

    pub fn load_font(&self, world: &mut World) -> FontHandle {
        world.read_resource::<Loader>().load(
            "font/meslo.ttf",
            TtfFormat,
            (),
            (),
            &world.read_resource(),
        )
    }

    pub const fn font_size() -> f32 {
        14.
    }
}

impl SimpleState for State {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // Initialize further components
        self.init_date_segment(world);
        self.init_cpu_segment(world);

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
            _ => Trans::None,
        }
    }
}
