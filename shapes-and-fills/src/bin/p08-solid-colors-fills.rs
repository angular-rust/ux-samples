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
            .set_title("UX Framework - Solid colours fills")
            .show()
            .connect_destroy(move |_win| Application::quit());

        app.window.set_background_color(Some(color::TEAL_9));

        let surface = Surface::new();
        surface.set_size(400.0, 400.0);

        // we should also change surface content size to avoid distortion
        surface.set_content_size(400.0, 400.0);
        surface.set_position(56.0, 56.0);

        app.window.set_child(&surface);

        surface.connect_draw(move |_widget, ctx, width, height| {
            ctx.clear_rect(0.0, 0.0, width as f64, height as f64);

            ctx.set_stroke_color(color::RED_9); // Stroke color
            ctx.set_line_width(10.0);

            ctx.set_fill_color(color::BLUE_9);
            ctx.begin_path();
            ctx.rect(20.0, 20.0, 100.0, 100.0);
            ctx.fill();

            ctx.set_fill_color(color::GRAY_9);
            ctx.begin_path();
            ctx.rect(150.0, 20.0, 100.0, 100.0);
            ctx.fill();

            ctx.set_fill_color(color::GREEN_9);
            ctx.begin_path();
            ctx.rect(20.0, 140.0, 100.0, 100.0);
            ctx.fill();

            ctx.set_fill_color(color::ORANGE_9);
            ctx.begin_path();
            ctx.rect(150.0, 140.0, 100.0, 100.0);
            ctx.fill();

            false
        });

        app
    }
}

fn main() {
    Application::run();
}
