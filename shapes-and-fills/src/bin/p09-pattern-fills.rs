use cairo::ImageSurface;
use ux::prelude::*;
use ux::{Pattern, PatternExtend, Surface, Window};

#[derive(Default, Application)]
struct Application {
    window: Window,
}

impl Application {
    fn new() -> Self {
        let app: Self = Default::default();
        app.window
            .set_window_size(512, 512)
            .set_title("UX Framework - Patterns")
            .show()
            .connect_destroy(move |_win| Application::quit());

        app.window.set_background_color(Some(color::TEAL_9));

        let surface = Surface::new();
        surface.set_size(400.0, 400.0);

        // we should also change surface content size to avoid distortion
        surface.set_content_size(400.0, 400.0);
        surface.set_position(56.0, 56.0);

        app.window.set_child(&surface);

        let blueweb = include_bytes!("blueweb.png");
        let maple = include_bytes!("maple.png");
        let crack = include_bytes!("crack.png");
        let chocolate = include_bytes!("chocolate.png");

        let surface1 = ImageSurface::create_from_png(&mut &blueweb[..]).unwrap();
        let surface2 = ImageSurface::create_from_png(&mut &maple[..]).unwrap();
        let surface3 = ImageSurface::create_from_png(&mut &crack[..]).unwrap();
        let surface4 = ImageSurface::create_from_png(&mut &chocolate[..]).unwrap();

        let pattern1 = Pattern::new(PatternExtend::Repeat, &surface1);
        let pattern2 = Pattern::new(PatternExtend::Repeat, &surface2);
        let pattern3 = Pattern::new(PatternExtend::Repeat, &surface3);
        let pattern4 = Pattern::new(PatternExtend::Repeat, &surface4);

        surface.connect_draw(move |_widget, ctx, width, height| {
            ctx.clear_rect(0.0, 0.0, width as f64, height as f64);

            ctx.begin_path();
            ctx.set_fill_pattern(&pattern1);
            ctx.rect(20.0, 20.0, 100.0, 100.0);
            ctx.fill();

            ctx.begin_path();
            ctx.set_fill_pattern(&pattern2);
            ctx.rect(150.0, 20.0, 100.0, 100.0);
            ctx.fill();

            ctx.begin_path();
            ctx.set_fill_pattern(&pattern3);
            ctx.rect(20.0, 140.0, 100.0, 100.0);
            ctx.fill();

            ctx.begin_path();
            ctx.set_fill_pattern(&pattern4);
            ctx.rect(150.0, 140.0, 100.0, 100.0);
            ctx.fill();

            false
        });

        app
    }
}

fn main() {
    Application::run();
}
