pub mod graphics;
pub mod math;
pub mod window;

use image::GenericImageView;
use std::collections::HashMap;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use wgpu::util::DeviceExt;
use winit::{
    event::{self, *},
    event_loop::{self, *},
    window::*,
};

pub trait Game {
    fn init(engine: &mut Engine) -> Self;
    fn update(&mut self, engine: &mut Engine, delta: f32) -> ();
    fn render(&self) -> Vec<(&graphics::Sprite, Vec<graphics::Instance>)>;
}

pub struct Engine {
    pub renderer: graphics::Renderer,
    pub window: window::Window,
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    UnknownFileType { extension: String },
    IO(std::io::Error),
    Image(image::ImageError),
    WGPU(wgpu::SurfaceError),
    NoRenderer,
}

impl Engine {
    pub async fn init() -> Result<(Self, winit::event_loop::EventLoop<()>)> {
        let scale = 4;
        let (window, event_loop) = window::Window::new(240 * scale, 224 * scale);

        #[cfg(target_arch = "wasm32")]
        {
            // Winit prevents sizing with CSS, so we have to set
            // the size manually when on web.
            use winit::dpi::PhysicalSize;
            window.set_inner_size(PhysicalSize::new(450, 400));

            use winit::platform::web::WindowExtWebSys;
            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| {
                    let dst = doc.get_element_by_id("wasm-example")?;
                    let canvas = web_sys::Element::from(window.canvas());
                    dst.append_child(&canvas).ok()?;
                    Some(())
                })
                .expect("Couldn't append canvas to document body.");
        }

        let renderer = graphics::Renderer::new(&window).await;

        Ok((Self { renderer, window }, event_loop))
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run<G>()
where
    G: Sized + Game + 'static,
{
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
    }

    let (mut engine, event_loop) = Engine::init().await.expect("Couldn't load engine");
    let mut game = G::init(&mut engine);

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == engine.window.winit.id() => {
            match event {
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => *control_flow = ControlFlow::Exit,

                WindowEvent::Resized(physical_size) => {
                    engine.renderer.resize(*physical_size);
                }

                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    // new_inner_size is &&mut so we have to dereference it twice
                    engine.renderer.resize(**new_inner_size);
                }

                _ => {
                    let dt = 0.016;
                    game.update(&mut engine, dt);
                    let _ = engine.renderer.render(&game.render());
                }
            }
        }

        Event::RedrawRequested(window_id) if window_id == engine.window.winit.id() => {
            match engine.renderer.render(&game.render()) {
                Ok(_) => {}
                // Reconfigure the surface if lost
                Err(Error::WGPU(wgpu::SurfaceError::Lost)) => {
                    engine.renderer.resize(engine.window.size)
                }
                // The system is out of memory, we should probably quit
                Err(Error::WGPU(wgpu::SurfaceError::OutOfMemory)) => {
                    *control_flow = ControlFlow::Exit
                }
                // All other errors (Outdated, Timeout) should be resolved by the next frame
                Err(e) => eprintln!("{:?}", e),
            }
        }

        Event::MainEventsCleared => {
            // RedrawRequested will only trigger once unless we manually
            // request it.
            engine.window.winit.request_redraw();
        }

        _ => {}
    });
}
