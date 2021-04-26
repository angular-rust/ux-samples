use charts::{Chart, PieChart, PieChartOptions};
use dataflow::*;
use ux::prelude::*;
use ux::{ClickAction, Surface, Window};

fn create_stream() -> DataStream<'static, &'static str, i32> {
    let metadata = vec![
        Channel {
            name: "Browser",
            tag: 0,
            visible: true,
        }
    ];

    let mut frames = vec![DataFrame {
        metric: "Chrome",
        data: [(0, 35)].iter().cloned().collect(),
    }];

    frames.push(DataFrame {
        metric: "Firefox",
        data: [(0, 20)].iter().cloned().collect(),
    });

    frames.push(DataFrame {
        metric: "IE",
        data: [(0, 30)].iter().cloned().collect(),
    });

    frames.push(DataFrame {
        metric: "Opera",
        data: [(0, 5)].iter().cloned().collect(),
    });

    frames.push(DataFrame {
        metric: "Safari",
        data: [(0, 8)].iter().cloned().collect(),
    });

    frames.push(DataFrame {
        metric: "Other",
        data: [(0, 2)].iter().cloned().collect(),
    });

    DataStream::new(metadata, frames)
}

#[derive(Default, Application)]
struct Application {
    window: Window,
}

impl Application {
    fn new() -> Self {
        let app: Self = Default::default();
        app.window
            .set_window_size(800, 400)
            .set_title("UX Framework - PieChart")
            .show()
            .connect_destroy(move |_win| Application::quit());

        app.window.set_background_color(Some(color::TEAL_9));

        let surface = Surface::new();
        surface.set_size(800.0, 400.0);
        surface.set_content_size(800.0, 400.0);

        app.window.set_child(&surface);

        let mut options: PieChartOptions = Default::default();
        options.channel.labels.enabled = true;
        options.channel.counterclockwise = true;
        options.channel.labels.enabled = true;
        options.channel.start_angle = 90. + 10. * 360.;
        options.pie_hole = 0.5;
        options.title.text = Some("Pie Chart Demo");

        let mut chart = PieChart::new(options);

        let stream = create_stream();
        chart.set_stream(stream);

        surface.connect_draw(move |_widget, ctx, width, height| {
            chart.resize(width as f64, height as f64);
            chart.draw(ctx);
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
