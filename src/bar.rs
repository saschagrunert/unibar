//! Bar definitions and functions
use crate::{fragment, vertex};
use failure::{format_err, Fallible};
use log::debug;
use std::sync::Arc;
use vulkano::{
    buffer::{BufferUsage, CpuAccessibleBuffer},
    command_buffer::{AutoCommandBufferBuilder, DynamicState},
    device::{Device, DeviceExtensions, Queue},
    framebuffer::{
        Framebuffer, FramebufferAbstract, RenderPassAbstract, Subpass,
    },
    image::SwapchainImage,
    instance::{Instance, PhysicalDevice},
    pipeline::{viewport::Viewport, GraphicsPipeline},
    swapchain::{
        self, AcquireError, PresentMode, Surface, SurfaceTransform, Swapchain,
        SwapchainCreationError,
    },
    sync::{self, FlushError, GpuFuture},
};
use vulkano_win::VkSurfaceBuild;
use winit::{
    dpi::LogicalSize, Event, EventsLoop, Window, WindowBuilder, WindowEvent,
};

#[derive(Debug, Clone)]
struct Vertex {
    position: [f32; 2],
}

/// The root bar structure
pub struct Bar {
    events_loop: EventsLoop,
    surface: Arc<Surface<Window>>,
    device: Arc<Device>,
    queue: Arc<Queue>,
    swapchain: Arc<Swapchain<Window>>,
    images: Vec<Arc<SwapchainImage<Window>>>,
    vertex_buffer: Arc<CpuAccessibleBuffer<[Vertex]>>,
    vs: vertex::Shader,
    fs: fragment::Shader,
}

impl Bar {
    /// Create a new Bar instance
    pub fn new() -> Fallible<Self> {
        debug!("Creating vulkan instance");
        let instance = Self::create_instance()?;

        debug!("Retrieving physical device");
        let physical = Self::get_physical_device(&instance)?;

        debug!("Creating events loop, surface and window");
        let events_loop = EventsLoop::new();
        let surface = WindowBuilder::new()
            .with_dimensions(LogicalSize::new(200.0, 200.0))
            .with_resizable(false)
            .with_decorations(false)
            .with_always_on_top(true)
            .build_vk_surface(&events_loop, instance.clone())?;

        debug!("Setting up device and queue");
        let (device, queue) = Self::create_device(&surface, physical)?;

        debug!("Creating swapchain");
        let (swapchain, images) =
            Self::setup_swapchain(&surface, physical, &device, &queue)?;

        debug!("Creating vertex buffer and shaders");
        let (vertex_buffer, vs, fs) = Self::create_buffer_and_shaders(&device)?;

        debug!("Bar initialization done");
        Ok(Bar {
            events_loop,
            surface,
            device,
            queue,
            swapchain,
            images,
            vertex_buffer,
            fs,
            vs,
        })
    }

    /// Create a vulkan instance
    fn create_instance() -> Fallible<Arc<Instance>> {
        let extensions = vulkano_win::required_extensions();
        Ok(Instance::new(None, &extensions, None)?)
    }

