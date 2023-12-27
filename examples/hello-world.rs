use std::error::Error;
use std::num::NonZeroU32;

use glam::vec2;
use glutin::surface::GlSurface;
use loki_draw::drawer::{Drawer, RectBlueprint, TextBlueprint};
use loki_draw::font::Font;
use loki_draw::rect::Rect;
use loki_draw::OpenglDrawer;
use opengl::{create_opengl_window, OpenglCtx};
use winit::event::{ElementState, Event, KeyEvent, WindowEvent};
use winit::event_loop::ControlFlow;
use winit::keyboard::{KeyCode, PhysicalKey};

const ROBOTO_FONT: &[u8] = include_bytes!("common/Roboto-Regular.ttf");

#[path = "common/opengl.rs"]
mod opengl;

fn main() -> Result<(), Box<dyn Error>> {
    let (width, height) = (1280, 720);

    let OpenglCtx {
        gl_ctx,
        gl_surface,
        gl_display,
        events,
        window,
    } = create_opengl_window(width, height)?;

    let default_font = Font::from_data(ROBOTO_FONT);

    let mut drawer = OpenglDrawer::new(width, height, 1.);
    let mut viewport = vec2(width as f32, height as f32);

    // Event loop
    events.run(move |event, elwt| {
        // They need to be present
        let _gl_display = &gl_display;
        let _window = &window;

        elwt.set_control_flow(ControlFlow::Wait);

        match event {
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::RedrawRequested => {
                    drawer.clear();

                    drawer.begin_frame();
                    drawer.draw_rect(&RectBlueprint {
                        rect: Rect {
                            x: viewport.x / 2. - 200.,
                            y: viewport.y / 2. - 200.,
                            w: 400.,
                            h: 400.,
                        },
                        color: 0x2a2939,
                        border_color: 0xff84c6,
                        border_width: 4.,
                        corner_radius: 10.,
                        borders: [true, true, true, true],
                        alpha: 1.,
                    });
                    drawer.draw_text(&TextBlueprint {
                        text: "Hello world!",
                        x: 20.,
                        y: viewport.y / 2. - 300.,
                        font: &default_font,
                        size: 100.,
                        col: 0xffffff,
                        alpha: 1.,
                    });
                    drawer.end_frame();

                    gl_surface.swap_buffers(&gl_ctx).unwrap();
                    window.request_redraw();
                }
                WindowEvent::Resized(physical_size) => {
                    // Handle window resizing
                    viewport = vec2(physical_size.width as f32, physical_size.height as f32);
                    drawer.resize(viewport, 1.);

                    gl_surface.resize(
                        &gl_ctx,
                        NonZeroU32::new(physical_size.width).unwrap(),
                        NonZeroU32::new(physical_size.height).unwrap(),
                    );
                    window.request_redraw();
                }
                WindowEvent::CloseRequested => elwt.exit(),
                WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            physical_key: PhysicalKey::Code(KeyCode::Escape),
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                } => elwt.exit(),
                _ => (),
            },
            Event::AboutToWait => {
                window.request_redraw();
            }
            _ => (),
        }
    })?;

    Ok(())
}
