#![allow(unused_imports)]
use cairo::{Extend, ImageSurface, SurfacePattern};
use std::{f64::consts::PI, mem};
use ux::prelude::*;
use ux::{
    Actor, ClickAction, ColorStop, Gradient, GradientType, LinearGradient, Pattern, PatternExtend,
    RadialGradient, Surface, Window,
};

#[derive(Default, Application)]
struct Application {
    window: Window,
}

impl Application {
    fn new() -> Self {
        let app: Self = Default::default();
        app.window
            .set_window_size(512, 512)
            .set_title("UX Framework - Simple Graphics")
            .show()
            .connect_destroy(move |_win| Application::quit());

        app.window.set_background_color(Some(color::TEAL_9));

        let surface = Surface::new();
        surface.set_size(400.0, 400.0);

        // we should also change surface content size to avoid distortion
        surface.set_content_size(400.0, 400.0);
        surface.set_position(56.0, 56.0);

        app.window.set_child(&surface);

        let blueweb = include_bytes!("redhand.png");
        let image_surface = ImageSurface::create_from_png(&mut &blueweb[..]).unwrap();
        let pattern = Pattern::new(PatternExtend::Repeat, &image_surface);

        surface.connect_draw(move |_widget, ctx, _, height| {
            ctx.set_fill_color(color::CYAN_9);
            ctx.fill_rect(50.0, 50.0, 250.0, 250.0);
            ctx.clear_rect(200.0, 200.0, 100.0, 100.0);

            let gradient = Gradient::new(GradientType::Linear(LinearGradient::new(
                0.0,
                0.0,
                0.0,
                height as f64,
            )));

            gradient.add_color_stop(ColorStop::new(0.0, color::CYAN_9));
            gradient.add_color_stop(ColorStop::new(0.5, color::LIME_9));
            gradient.add_color_stop(ColorStop::new(1.0, color::CYAN_0.opacity(0)));
            ctx.set_fill_gradient(&gradient);
            ctx.rect(0.0, 0.0, 200.0, height as f64);
            ctx.fill();

            let gradient = Gradient::new(GradientType::Radial(RadialGradient::new(
                250.0,
                0.0,
                10.0,
                250.0,
                0.0,
                height as f64 / 2.0,
            )));
            gradient.add_color_stop(ColorStop::new(0.0, color::INDIGO_0.opacity(128)));
            gradient.add_color_stop(ColorStop::new(1.0, color::ORANGE_9));
            ctx.set_fill_gradient(&gradient);
            ctx.rect(200.0, 0.0, 200.0, height as f64);
            ctx.fill();

            ctx.clear_rect(300.0, 200.0, 200.0, 200.0);

            ctx.save();
            ctx.translate(100.0, 93.0);
            ctx.set_fill_pattern(&pattern);
            ctx.fill_rect(0.0, 0.0, 200.0, 213.0);
            ctx.restore();

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
