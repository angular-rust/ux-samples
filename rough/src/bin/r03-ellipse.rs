#![allow(unused_imports)]
use std::f64::consts::PI;
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

            ctx.save();
            ctx.translate(40.0, (512.0 - 70.0) / 2.0);

            ctx.begin_path();
            ctx.set_stroke_color(color::TEAL_9);
            ctx.ellipse(10.0, 10.0, 70.0, 70.0, 0.0, 0.0, PI * 2.0, false);
            ctx.stroke();

            ctx.begin_path();
            ctx.set_stroke_color(color::RED_9);
            ctx.ellipse(90.0, 10.0, 70.0, 70.0, 0.0, 0.0, PI * 2.0, false);
            ctx.fill();

            ctx.begin_path();
            ctx.set_stroke_color(color::PINK_9);
            ctx.ellipse(170.0, 10.0, 70.0, 70.0, 0.0, 0.0, PI * 2.0, false);
            ctx.fill(); //{ fillStyle: 'solid' });

            ctx.begin_path();
            ctx.set_stroke_color(color::YELLOW_9);
            ctx.ellipse(250.0, 10.0, 70.0, 70.0, 0.0, 0.0, PI * 2.0, false);
            ctx.fill(); //{ fillStyle: 'cross-hatch' });

            ctx.begin_path();
            ctx.set_stroke_color(color::INDIGO_9);
            ctx.ellipse(330.0, 10.0, 70.0, 70.0, 0.0, 0.0, PI * 2.0, false);
            ctx.fill(); // { fillStyle: 'zigzag', hachureGap: 8 });

            ctx.begin_path();
            ctx.set_stroke_color(color::ORANGE_9);
            ctx.ellipse(410.0, 10.0, 70.0, 70.0, 0.0, 0.0, PI * 2.0, false);
            ctx.fill(); //{ fillStyle: 'dots' });

            ctx.restore();

            // ctx.circle(10, 110 + 40, 80, 0.0, 0.0, PI * 2.0, false);
            // { roughness: 2 });
            // ctx.circle(110, 110 + 40, 80, 0.0, 0.0, PI * 2.0, false);
            // { fill: 'red', stroke: 'blue', hachureAngle: 0, strokeWidth: 3 });
            // ctx.circle(210, 110 + 40, 80, 0.0, 0.0, PI * 2.0, false);
            // { fill: 'pink', fillWeight: 3, hachureGap: 8, hachureAngle: 45 });

            // ctx.ellipse(300.0, 350.0, 480.0, 280.0, 0.0, 0.0, PI * 2.0, false);
            // { fill: 'red', fillStyle: 'dots', hachureGap: 20, hachureAngle: 0, fillWeight: 2 });

            false
        });

        app
    }
}

fn main() {
    Application::run();
}
