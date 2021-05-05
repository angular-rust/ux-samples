use charts::{Chart, GaugeChart, GaugeChartOptions};
use dataflow::*;
use ux::prelude::*;
use ux::{ClickAction, Surface, Window};

fn create_stream() -> DataStream<String, i32> {
    let metadata = vec![
        Channel {
            name: "Browser".into(),
            tag: 0,
            visible: true,
        },
        Channel {
            name: "Share".into(),
            tag: 1,
            visible: true,
        },
    ];

    let mut frames: Vec<DataFrame<String, i32>> = vec![DataFrame {
        metric: "Memory".into(),
        data: [(0, 25)].iter().cloned().collect(),
    }];

    frames.push(DataFrame {
        metric: "CPU".into(),
        data: [(0, 75)].iter().cloned().collect(),
    });

    frames.push(DataFrame {
        metric: "Disk".into(),
        data: [(0, 40)].iter().cloned().collect(),
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
            .set_title("UX Framework - GaugeChart")
            .show()
            .connect_destroy(move |_win| Application::quit());

        app.window.set_background_color(Some(color::TEAL_9));

        let surface = Surface::new();
        surface.set_size(800.0, 400.0);
        surface.set_content_size(800.0, 400.0);

        app.window.set_child(&surface);

        let mut options: GaugeChartOptions = Default::default();
        options.labels = Some(Default::default());
        options.title.text = Some("Gauge Chart Demo".into());

        let mut chart = GaugeChart::new(options);

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
