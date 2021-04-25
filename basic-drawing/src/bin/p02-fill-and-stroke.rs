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
            .set_title("UX Framework - Fill and stroke")
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

            ctx.set_line_width(14.0);
            ctx.set_stroke_color(color::YELLOW_9);
            ctx.translate(width as f64 / 2.0, height as f64 / 2.0);
            ctx.arc(0.0, 0.0, 50.0, 0.0, 2.0 * PI, false);
            ctx.stroke();

            ctx.set_fill_color(color::RED_9);
            ctx.fill();

            false
        });

        app
    }
}

fn main() {
    Application::run();
}
