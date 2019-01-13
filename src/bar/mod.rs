//! Bar definitions and functions
use crate::state::BarState;
use amethyst::{
    prelude::*,
    renderer::{
        DisplayConfig, DrawFlat, Pipeline, PosNormTex, RenderBundle, Stage,
    },
    LoggerConfig, StdoutLog,
};
use failure::{err_msg, Fallible};
use i3ipc::I3Connection;
use log::{debug, LevelFilter};
use winit::{
    dpi::LogicalSize,
    os::unix::{WindowBuilderExt, XWindowType},
    WindowBuilder,
};

#[allow(dead_code)]
/// The root bar structure
pub struct Bar {
    display_config: DisplayConfig,
}

impl Bar {
    /// Create a new Bar instance
    pub fn new(level_filter: LevelFilter) -> Fallible<Self> {
        // Create the applicaiton window
        let window_builder = WindowBuilder::new()
            .with_dimensions(LogicalSize::new(20.0, 20.0))
            .with_x11_window_type(XWindowType::Dock)
            .with_resizable(false)
            .with_decorations(false)
            .with_always_on_top(true);

        let display_config = Self::setup_engine(level_filter, window_builder);
        debug!("Engine setup done");

        debug!("Connecting to i3");
        let mut i3ipc = I3Connection::connect()?;
        debug!("Found i3 version {}", i3ipc.get_version()?.human_readable);

        debug!("Initialization done");
        Ok(Bar { display_config })
    }

    /// Setup the application engine
    fn setup_engine(
        level_filter: LevelFilter,
        window_builder: WindowBuilder,
    ) -> DisplayConfig {
        // Initialize logging
        let logger_config = LoggerConfig {
            stdout: StdoutLog::Colored,
            level_filter,
            log_file: None,
            allow_env_override: true,
        };
        amethyst::start_logger(logger_config);

        // Setup the config
        let mut config = DisplayConfig::from(window_builder);
        config.vsync = true;

        config
    }

    /// Run the bar
    pub fn run(&mut self) -> Fallible<()> {
        // Create the pipeline
        let pipe = Pipeline::build().with_stage(
            Stage::with_backbuffer()
                .clear_target([0.00196, 0.23726, 0.21765, 1.0], 1.0)
                .with_pass(DrawFlat::<PosNormTex>::new()),
        );

        // Setup the application data
        let app_data = GameDataBuilder::default()
            .with_bundle(RenderBundle::new(
                pipe,
                Some(self.display_config.clone()),
            ))
            .map_err(|_| err_msg("Unable to create app data"))?;

        // Create and start the applicaiton
        let mut app = Application::new("./", BarState, app_data)
            .map_err(|_| err_msg("Unable to create application"))?;
        app.run();

        Ok(())
    }
}
