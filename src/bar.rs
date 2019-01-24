//! Bar definitions and functions

use crate::{bundle::BarBundle, color::ColorScheme, state::BarState};
use amethyst::{
    assets::Processor,
    audio::Source,
    core::{frame_limiter::FrameRateLimitStrategy, transform::TransformBundle},
    input::InputBundle,
    prelude::*,
    renderer::{
        ColorMask, DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage,
        ALPHA,
    },
    ui::{DrawUi, UiBundle},
    utils::{application_root_dir, fps_counter::FPSCounterBundle},
    LoggerConfig, StdoutLog,
};
use failure::{err_msg, Fallible};
use log::{debug, LevelFilter};
use std::path::PathBuf;

/// The root bar structure
pub struct Bar;

impl Bar {
    /// Create a new Bar instance and run it
    pub fn run(level_filter: LevelFilter) -> Fallible<()> {
        // Setup the internal logger
        Self::setup_logging(level_filter);
        debug!("Logger setup done");

        // Setup the application data
        let app_root = PathBuf::from(application_root_dir());
        let resources = app_root.join("assets");
        let display_config_path = resources.join("display.ron");

        let config = DisplayConfig::load(&display_config_path);

        let pipe = Pipeline::build().with_stage(
            Stage::with_backbuffer()
                .clear_target(
                    // Convert SRGB to linear
                    [
                        ColorScheme::background()[0].powf(2.1),
                        ColorScheme::background()[1].powf(2.1),
                        ColorScheme::background()[2].powf(2.1),
                        1.,
                    ],
                    1.,
                )
                .with_pass(DrawFlat2D::new().with_transparency(
                    ColorMask::all(),
                    ALPHA,
                    None,
                ))
                .with_pass(DrawUi::new()),
        );

        let app_data = GameDataBuilder::default()
            .with_bundle(BarBundle)
            .map_err(|_| err_msg("Unable to load BarBundle"))?
            .with_bundle(
                TransformBundle::new()
                    .with_dep(&["workspace_system", "status_system"]),
            )
            .map_err(|_| err_msg("Unable to load TransformBundle"))?
            .with_bundle(UiBundle::<String, String>::new())
            .map_err(|_| err_msg("Unable to load UiBundle"))?
            .with(Processor::<Source>::new(), "source_processor", &[])
            .with_bundle(FPSCounterBundle::default())
            .map_err(|_| err_msg("Unable to load FPSCounterBundle"))?
            .with_bundle(InputBundle::<String, String>::new())
            .map_err(|_| err_msg("Unable to load InputBundle"))?
            .with_bundle(
                RenderBundle::new(pipe, Some(config))
                    .with_sprite_sheet_processor(),
            )
            .map_err(|_| err_msg("Unable to load RenderBundle"))?;
        debug!("Application data setup done");

        // Create and start the applicaiton
        let mut app = Application::build(resources, BarState)
            .map_err(|_| err_msg("Unable to create application builder"))?
            .with_frame_limit(FrameRateLimitStrategy::Unlimited, 9999)
            .build(app_data)
            .map_err(|_| err_msg("Unable to create application"))?;

        debug!("Initialization done, starting app");
        app.run();
        Ok(())
    }

    /// Setup the application logger
    fn setup_logging(level_filter: LevelFilter) {
        // Initialize logging
        let logger_config = LoggerConfig {
            stdout: StdoutLog::Colored,
            level_filter,
            log_file: None,
            allow_env_override: true,
        };
        amethyst::start_logger(logger_config);
    }
}
