use std::{cell::RefCell, f64::consts::PI, rc::Rc};
use ux::prelude::*;
use ux::{LineCap, Surface, Window};

struct Glob {
    count: usize,
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
            .set_title("UX Framework - Waiting demo")
            .show()
            .connect_destroy(move |_win| Application::quit());

        app.window.set_background_color(Some(color::GRAY_9));

        app.window.connect_activate(move |window| {
            let surface = Surface::new();
            surface.set_size(512.0, 512.0);

            // we should also change surface content size to avoid distortion
            surface.set_content_size(512.0, 512.0);

            window.set_child(&surface);

            let glob = Rc::new(RefCell::new(Glob { count: 0 }));

            const LEN: usize = 8;
            let trs: [[f64; LEN]; LEN] = [
                [0.0, 0.15, 0.30, 0.5, 0.65, 0.80, 0.9, 1.0],
                [1.0, 0.0, 0.15, 0.30, 0.5, 0.65, 0.8, 0.9],
                [0.9, 1.0, 0.0, 0.15, 0.3, 0.5, 0.65, 0.8],
                [0.8, 0.9, 1.0, 0.0, 0.15, 0.3, 0.5, 0.65],
                [0.65, 0.8, 0.9, 1.0, 0.0, 0.15, 0.3, 0.5],
                [0.5, 0.65, 0.8, 0.9, 1.0, 0.0, 0.15, 0.3],
                [0.3, 0.5, 0.65, 0.8, 0.9, 1.0, 0.0, 0.15],
                [0.15, 0.3, 0.5, 0.65, 0.8, 0.9, 1.0, 0.0],
            ];

            let state = glob.clone();

            surface.connect_draw(move |_widget, ctx, width, height| {
                ctx.clear_rect(0.0, 0.0, width as f64, height as f64);

                ctx.translate(width as f64 / 2.0, height as f64 / 2.0);

                let state = state.borrow();

                ctx.set_line_width(4.0);
                ctx.set_line_cap(LineCap::Round);

                for idx in 0..LEN {
                    ctx.begin_path();
                    let alpha = trs[state.count % 8][idx];
                    let opacity = (255.0 * alpha) as u8;
                    ctx.set_stroke_color(color::WHITE.opacity(opacity));

                    ctx.move_to(0.0, -15.0);
                    ctx.line_to(0.0, -30.0);
                    ctx.rotate(PI / 4.0);
                    ctx.stroke();
                }

                false
            });

            let state = glob;
            ux::interval(150, move || {
                // We should scope state to prevent dowble borrow
                // This is a standard pattern for Rust
                {
                    let mut state = state.borrow_mut();
                    state.count += 1;
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
