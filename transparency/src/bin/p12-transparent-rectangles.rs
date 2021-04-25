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
            .set_title("UX Framework - Transparent rectangles")
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

            let step = 255.0 / 10.0;
            for idx in 0..10 {
                let opacity = (idx as f64 * step ) as u8;
                ctx.set_fill_color(color::BLUE_9.opacity( opacity)); // Fill color
                ctx.fill_rect(50.0 * idx as f64, 20.0, 40.0, 40.0);
            }

            false
        });

        app
    }
}

fn main() {
    Application::run();
}

