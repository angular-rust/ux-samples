use dataflow::*;
use ux::prelude::*;
use ux::{ClickAction, Surface, Window};

use charts::{BarChart, BarChartOptions, Chart};

fn create_stream() -> DataStream<'static, &'static str, i32> {
    let metadata = vec![
        Channel {
            name: "Series 1",
            tag: 0,
            visible: true,
        },
        Channel {
            name: "Series 2",
            tag: 1,
            visible: true,
        },
        Channel {
            name: "Series 3",
            tag: 2,
            visible: true,
        },
    ];

    let mut frames = vec![DataFrame {
        metric: "Monday",
        data: [(0, 1), (1, 3), (2, 5)].iter().cloned().collect(),
    }];

    frames.push(DataFrame {
        metric: "Tuesday",
        data: [(0, 3), (1, 4), (2, 6)].iter().cloned().collect(),
    });

    frames.push(DataFrame {
        metric: "Wednesday",
        data: [(0, 4), (1, 3), (2, 1)].iter().cloned().collect(),
    });

    // let skip one stream flow
    frames.push(DataFrame {
        metric: "Thursday",
        data: [(1, 5), (2, 1)].iter().cloned().collect(),
    });

    frames.push(DataFrame {
        metric: "Friday",
        data: [(0, 3), (1, 4), (2, 2)].iter().cloned().collect(),
    });

    frames.push(DataFrame {
        metric: "Saturday",
        data: [(0, 5), (1, 10), (2, 4)].iter().cloned().collect(),
    });

    frames.push(DataFrame {
        metric: "Sunday",
        data: [(0, 4), (1, 12), (2, 8)].iter().cloned().collect(),
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
            .set_title("UX Framework - BarChart")
            .show()
            .connect_destroy(move |_win| Application::quit());

        app.window.set_background_color(Some(color::TEAL_9));

        let surface = Surface::new();
        surface.set_size(800.0, 400.0);
        surface.set_content_size(800.0, 400.0);

        app.window.set_child(&surface);

        let mut options: BarChartOptions = Default::default();
        options.channel.labels = Some(Default::default());
        options.xaxis.labels.max_rotation = 90;
        options.xaxis.labels.min_rotation = 0;
        options.yaxis.min_value = Some(0);
        options.yaxis.min_interval = Some(2.);
        options.title.text = Some("Bar Chart Demo");

        let mut chart = BarChart::new(options);

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
