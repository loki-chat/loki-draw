use std::error::Error;
use std::num::NonZeroU32;

use glam::vec2;
use glutin::surface::GlSurface;
use loki_draw::drawer::{Drawer, RectBlueprint, TextBlueprint};
use loki_draw::rect::Rect;
use loki_draw::text::Text;
use loki_draw::OpenglDrawer;
use opengl::{create_opengl_window, OpenglCtx};
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};

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

    let mut drawer = OpenglDrawer::new(width, height, 1.);
    let mut viewport = vec2(width as f32, height as f32);

    // Event loop
    events.run(move |event, _, control_flow| {
        // They need to be present
        let _gl_display = &gl_display;
        let _window = &window;

        control_flow.set_wait();

        match event {
            Event::RedrawRequested(_) => {
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
                    text: Text::new("The quick brown fox jumps over the lazy dog.").computed(50., true, false, true, false),
                    x: 20.,
                    y: viewport.y / 2. - 200.,
                    col: 0xffffff,
                    alpha: 1.,
                });
                drawer.end_frame();

                gl_surface.swap_buffers(&gl_ctx).unwrap();
                window.request_redraw();
            }
            Event::WindowEvent { ref event, .. } => match event {
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
                WindowEvent::CloseRequested => control_flow.set_exit(),
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                } => control_flow.set_exit(),
                _ => (),
            },
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => (),
        }
    })
}
