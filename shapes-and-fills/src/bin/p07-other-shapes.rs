use ux::prelude::*;
use ux::{Point, Surface, Window};

#[derive(Default, Application)]
struct Application {
    window: Window,
}

impl Application {
    fn new() -> Self {
        let app: Self = Default::default();
        app.window
            .set_window_size(512, 512)
            .set_title("UX Framework - Other shapes")
            .show()
            .connect_destroy(move |_win| Application::quit());

        app.window.set_background_color(Some(color::TEAL_9));

        let surface = Surface::new();
        surface.set_size(400.0, 400.0);

        // we should also change surface content size to avoid distortion
        surface.set_content_size(400.0, 400.0);
        surface.set_position(56.0, 56.0);

        app.window.set_child(&surface);

        let points = vec![
            Point::new(0.0, 85.0),
            Point::new(75.0, 75.0),
            Point::new(100.0, 10.0),
            Point::new(125.0, 75.0),
            Point::new(200.0, 85.0),
            Point::new(150.0, 125.0),
            Point::new(160.0, 190.0),
            Point::new(100.0, 150.0),
            Point::new(40.0, 190.0),
            Point::new(50.0, 125.0),
            Point::new(0.0, 85.0),
        ];

        surface.connect_draw(move |_widget, ctx, width, height| {
            ctx.clear_rect(0.0, 0.0, width as f64, height as f64);

            ctx.set_stroke_color(color::RED_9); // Stroke color
            ctx.set_line_width(1.0);
    
            ctx.begin_path();
            for point in points.iter() {
                ctx.line_to(point.x, point.y);
            }
    
            ctx.close_path();
            ctx.stroke();
            ctx.fill();
    
            ctx.begin_path();
            ctx.move_to(240.0, 40.0);
            ctx.line_to(240.0, 160.0);
            ctx.line_to(350.0, 160.0);
            ctx.close_path();
    
            ctx.stroke();
            ctx.fill();
    
            ctx.begin_path();
            ctx.move_to(380.0, 40.0);
            ctx.line_to(380.0, 160.0);
            ctx.line_to(480.0, 160.0);
            ctx.bezier_curve_to(470.0, 155.0, 390.0, 145.0, 380.0, 40.0);
    
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
