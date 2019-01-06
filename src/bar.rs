//! Bar definitions and functions
use backend::{
    self,
    glutin::{ContextBuilder, GlWindow},
    Backend,
};
use failure::{format_err, Fallible};
use gfx_hal::{
    adapter::Adapter,
    format::{AsFormat, Rgba8Srgb},
    pool::CommandPoolCreateFlags,
    pso::{
        DescriptorRangeDesc, DescriptorSetLayoutBinding, DescriptorType,
        ShaderStageFlags,
    },
    queue::family::QueueGroup,
    window::Extent2D,
    CommandPool, DescriptorPool, Device, Graphics, Instance, Surface,
};
use log::{debug, info};
use winit::{dpi::LogicalSize, EventsLoop, WindowBuilder};

/// The root bar structure
pub struct Bar {
    dimensions: Extent2D,
    running: bool,
    events_loop: EventsLoop,
    surface: backend::Surface,
    adapter: Adapter<Backend>,
    device: backend::Device,
    queue_group: QueueGroup<Backend, Graphics>,
    command_pool: CommandPool<backend::Backend, Graphics>,
    set_layout: Vec<DescriptorSetLayoutBinding>,
}

impl Bar {
    /// Create a new Bar instance
    pub fn new(width: u32, height: u32) -> Fallible<Self> {
        // Setup the events loop
        debug!("Creating event loop");
        let events_loop = EventsLoop::new();

        // Create the window
        debug!("Creating rendering surface");
        let surface = Self::create_surface(&events_loop, width, height)?;

        // Create the adapter
        debug!("Getting surface adapter");
        let adapter = Self::get_adapter(&surface)?;

        // Build a new device, queue group and command pool
        debug!("Creating device");
        let (device, queue_group, command_pool) =
            Self::create_device(&surface, &adapter)?;

        // Setup the layout
        debug!("Creating layout");
        let set_layout = Self::create_layout(&device)?;

        debug!("Bar initialization done");
        Ok(Bar {
            dimensions: Extent2D { width, height },
            running: false,
            events_loop,
            surface,
            adapter,
            device,
            queue_group,
            command_pool,
            set_layout,
        })
    }

    /// Run the render loop
    pub fn run(&mut self) -> Fallible<()> {
        self.running = true;

        // Create descriptor pool and set
        let mut desc_pool = unsafe {
            self.device.create_descriptor_pool(
                1, // sets
                &[DescriptorRangeDesc {
                    ty: DescriptorType::Sampler,
                    count: 1,
                }],
            )
        }?;
        let desc_set = unsafe { desc_pool.allocate_set(&self.set_layout) }?;

        // Run the event loop
        let mut resize_dims = Extent2D {
            width: 0,
            height: 0,
        };
        while self.running {
            let mut is_running = true;
            let mut update_dims = false;
            let mut new_dims = LogicalSize::new(0f64, 0f64);

            self.events_loop.poll_events(|event| {
                if let winit::Event::WindowEvent { event, .. } = event {
                    match event {
                        winit::WindowEvent::KeyboardInput {
                            input:
                                winit::KeyboardInput {
                                    virtual_keycode:
                                        Some(winit::VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        }
                        | winit::WindowEvent::CloseRequested => {
                            debug!("Got windo close request, exiting.");
                            is_running = false;
                        }
                        winit::WindowEvent::Resized(dims) => {
                            debug!("Window resized to {:?}", dims);
                            new_dims = dims;
                            update_dims = true;
                            resize_dims.width = dims.width as u32;
                            resize_dims.height = dims.height as u32;
                        }
                        _ => (),
                    }
                }
            });

            // Update the window dimensions
            if update_dims {
                debug!("Updating window dimensions");
                self.surface.get_window().resize(
                    new_dims.to_physical(
                        self.surface.get_window().get_hidpi_factor(),
                    ),
                );
            }

            // Update the run status
            self.running = is_running;
        }

        Ok(())
    }

    /// Create the renderable surface from the given dimensions and attach it
    /// to the provided events loop
    fn create_surface(
        events_loop: &EventsLoop,
        width: u32,
        height: u32,
    ) -> Fallible<backend::Surface> {
        let window_builder = WindowBuilder::new()
            .with_dimensions(LogicalSize::new(width as _, height as _))
            .with_title("bar".to_owned());
        let config_context = backend::config_context(
            ContextBuilder::new(),
            Rgba8Srgb::SELF,
            None,
        )
        .with_vsync(true);
        let window = GlWindow::new(window_builder, config_context, events_loop)
            .map_err(|_| format_err!("Unable to create window"))?;
        let surface = backend::Surface::from_window(window);

        Ok(surface)
    }

    /// Retrieve the adapter from the provided surface
    fn get_adapter(surface: &backend::Surface) -> Fallible<Adapter<Backend>> {
        let mut adapters = surface.enumerate_adapters();
        debug!("Found adapters:");
        for adapter in &adapters {
            debug!("  - {:?}", adapter.info);
        }
        let adapter = adapters
            .pop()
            .ok_or_else(|| format_err!("No adapters found"))?;
        info!("Using adapter: {}", adapter.info.name);
        Ok(adapter)
    }

    /// Create the device, queue group and command pool for the provided surface
    /// and adapter
    fn create_device(
        surface: &backend::Surface,
        adapter: &Adapter<Backend>,
    ) -> Fallible<(
        backend::Device,
        QueueGroup<Backend, Graphics>,
        CommandPool<backend::Backend, Graphics>,
    )> {
        let (device, queue_group) = adapter
            .open_with(1, |family| surface.supports_queue_family(family))?;
        let command_pool = unsafe {
            device.create_command_pool_typed(
                &queue_group,
                CommandPoolCreateFlags::empty(),
            )
        }?;
        Ok((device, queue_group, command_pool))
    }

    /// Create the descriptor set layout for the given device
    fn create_layout(
        device: &backend::Device,
    ) -> Fallible<Vec<DescriptorSetLayoutBinding>> {
        Ok(unsafe {
            device.create_descriptor_set_layout(
                &[DescriptorSetLayoutBinding {
                    binding: 1,
                    ty: DescriptorType::Sampler,
                    count: 1,
                    stage_flags: ShaderStageFlags::FRAGMENT,
                    immutable_samplers: false,
                }],
                &[],
            )
        }?)
    }
}
