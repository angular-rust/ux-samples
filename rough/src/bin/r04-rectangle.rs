#![allow(unused_imports)]
use ux::prelude::*;
use ux::{RoughCanvas, RoughConfig, Surface, Window};

#[derive(Default, Application)]
struct Application {
    window: Window,
}

impl Application {
    fn new() -> Self {
        let app: Self = Default::default();
        app.window
            .set_window_size(512, 512)
            .set_title("UX Framework - Transparent rectangles")
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

            let config: RoughConfig = Default::default();
            let ctx = RoughCanvas::new(ctx, config);

            ctx.begin_path();
            ctx.set_fill_color(color::TEAL_9);
            ctx.rect(10.0, 10.0, 80.0, 80.0);
            ctx.fill();

            ctx.begin_path();
            ctx.set_fill_color(color::RED_9);
            ctx.rect(110.0, 10.0, 80.0, 80.0);
            ctx.fill();

            ctx.begin_path();
            ctx.set_fill_color(color::GRAPE_9);
            ctx.rect(210.0, 10.0, 80.0, 80.0); //, { fillStyle: 'solid' });
            ctx.fill();

            ctx.begin_path();
            ctx.set_fill_color(color::GREEN_9);
            ctx.rect(310.0, 10.0, 80.0, 80.0); //, { fillStyle: 'cross-hatch' });
            ctx.fill();

            ctx.begin_path();
            ctx.set_fill_color(color::ORANGE_9);
            ctx.rect(410.0, 10.0, 80.0, 80.0); //, { fillStyle: 'dots' });
            ctx.fill();

            // second row
            ctx.begin_path();
            ctx.set_fill_color(color::CYAN_9);
            ctx.rect(10.0, 110.0, 80.0, 80.0); //, { roughness: 2 });
            ctx.fill();

            ctx.begin_path();
            ctx.set_fill_color(color::INDIGO_9);
            ctx.rect(110.0, 110.0, 80.0, 80.0); //, { stroke: 'blue', hachureAngle: 0, strokeWidth: 3 });
            ctx.fill();

            ctx.begin_path();
            ctx.set_fill_color(color::PINK_9);
            ctx.rect(210.0, 110.0, 80.0, 80.0); //, { fillWeight: 5, hachureGap: 10, hachureAngle: 90 });
            ctx.fill();

            ctx.begin_path();
            ctx.set_fill_color(color::VIOLET_9);
            ctx.rect(310.0, 110.0, 80.0, 80.0); //, { fillStyle: 'zigzag', hachureGap: 8 });
            ctx.fill();

            ctx.begin_path();
            ctx.set_fill_color(color::YELLOW_9);
            ctx.rect(410.0, 110.0, 80.0, 80.0); //, { fillStyle: 'dots' });
            ctx.fill();

            // Bottom
            ctx.begin_path();
            ctx.set_fill_color(color::LIME_9);
            ctx.rect(10.0, 210.0, 480.0, 280.0); //, { fillStyle: 'dots', hachureGap: 20, fillWeight: 2 });
            ctx.fill();

            false
        });

        app
    }
}

fn main() {
    Application::run();
}
