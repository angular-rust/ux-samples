use std::f64::consts::PI;
use ux::prelude::*;
use ux::{Surface, Window};

#[derive(Default, Application)]
struct Application {
    window: Window,
}

impl Application {
    fn new() -> Self {
        let app: Self = Default::default();
        app.window
            .set_window_size(512, 512)
            .set_title("UX Framework - Donut")
            .show()
            .connect_destroy(move |_win| Application::quit());

        app.window.set_background_color(Some(color::GRAY_9));

        let surface = Surface::new();
        surface.set_size(400.0, 400.0);

        // we should also change surface content size to avoid distortion
        surface.set_content_size(400.0, 400.0);
        surface.set_position(56.0, 56.0);

        app.window.set_child(&surface);

        surface.connect_draw(move |_widget, ctx, width, height| {
            ctx.clear_rect(0.0, 0.0, width as f64, height as f64);

            ctx.set_stroke_color(color::TEAL_9); // Stroke color

            ctx.set_line_width(0.5);
            ctx.translate(width as f64 / 2.0, height as f64 / 2.0);
            ctx.arc(0.0, 0.0, 120.0, 0.0, 2.0 * PI, false);
            ctx.stroke();

            for idx in 0..36 {
                ctx.save();
                ctx.rotate(idx as f64 * PI / 36.0);
                ctx.scale(0.3, 1.0);
                ctx.arc(0.0, 0.0, 120.0, 0.0, 2.0 * PI, false);
                ctx.restore();
                ctx.stroke();
            }

            false
        });

        app
    }
}

fn main() {
    Application::run();
}
