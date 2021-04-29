use std::cell::RefCell;
use ux::prelude::*;
use ux::{Point, Surface, Window};

struct Glob {
    angle: f64,
    scale: f64,
    delta: f64,
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
            .set_title("UX Framework - Star")
            .show()
            .connect_destroy(move |_win| Application::quit());

        app.window.set_background_color(Some(color::GRAY_9));

        app.window.connect_activate(move |window| {

            let points = vec![
                Point::new(0.0, 85.0),
                Point::new(75.0, 75.0),
                Point::new(100.0, 10.0),
                Point::new(125.0, 75.0),
                Point::new(200.0, 85.0),
                Point::new(150.0, 125.0),
                Point::new(160.0, 190.0),
                Point::new(100.0, 150.0),
                Point::new(40.0, 190.0),
                Point::new(50.0, 125.0),
                Point::new(0.0, 85.0),
            ];
    
            let glob = RefCell::new(Glob {
                angle: 0.0,
                scale: 1.0,
                delta: 0.01,
            });

            let surface = Surface::new();
            surface.set_size(512.0, 512.0);

            // we should also change surface content size to avoid distortion
            surface.set_content_size(512.0, 512.0);

            window.set_child(&surface);
            
            surface.connect_draw(move |_widget, ctx, width, height| {
                ctx.clear_rect(0.0, 0.0, width as f64, height as f64);

                let mut state = glob.borrow_mut();

                ctx.set_fill_color(color::TEAL_9);
                ctx.set_line_width(1.0);

                ctx.translate(width as f64 / 2.0, height as f64 / 2.0);
                ctx.rotate(state.angle);
                ctx.scale(state.scale, state.scale);

                for point in points.iter() {
                    ctx.line_to(point.x, point.y);
                }

                ctx.close_path();
                ctx.fill();
                ctx.stroke();

                if state.scale < 0.01 {
                    state.delta = -state.delta;
                } else if state.scale > 0.99 {
                    state.delta = -state.delta;
                }

                state.scale += state.delta;
                state.angle += 0.01;

                false
            });

            ux::interval(10, move || {
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
