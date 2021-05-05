use ux::{RoughCanvas, RoughConfig, prelude::*};
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
            .set_title("UX Framework - Rough lines")
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

            let config: RoughConfig = Default::default();
            let ctx = RoughCanvas::new(ctx, config);

            ctx.set_fill_color(color::RED_9);
            ctx.begin_path();
            ctx.move_to(10.0, 10.0);
            ctx.line_to(100.0, 10.0);

            ctx.move_to(10.0, 210.0);
            ctx.line_to(500.0, 210.0);

            ctx.move_to(10.0, 20.0);
            ctx.line_to(10.0, 110.0);

            ctx.stroke();

            ctx.set_fill_color(color::BLUE_9);
            ctx.begin_path();
            ctx.move_to(10.0, 10.0);
            ctx.line_to(100.0, 10.0);
            
            ctx.move_to(50.0, 30.0);
            ctx.line_to(200.0, 100.0);

            ctx.set_line_width(5.0);
            ctx.stroke();

            false
        });

        app
    }
}

fn main() {
    Application::run();
}
