use animate::Canvas;
use charts::{BarChart, BarChartOptions, Chart};
use dataflow::*;
use gtk::prelude::*;

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

pub struct Window {
    pub widget: gtk::Window,
}

impl Window {
    pub fn new() -> Self {
        let widget = gtk::Window::new(gtk::WindowType::Toplevel);

        let drawing_area = Box::new(gtk::DrawingArea::new)();
        let default_size = (800.0, 400.0);

        let stream = create_stream();

        let mut options: BarChartOptions = Default::default();
        options.channel.labels = Some(Default::default());
        options.xaxis.labels.max_rotation = 90;
        options.xaxis.labels.min_rotation = 0;
        options.yaxis.min_value = Some(0);
        options.yaxis.min_interval = Some(2.);
        options.title.text = Some("Bar Chart Demo");

        let mut chart = BarChart::new(options);
        chart.set_stream(stream);

        drawing_area.connect_draw(move |area, cr| {
            let (rect, _) = area.get_allocated_size();
            let size = (rect.width as f64, rect.height as f64);

            chart.resize(size.0, size.1);

            let ctx = Canvas::new(cr);
            chart.draw(&ctx);

            Inhibit(false)
        });

        widget.set_default_size(default_size.0 as i32, default_size.1 as i32);
        widget.add(&drawing_area);

        // glib::timeout_add_local(1000, move || {
        //     drawing_area.queue_draw();
        //     Continue(true) // Continue(false) - to stop
        // });

        Self { widget }
    }

    pub fn show_all(&self) {
        self.widget.show_all();
    }
}
