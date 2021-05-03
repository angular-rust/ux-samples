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
        surface.set_size(400.0, 400.0);

        // we should also change surface content size to avoid distortion
        surface.set_content_size(400.0, 400.0);
        surface.set_position(56.0, 56.0);

        app.window.set_child(&surface);

        surface.connect_draw(move |_widget, ctx, width, height| {
            ctx.clear_rect(0.0, 0.0, width as f64, height as f64);

            let config: RoughConfig = Default::default();
            let ctx = RoughCanvas::new(ctx, config);

            ctx.set_stroke_color(color::TEAL_9);

            ctx.arc(150.0, 100.0, 90.0, PI, PI * 1.6, false);
            ctx.arc(150.0, 200.0, 90.0, PI, PI * 1.6, false);
            ctx.arc(150.0, 200.0, 90.0, 0.0, PI / 2.0, false);

            // ctx.fill();

            // ctx.set_stroke_color(color::RED_9);
            ctx.stroke();
            // {
            // stroke: 'red', strokeWidth: 4,
            // fill: 'rgba(255,255,0,0.4)', fillStyle: 'solid'
            // });
            // rc.arc(350, 300, 200, 180, Math.PI / 2, Math.PI, true, {
            // stroke: 'blue', strokeWidth: 2,
            // fill: 'rgba(255,0,255,0.4)'
            // });

            // canvas.getContext('2d').translate(-210, 0);
            // rc.arc(350, 300, 200, 180, Math.PI, Math.PI * 1.6, true, {
            // fill: 'red', fillStyle: 'zigzag',
            // hachureGap: 10
            // });
            // rc.arc(350, 300, 200, 180, 0, Math.PI / 2, true, {
            // stroke: 'red', strokeWidth: 4,
            // fill: 'orange', fillStyle: 'dots'
            // });
            // rc.arc(350, 300, 200, 180, Math.PI / 2, Math.PI, true, {
            // stroke: 'blue', strokeWidth: 2,
            // fill: 'red', fillStyle: 'cross-hatch'
            // });
            false
        });

        app
    }
}

fn main() {
    Application::run();
}
