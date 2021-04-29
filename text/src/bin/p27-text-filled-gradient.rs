use ux::{ColorStop, prelude::*};
use ux::{Gradient, GradientType, LinearGradient, Surface, TextStyle, TextWeight, Window};

#[derive(Default, Application)]
struct Application {
    window: Window,
}

impl Application {
    fn new() -> Self {
        let app: Self = Default::default();
        app.window
            .set_window_size(512, 512)
            .set_title("UX Framework - Text filled with gradient")
            .show()
            .connect_destroy(move |_win| Application::quit());

        app.window.set_background_color(Some(color::GRAY_9));

        let surface = Surface::new();
        surface.set_size(512.0, 512.0);

        // we should also change surface content size to avoid distortion
        surface.set_content_size(512.0, 512.0);

        app.window.set_child(&surface);

        const HEIGHT: f64 = 90.0;

        let gradient = Gradient::new(GradientType::Linear(LinearGradient::new(
            0.0,
            15.0,
            0.0,
            HEIGHT * 0.8,
        )));
        gradient.add_color_stop(ColorStop::new(0.0, color::TEAL_9));
        gradient.add_color_stop(ColorStop::new(1.0, color::TEAL_3));

        surface.connect_draw(move |_widget, ctx, width, height| {
            ctx.clear_rect(0.0, 0.0, width as f64, height as f64);

            ctx.set_font("Roboto", TextStyle::Normal, TextWeight::Bold, HEIGHT);

            ctx.set_fill_gradient(&gradient);
            ctx.fill_text("ZetCode", 15.0, 80.0);

            false
        });

        app
    }
}

fn main() {
    Application::run();
}