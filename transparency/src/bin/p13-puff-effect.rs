use std::{cell::RefCell, rc::Rc};
use ux::prelude::*;
use ux::{Surface, TextStyle, TextWeight, Window};

struct Glob {
    timer: bool,
    alpha: f64,
    size: f64,
}

#[derive(Default, Application)]
struct Application {
    window: Window,
}

impl Application {
    fn new() -> Self {
        let app: Self = Default::default();
        app.window
            .set_window_size(512, 512)
            .set_title("UX Framework - Puff effect")
            .show()
            .connect_destroy(move |_win| Application::quit());

        app.window.set_background_color(Some(color::GRAY_9));

        app.window.connect_activate(move |window| {

            let surface = Surface::new();
            surface.set_size(512.0, 512.0);

            // we should also change surface content size to avoid distortion
            surface.set_content_size(512.0, 512.0);

            window.set_child(&surface);

            let glob = Rc::new(RefCell::new(Glob {
                timer: true,
                alpha: 1.0,
                size: 1.0,
            }));

            let state = glob.clone();

            surface.connect_draw(move |_widget, ctx, width, height| {
                ctx.clear_rect(0.0, 0.0, width as f64, height as f64);

                let mut state = state.borrow_mut();
                state.size += 0.8;

                if state.size > 20.0 {
                    state.alpha -= 0.01;
                }

                ctx.set_font("Roboto", TextStyle::Normal, TextWeight::Bold, state.size);

                let extents = ctx.measure_text("UX Framework");

                let x = (width as f64 - extents.width) / 2.0;
                let y = height as f64 / 2.0;

                let opacity = (state.alpha * 255.0) as u8;

                ctx.set_fill_color(color::WHITE.opacity(opacity));
                ctx.fill_text("UX Framework", x, y);

                if state.alpha <= 0.0 {
                    state.timer = false;
                }

                false
            });

            
            let state = glob;
            // 1000/50fps = 20
            ux::interval(20, move || {
                // We should scope state to prevent dowble borrow
                // This is a standard pattern for Rust
                {
                    let state = state.borrow();
                    if !state.timer {
                        return Continue(false);
                    }
                }
                surface.invalidate();

                Continue(true)
            });
        });

        app
    }
}

fn main() {
    Application::run();
}
