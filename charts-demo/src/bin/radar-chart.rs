use charts::{Chart, RadarChart, RadarChartOptions};
use dataflow::*;
use ux::prelude::*;
use ux::{ClickAction, Surface, Window};

fn create_stream() -> DataStream<'static, &'static str, i32> {
    let metadata = vec![
        Channel {
            name: "Series 1",
            tag: 0,
            visible: true,
        },
        Channel {
            name: "New Series",
            tag: 1,
            visible: true,
        },
    ];

    let mut frames = vec![DataFrame {
        metric: "Monday",
        data: [(0, 11), (1, 16)].iter().cloned().collect(),
    }];

    frames.push(DataFrame {
        metric: "Tuesday",
        data: [(0, 19), (1, 15)].iter().cloned().collect(),
    });

    frames.push(DataFrame {
        metric: "Wednesday",
        data: [(0, 7), (1, 14)].iter().cloned().collect(),
    });

    frames.push(DataFrame {
        metric: "Thursday",
        data: [(0, 17), (1, 12)].iter().cloned().collect(),
    });

    frames.push(DataFrame {
        metric: "Friday",
        data: [(0, 17), (1, 10)].iter().cloned().collect(),
    });

    frames.push(DataFrame {
        metric: "Saturday",
        data: [(0, 18), (1, 9)].iter().cloned().collect(),
    });

    frames.push(DataFrame {
        metric: "Sunday",
        data: [(0, 15), (1, 14)].iter().cloned().collect(),
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
            .set_title("UX Framework - RadarChart")
            .show()
            .connect_destroy(move |_win| Application::quit());

        app.window.set_background_color(Some(color::TEAL_9));

        let surface = Surface::new();
        surface.set_size(800.0, 400.0);
        surface.set_content_size(800.0, 400.0);

        app.window.set_child(&surface);

        let mut options: RadarChartOptions = Default::default();
        options.channel.labels = Some(Default::default());
        options.title.text = Some("Radar Chart Demo");

        let mut chart = RadarChart::new(options);

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
