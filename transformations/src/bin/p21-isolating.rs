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
            .set_title("UX Framework - Isolating transformations")
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
            ctx.rect(10.0, 10.0, 90.0, 90.0);
            ctx.fill();
    
            ctx.save();
            ctx.scale(0.6, 0.6);
            ctx.set_fill_color(color::INDIGO_9); // Fill color
            ctx.begin_path();
            ctx.rect(30.0, 30.0, 90.0, 90.0);
            ctx.fill();
            ctx.restore();
    
            ctx.save();
            ctx.scale(0.8, 0.8);
            ctx.set_fill_color(color::ORANGE_9); // Fill color
            ctx.begin_path();
            ctx.rect(50.0, 50.0, 90.0, 90.0);
            ctx.fill();
            ctx.restore();

            false
        });

        app
    }
}

fn main() {
    Application::run();
}