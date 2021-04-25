use std::{f64::consts::PI};
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
            .set_title("UX Framework - Basic shapes")
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
            ctx.set_line_width(1.0);
    
            ctx.begin_path();
            ctx.rect(20.0, 20.0, 120.0, 80.0);
            ctx.rect(180.0, 20.0, 80.0, 80.0);
            ctx.stroke();
            ctx.fill();
    
            ctx.begin_path();
            ctx.arc(330.0, 60.0, 40.0, 0.0, 2.0 * PI, false);
            ctx.stroke();
            ctx.fill();
    
            ctx.begin_path();
            ctx.arc(90.0, 160.0, 40.0, PI / 4.0, PI, false);
            ctx.close_path();
            ctx.stroke();
            ctx.fill();
    
            ctx.begin_path();
            ctx.translate(220.0, 180.0);
            ctx.arc(0.0, 0.0, 50.0, 0.0, 2.0 * PI, false);
            ctx.close_path();
            ctx.stroke();
            ctx.fill();

            false
        });

        app
    }
}

fn main() {
    Application::run();
}

