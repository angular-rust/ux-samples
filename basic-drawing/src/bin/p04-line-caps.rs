use ux::{LineCap, prelude::*};
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
            .set_title("UX Framework - Line caps")
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

            ctx.set_stroke_color(color::TEAL_9);
            ctx.set_line_width(20.0);

            // Draw lines
            ctx.begin_path();
            ctx.set_line_cap(LineCap::Butt);
            ctx.move_to(30.0, 50.0);
            ctx.line_to(150.0, 50.0);
            ctx.stroke();

            ctx.begin_path();
            ctx.set_line_cap(LineCap::Round);
            ctx.move_to(30.0, 90.0);
            ctx.line_to(150.0, 90.0);
            ctx.stroke();

            ctx.begin_path();
            ctx.set_line_cap(LineCap::Square);
            ctx.move_to(30.0, 130.0);
            ctx.line_to(150.0, 130.0);
            ctx.stroke();

            // Draw guides
            ctx.set_line_cap(LineCap::Butt);
            ctx.set_stroke_color(color::GRAY_3);
            ctx.set_line_width(1.0);

            ctx.begin_path();
            ctx.move_to(30.0, 40.0);
            ctx.line_to(30.0, 140.0);

            ctx.move_to(150.0, 40.0);
            ctx.line_to(150.0, 140.0);

            ctx.move_to(160.0, 40.0);
            ctx.line_to(160.0, 140.0);

            ctx.stroke();

            false
        });

        app
    }
}

fn main() {
    Application::run();
}

