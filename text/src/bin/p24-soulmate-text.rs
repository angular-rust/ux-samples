use ux::prelude::*;
use ux::{TextStyle, TextWeight, Surface, Window};

#[derive(Default, Application)]
struct Application {
    window: Window,
}

impl Application {
    fn new() -> Self {
        let app: Self = Default::default();
        app.window
            .set_window_size(512, 512)
            .set_title("UX Framework - Soulmate")
            .show()
            .connect_destroy(move |_win| Application::quit());

        app.window.set_background_color(Some(color::GRAY_9));

        let surface = Surface::new();
        surface.set_size(512.0, 512.0);

        // we should also change surface content size to avoid distortion
        surface.set_content_size(512.0, 512.0);

        app.window.set_child(&surface);

        surface.connect_draw(move |_widget, ctx, width, height| {
            ctx.clear_rect(0.0, 0.0, width as f64, height as f64);

            ctx.set_font("Roboto", TextStyle::Normal, TextWeight::Bold, 18.0);

            ctx.set_fill_color(color::TEAL_9); // Stroke color

            ctx.fill_text("Most relationships seem so transitory", 20.0, 30.0);

            ctx.fill_text("They're all good but not the permanent one", 20.0, 60.0);

            ctx.fill_text("Who doesn't long for someone to hold", 20.0, 120.0);

            ctx.fill_text("Who knows how to love you without being told", 20.0, 150.0);

            ctx.fill_text("Somebody tell me why I'm on my own", 20.0, 180.0);

            ctx.fill_text("If there's a soulmate for everyone", 20.0, 210.0);

            false
        });

        app
    }
}

fn main() {
    Application::run();
}
