use std::f64::consts::PI;
use ux::prelude::*;
use ux::{ColorStop, Gradient, GradientType, RadialGradient, Surface, Window};

#[derive(Default, Application)]
struct Application {
    window: Window,
}

impl Application {
    fn new() -> Self {
        let app: Self = Default::default();
        app.window
            .set_window_size(512, 512)
            .set_title("UX Framework - Radial gradients")
            .show()
            .connect_destroy(move |_win| Application::quit());

        app.window.set_background_color(Some(color::GRAY_9));

        let surface = Surface::new();
        surface.set_size(400.0, 400.0);

        // we should also change surface content size to avoid distortion
        surface.set_content_size(400.0, 400.0);
        surface.set_position(56.0, 56.0);

        app.window.set_child(&surface);

        let pattern1 = Gradient::new(GradientType::Radial(RadialGradient::new(
            30.0, 30.0, 10.0, 30.0, 30.0, 90.0,
        )));
        pattern1.add_color_stop(ColorStop::new(0.0, color::WHITE));
        pattern1.add_color_stop(ColorStop::new(1.0, color::BLUE_9));

        let pattern2 = Gradient::new(GradientType::Radial(RadialGradient::new(
            0.0, 0.0, 10.0, 0.0, 0.0, 40.0,
        )));
        pattern2.add_color_stop(ColorStop::new(0.0, color::YELLOW_5));
        pattern2.add_color_stop(ColorStop::new(1.0, color::RED_9));

        surface.connect_draw(move |_widget, ctx, width, height| {
            ctx.clear_rect(0.0, 0.0, width as f64, height as f64);

            ctx.begin_path();
            ctx.translate(60.0, 60.0);
            ctx.set_fill_gradient(&pattern1);
            ctx.arc(0.0, 0.0, 40.0, 0.0, 2.0 * PI, false);
            ctx.fill();

            ctx.begin_path();
            ctx.translate(120.0, 0.0);
            ctx.set_fill_gradient(&pattern2);
            ctx.arc(0.0, 0.0, 40.0, 0.0, 2.0 * PI, false);
            ctx.fill();

            false
        });

        app
    }
}

fn main() {
    Application::run();
}
