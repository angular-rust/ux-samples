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
            .set_title("UX Framework - Shear")
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

            ctx.set_fill_color(color::TEAL_9); // Fill color
            ctx.begin_path();
            ctx.rect(20.0, 30.0, 80.0, 50.0);
            ctx.fill();
        
            ctx.transform(1.0, 0.5, 0.0, 1.0, 0.0, 0.0);
            ctx.begin_path();
            ctx.rect(130.0, 30.0, 80.0, 50.0);
            ctx.fill();

            false
        });

        app
    }
}

fn main() {
    Application::run();
}
