use ux::prelude::*;
use ux::{ColorStop, Gradient, GradientType, LinearGradient, Surface, Window};

#[derive(Default, Application)]
struct Application {
    window: Window,
}

impl Application {
    fn new() -> Self {
        let app: Self = Default::default();
        app.window
            .set_window_size(512, 512)
            .set_title("UX Framework - Linear gradients")
            .show()
            .connect_destroy(move |_win| Application::quit());

        app.window.set_background_color(Some(color::TEAL_9));

        let surface = Surface::new();
        surface.set_size(400.0, 400.0);

        // we should also change surface content size to avoid distortion
        surface.set_content_size(400.0, 400.0);
        surface.set_position(56.0, 56.0);

        app.window.set_child(&surface);

        let mut offset = 0.0;
        let mut count = 1;
        let gradient1 = Gradient::new(GradientType::Linear(LinearGradient::new(
            0.0, 0.0, 350.0, 350.0,
        )));

        while offset <= 1.0 {
            if count % 2 == 1 {
                gradient1.add_color_stop(ColorStop::new(offset, color::BLACK));
            } else {
                gradient1.add_color_stop(ColorStop::new(offset, color::RED_9));
            }
            offset += 0.1;
            count += 1;
        }

        let mut offset = 0.0;
        let mut count = 1;
        let gradient2 = Gradient::new(GradientType::Linear(LinearGradient::new(
            0.0, 0.0, 350.0, 0.0,
        )));
        while offset <= 1.0 {
            if count % 2 == 1 {
                gradient2.add_color_stop(ColorStop::new(offset, color::BLACK));
            } else {
                gradient2.add_color_stop(ColorStop::new(offset, color::YELLOW_9));
            }
            offset += 0.05;
            count += 1;
        }

        let gradient3 = Gradient::new(GradientType::Linear(LinearGradient::new(
            20.0, 260.0, 20.0, 360.0,
        )));

        gradient3.add_color_stop(ColorStop::new(0.1, color::BLACK));
        gradient3.add_color_stop(ColorStop::new(0.5, color::BLUE_9));
        gradient3.add_color_stop(ColorStop::new(0.9, color::BLACK));

        surface.connect_draw(move |_widget, ctx, width, height| {
            ctx.clear_rect(0.0, 0.0, width as f64, height as f64);

            ctx.set_stroke_color(color::RED_9); // Stroke color
            ctx.set_line_width(10.0);

            ctx.set_fill_gradient(&gradient1); // Fill color
            ctx.fill_rect(20.0, 20.0, 300.0, 100.0);

            ctx.set_fill_gradient(&gradient2); // Fill color
            ctx.fill_rect(20.0, 140.0, 300.0, 100.0);

            ctx.set_fill_gradient(&gradient3); // Fill color
            ctx.fill_rect(20.0, 260.0, 300.0, 100.0);

            false
        });

        app
    }
}

fn main() {
    Application::run();
}