    /// Choose which physical device to use.
    fn get_physical_device<'a>(
        instance: &'a Arc<Instance>,
    ) -> Fallible<PhysicalDevice<'a>> {
        let physical = PhysicalDevice::enumerate(&instance)
            .next()
            .ok_or(format_err!("No valid physical device found"))?;

        debug!(
            "Using physical device: {} (type: {:?})",
            physical.name(),
            physical.ty()
        );
        Ok(physical)
    }

    /// Create a device and choose a corresponding queue
    fn create_device<'a>(
        surface: &Arc<Surface<Window>>,
        physical: PhysicalDevice<'a>,
    ) -> Fallible<(Arc<Device>, Arc<Queue>)> {
        let queue_family = physical
            .queue_families()
            .find(|&q| {
                q.supports_graphics()
                    && surface.is_supported(q).unwrap_or(false)
            })
            .ok_or(format_err!("Unable to create queue family"))?;
        let (device, mut queues) = Device::new(
            physical,
            physical.supported_features(),
            &DeviceExtensions {
                khr_swapchain: true,
                ..DeviceExtensions::none()
            },
            [(queue_family, 0.5)].iter().cloned(),
        )?;
        let queue = queues.next().ok_or(format_err!("No valid queue found"))?;
        Ok((device, queue))
    }

    // Creating a swapchain allocates the color buffers that will contain the
    // image that will ultimately be visible on the screen. These images are
    // returned alongside with the swapchain.
    fn setup_swapchain<'a>(
        surface: &Arc<Surface<Window>>,
        physical: PhysicalDevice<'a>,
        device: &Arc<Device>,
        queue: &Arc<Queue>,
    ) -> Fallible<(Arc<Swapchain<Window>>, Vec<Arc<SwapchainImage<Window>>>)>
    {
        // Querying the capabilities of the surface. When we create the
        // swapchain we can only pass values that are allowed by
        // the capabilities.
        let caps = surface.capabilities(physical)?;

        let usage = caps.supported_usage_flags;

        // The alpha mode indicates how the alpha value of the final image
        // will behave. For example you can choose whether the
        // window will be opaque or transparent.
        let alpha = caps
            .supported_composite_alpha
            .iter()
            .next()
            .ok_or(format_err!("No valid alpha mode found"))?;

        // Choosing the internal format that the images will have.
        let format = caps.supported_formats[0].0;

        // The dimensions of the window, only used to initially setup the
        // swapchain.
        let initial_dimensions =
            if let Some(dimensions) = surface.window().get_inner_size() {
                // convert to physical pixels
                let dimensions: (u32, u32) = dimensions
                    .to_physical(surface.window().get_hidpi_factor())
                    .into();
                [dimensions.0, dimensions.1]
            } else {
                return Err(format_err!("Window no longer exists"));
            };

        // Please take a look at the docs for the meaning of the parameters
        // we didn't mention.
        Ok(Swapchain::new(
            device.clone(),
            surface.clone(),
            caps.min_image_count,
            format,
            initial_dimensions,
            1,
            usage,
            queue,
            SurfaceTransform::Identity,
            alpha,
            PresentMode::Fifo,
            true,
            None,
        )?)
    }

    /// Create all needed buffers and shaders
    fn create_buffer_and_shaders(
        device: &Arc<Device>,
    ) -> Fallible<(
        Arc<CpuAccessibleBuffer<[Vertex]>>,
        vertex::Shader,
        fragment::Shader,
    )> {
        let vertex_buffer = {
            vulkano::impl_vertex!(Vertex, position);

            CpuAccessibleBuffer::from_iter(
                device.clone(),
                BufferUsage::all(),
                [
                    Vertex {
                        position: [-1.0, -0.25],
                    },
                    Vertex {
                        position: [0.0, 0.5],
                    },
                    Vertex {
                        position: [0.25, -0.1],
                    },
                ]
                .iter()
                .cloned(),
            )?
        };
        let vs = crate::vertex::Shader::load(device.clone())?;
        let fs = crate::fragment::Shader::load(device.clone())?;

        Ok((vertex_buffer, vs, fs))
    }

    /// Run the bar
    pub fn run(&mut self) -> Fallible<()> {
        debug!("Creating render pass");
        let render_pass = Arc::new(vulkano::single_pass_renderpass!(
            self.device.clone(),
            attachments: {
                // `color` is a custom name we give to the first and only attachment.
                color: {
                    // `load: Clear` means that we ask the GPU to clear the content of this
                    // attachment at the start of the drawing.
                    load: Clear,
                    // `store: Store` means that we ask the GPU to store the output of the draw
                    // in the actual image. We could also ask it to discard the result.
                    store: Store,
                    // `format: <ty>` indicates the type of the format of the image. This has to
                    // be one of the types of the `vulkano::format` module (or alternatively one
                    // of your structs that implements the `FormatDesc` trait). Here we use the
                    // same format as the swapchain.
                    format: self.swapchain.format(),
                    samples: 1,
                }
            },
            pass: {
                // We use the attachment named `color` as the one and only color attachment.
                color: [color],
                // No depth-stencil attachment is indicated with empty brackets.
                depth_stencil: {}
            }
        )?);

        // Before we draw we have to create what is called a pipeline. This is
        // similar to an OpenGL program, but much more specific.
        let pipeline = Arc::new(
            GraphicsPipeline::start()
        // We need to indicate the layout of the vertices.
        // The type `SingleBufferDefinition` actually contains a template parameter corresponding
        // to the type of each vertex. But in this code it is automatically inferred.
        .vertex_input_single_buffer()
        // A Vulkan shader can in theory contain multiple entry points, so we have to specify
        // which one. The `main` word of `main_entry_point` actually corresponds to the name of
        // the entry point.
        .vertex_shader(self.vs.main_entry_point(), ())
        // The content of the vertex buffer describes a list of triangles.
        .triangle_list()
        // Use a resizable viewport set to draw over the entire window
        .viewports_dynamic_scissors_irrelevant(1)
        // See `vertex_shader`.
        .fragment_shader(self.fs.main_entry_point(), ())
        // We have to indicate which subpass of which render pass this pipeline is going to be used
        // in. The pipeline will only be usable from this particular subpass.
        .render_pass(Subpass::from(render_pass.clone(), 0).ok_or(format_err!("Unable to create subpass"))?)
        // Now that our builder is filled, we call `build()` to obtain an actual pipeline.
        .build(self.device.clone())?,
        );

        // Dynamic viewports allow us to recreate just the viewport when the
        // window is resized Otherwise we would have to recreate the
        // whole pipeline.
        let mut dynamic_state = DynamicState {
            line_width: None,
            viewports: None,
            scissors: None,
        };

        // The render pass we created above only describes the layout of our
        // framebuffers. Before we can draw we also need to create the
        // actual framebuffers.
        //
        // Since we need to draw to multiple images, we are going to create a
        // different framebuffer for each image.
        let mut framebuffers = self.window_size_dependent_setup(
            &self.images,
            render_pass.clone(),
            &mut dynamic_state,
        );

        // Initialization is finally finished!

        // In some situations, the swapchain will become invalid by itself. This
        // includes for example when the window is resized (as the
        // images of the swapchain will no longer match the
        // window's) or, on Android, when the application went to the background
        // and goes back to the foreground.
        //
        // In this situation, acquiring a swapchain image or presenting it will
        // return an error. Rendering to an image of that swapchain will
        // not produce any error, but may or may not work. To continue
        // rendering, we need to recreate the swapchain by creating a new
        // swapchain. Here, we remember that we need to do this for the
        // next loop iteration.
        let mut recreate_swapchain = false;

        // In the loop below we are going to submit commands to the GPU.
        // Submitting a command produces an object that implements the
        // `GpuFuture` trait, which holds the resources for as long as
        // they are in use by the GPU.
        //
        // Destroying the `GpuFuture` blocks until the GPU is finished executing
        // it. In order to avoid that, we store the submission of the
        // previous frame here.
        let mut previous_frame_end =
            Box::new(sync::now(self.device.clone())) as Box<GpuFuture>;

        loop {
            // It is important to call this function from time to time,
            // otherwise resources will keep accumulating and you
            // will eventually reach an out of memory error. Calling
            // this function polls various fences in order to determine what the
            // GPU has already processed, and frees the resources
            // that are no longer needed.
            previous_frame_end.cleanup_finished();

            // Whenever the window resizes we need to recreate everything
            // dependent on the window size. In this example that
            // includes the swapchain, the framebuffers and the dynamic state
            // viewport.
            if recreate_swapchain {
                // Get the new dimensions of the window.
                let dimensions = if let Some(dimensions) =
                    self.surface.window().get_inner_size()
                {
                    let dimensions: (u32, u32) = dimensions
                        .to_physical(self.surface.window().get_hidpi_factor())
                        .into();
                    [dimensions.0, dimensions.1]
                } else {
                    return Ok(());
                };

                let (new_swapchain, new_images) =
                    match self.swapchain.recreate_with_dimension(dimensions) {
                        Ok(r) => r,
                        // This error tends to happen when the user is manually
                        // resizing the window.
                        // Simply restarting the loop is the easiest way to fix
                        // this issue.
                        Err(SwapchainCreationError::UnsupportedDimensions) => {
                            continue;
                        }
                        Err(err) => panic!("{:?}", err),
                    };

                self.swapchain = new_swapchain;
                // Because framebuffers contains an Arc on the old swapchain, we
                // need to recreate framebuffers as well.
                framebuffers = self.window_size_dependent_setup(
                    &new_images,
                    render_pass.clone(),
                    &mut dynamic_state,
                );

                recreate_swapchain = false;
            }

            // Before we can draw on the output, we have to *acquire* an image
            // from the swapchain. If no image is available (which
            // happens if you submit draw commands too quickly), then the
            // function will block.
            // This operation returns the index of the image that we are allowed
            // to draw upon.
            //
            // This function can block if no image is available. The parameter
            // is an optional timeout after which the function call
            // will return an error.
            let (image_num, acquire_future) =
                match swapchain::acquire_next_image(
                    self.swapchain.clone(),
                    None,
                ) {
                    Ok(r) => r,
                    Err(AcquireError::OutOfDate) => {
                        recreate_swapchain = true;
                        continue;
                    }
                    Err(err) => panic!("{:?}", err),
                };

            // Specify the color to clear the framebuffer with
            let clear_values = vec![[0.0, 0.0, 0.0, 0.0].into()];

            // In order to draw, we have to build a *command buffer*. The
            // command buffer object holds the list of commands that
            // are going to be executed.
            //
            // Building a command buffer is an expensive operation (usually a
            // few hundred microseconds), but it is known to be a
            // hot path in the driver and is expected to be
            // optimized.
            let command_buffer =
                AutoCommandBufferBuilder::primary_one_time_submit(self.device.clone(), self.queue.family())?

            // Before we can draw, we have to *enter a render pass*. There are two methods to do
            // this: `draw_inline` and `draw_secondary`. The latter is a bit more advanced and is
            // not covered here.
            //
            // The third parameter builds the list of values to clear the attachments with. The API
            // is similar to the list of attachments when building the framebuffers, except that
            // only the attachments that use `load: Clear` appear in the list.
            .begin_render_pass(framebuffers[image_num].clone(), false, clear_values)?

            // We are now inside the first subpass of the render pass. We add a draw command.
            //
            // The last two parameters contain the list of resources to pass to the shaders.
            // Since we used an `EmptyPipeline` object, the objects have to be `()`.
            .draw(pipeline.clone(), &dynamic_state, self.vertex_buffer.clone(), (), ())?

            // We leave the render pass by calling `draw_end`. Note that if we had multiple
            // subpasses we could have called `next_inline` (or `next_secondary`) to jump to the
            // next subpass.
            .end_render_pass()?

            // Finish building the command buffer by calling `build`.
            .build()?;

            let future = previous_frame_end.join(acquire_future)
            .then_execute(self.queue.clone(), command_buffer)?

            // The color output is now expected to contain our triangle. But in order to show it on
            // the screen, we have to *present* the image by calling `present`.
            .then_swapchain_present(self.queue.clone(), self.swapchain.clone(), image_num)
            .then_signal_fence_and_flush();

            match future {
                Ok(future) => {
                    previous_frame_end = Box::new(future) as Box<_>;
                }
                Err(FlushError::OutOfDate) => {
                    recreate_swapchain = true;
                    previous_frame_end =
                        Box::new(sync::now(self.device.clone())) as Box<_>;
                }
                Err(e) => {
                    println!("{:?}", e);
                    previous_frame_end =
                        Box::new(sync::now(self.device.clone())) as Box<_>;
                }
            }

            // Handling the window events in order to close the program when the
            // user wants to close it.
            let mut done = false;

            self.events_loop.poll_events(|ev| match ev {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => done = true,
                Event::WindowEvent {
                    event: WindowEvent::Resized(_),
                    ..
                } => recreate_swapchain = true,
                _ => (),
            });
            if done {
                return Ok(());
            }
        }
    }

    /// This method is called once during initialization, then again whenever
    /// the window is resized
    fn window_size_dependent_setup(
        &self,
        images: &[Arc<SwapchainImage<Window>>],
        render_pass: Arc<RenderPassAbstract + Send + Sync>,
        dynamic_state: &mut DynamicState,
    ) -> Vec<Arc<FramebufferAbstract + Send + Sync>> {
        let dimensions = images[0].dimensions();

        let viewport = Viewport {
            origin: [0.0, 0.0],
            dimensions: [dimensions[0] as f32, dimensions[1] as f32],
            depth_range: 0.0..1.0,
        };
        dynamic_state.viewports = Some(vec![viewport]);

        images
            .iter()
            .map(|image| {
                Arc::new(
                    Framebuffer::start(render_pass.clone())
                        .add(image.clone())
                        .unwrap()
                        .build()
                        .unwrap(),
                ) as Arc<FramebufferAbstract + Send + Sync>
            })
            .collect::<Vec<_>>()
    }
}
