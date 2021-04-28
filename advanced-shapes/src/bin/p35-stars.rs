use ux::prelude::*;
use ux::{ClickAction, Surface, Window};

#[derive(Default, Application)]
struct Application {
    window: Window,
}

impl Application {
    fn new() -> Self {
        let app: Self = Default::default();
        app.window
            .set_window_size(512, 512)
            .set_title("UX Framework - Stars")
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

            ctx.begin_path();
            ctx.set_line_width(9.0);
            ctx.star(75.0, 75.0, 5, 50.0, 25.0);
            ctx.set_fill_color(color::TEAL_9);
            ctx.fill();
            ctx.set_stroke_color(color::GRAY_7);
            ctx.stroke();

            ctx.begin_path();
            ctx.set_line_width(3.0);
            ctx.star(150.0, 200.0, 8, 50.0, 25.0);
            ctx.set_fill_color(color::CYAN_6);
            ctx.fill();
            ctx.set_stroke_color(color::GRAY_7);
            ctx.stroke();

            ctx.begin_path();
            ctx.set_line_width(0.0);
            ctx.star(225.0, 75.0, 16, 50.0, 20.0);
            ctx.set_fill_color(color::RED_5);
            ctx.fill();

            ctx.begin_path();
            ctx.set_line_width(3.0);
            ctx.star(300.0, 200.0, 16, 50.0, 40.0);
            ctx.set_fill_color(color::YELLOW_5);
            ctx.fill();
            ctx.set_stroke_color(color::GRAY_7);
            ctx.stroke();

            false
        });

        let action = ClickAction::new();
        surface.add_action(&action);
        surface.set_reactive(true);

        action.connect_clicked(|action, actor| {
            let (x, y) = action.get_coords();
            let (ax, ay) = actor.get_position();
            println!("Clicked {}, {}: {}", action.get_button(), x - ax, y - ay);
        });

        app
    }
}

fn main() {
    Application::run();
}
