use std::{cell::RefCell, rc::Rc};
use ux::prelude::*;
use ux::{ClickAction, Point, Surface, Window};

#[derive(Default)]
struct Glob {
    coords: Vec<Point<f32>>,
}

#[derive(Default, Application)]
struct Application {
    window: Window,
}

impl Application {
    fn new() -> Self {
        let app: Self = Default::default();
        app.window
            .set_window_size(512, 512)
            .set_title("UX Framework - Lines")
            .show()
            .connect_destroy(move |_win| Application::quit());

        app.window.set_background_color(Some(color::GRAY_9));

        let surface = Surface::new();
        surface.set_size(400.0, 400.0);

        // we should also change surface content size to avoid distortion
        surface.set_content_size(400.0, 400.0);
        surface.set_position(56.0, 56.0);

        app.window.set_child(&surface);

        let glob = Rc::new(RefCell::new(Glob::default()));

        let state = glob.clone();
        surface.connect_draw(move |_widget, ctx, width, height| {

            ctx.set_fill_color(color::GRAY_3);
            ctx.fill_rect(0.0, 0.0, width as f64, height as f64);
            
            ctx.set_stroke_color(color::TEAL_9);
            ctx.set_line_width(3.0);

            let mut state = state.borrow_mut();
            ctx.begin_path();

            let count = state.coords.len();
            for idx in 0..count {
                for jdx in 0..count {
                    let point = state.coords.get(idx).unwrap();
                    ctx.move_to(point.x as f64, point.y as f64);

                    let point = state.coords.get(jdx).unwrap();
                    ctx.line_to(point.x as f64, point.y as f64);
                }
            }

            ctx.stroke();

            state.coords.clear();
            false
        });

        let action = ClickAction::new();
        surface.add_action(&action);
        surface.set_reactive(true);

        let state = glob;
        action.connect_clicked(move |action, actor| {
            match action.get_button() {
                1 => {
                    // Left button click
                    let (screen_x, screen_y) = action.get_coords();
                    let (actor_x, actor_y) = actor.get_position();
                    let mut state = state.borrow_mut();
                    state.coords.push(Point::new(screen_x - actor_x, screen_y - actor_y));
                }
                3 => {
                    // Right button click
                    surface.invalidate()
                }
                _ => {}
            }
        });

        app
    }
}

fn main() {
    Application::run();
}
