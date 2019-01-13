//! Bar definitions and functions
mod test;

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

/// The root bar structure
pub struct Bar;

impl Bar {
    /// Create a new Bar instance and run it
    pub fn run(level_filter: LevelFilter, dry_run: bool) -> Fallible<()> {
        // Create the application window
        let display_config =
            Self::setup_engine(level_filter, Self::create_window());
        debug!("Engine setup done");

        // Connect to i3
        let mut i3_conn = I3Connection::connect()?;
        debug!("Found i3 version {}", i3_conn.get_version()?.human_readable);

        // Create the pipeline
        let pipe = Pipeline::build().with_stage(
            Stage::with_backbuffer()
                .clear_target([0.00196, 0.23726, 0.21765, 1.0], 1.0)
                .with_pass(DrawFlat::<PosNormTex>::new()),
        );
        debug!("Pipeline creation done");

        // Setup the application data
        let app_data = GameDataBuilder::default()
            .with_bundle(RenderBundle::new(pipe, Some(display_config)))
            .map_err(|_| err_msg("Unable to create app data"))?;
        debug!("Application data setup done");

        // Create and start the applicaiton
        let bar_state = BarState { i3_conn };
        let mut app = Application::new("./", bar_state, app_data)
            .map_err(|_| err_msg("Unable to create application"))?;
        debug!("Initialization done");

        if dry_run {
            debug!("Dry run specified, everything is okay")
        } else {
            debug!("Starting app");
            app.run();
        }

        Ok(())
    }

    /// Create the window for the application
    fn create_window() -> WindowBuilder {
        WindowBuilder::new()
            .with_dimensions(LogicalSize::new(20.0, 20.0))
            .with_x11_window_type(XWindowType::Dock)
            .with_resizable(false)
            .with_decorations(false)
            .with_always_on_top(true)
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
}
