use animate::CairoCanvas;
use charts::{Chart, GaugeChart, GaugeChartOptions};
use dataflow::*;
use gtk::prelude::*;

fn create_stream() -> DataStream<'static, &'static str, i32> {
    let metadata = vec![
        Channel {
            name: "Browser",
            tag: 0,
            visible: true,
        },
        Channel {
            name: "Share",
            tag: 1,
            visible: true,
        },
    ];

    let mut frames = vec![DataFrame {
        metric: "Memory",
        data: [(0, 25)].iter().cloned().collect(),
    }];

    frames.push(DataFrame {
        metric: "CPU",
        data: [(0, 75)].iter().cloned().collect(),
    });

    frames.push(DataFrame {
        metric: "Disk",
        data: [(0, 40)].iter().cloned().collect(),
    });

    DataStream::new(metadata, frames)
}

#[allow(dead_code)]
fn prepare() {
    //   let changeDataButton = ButtonElement()..text = "Change data";
    //   document.body.append(changeDataButton);

    //   let insertRemoveRowButton = ButtonElement()..text = "Insert/remove data row";
    //   document.body.append(insertRemoveRowButton);

    //   fn disableAllButtons() {
    //     changeDataButton.disabled = true;
    //     insertRemoveRowButton.disabled = true;
    //   }

    //   changeDataButton.onClick.listen((_) {
    //     disableAllButtons();
    //     for (let row in table.rows) {
    //       for (let i = 1; i < table.columns.length; i++) {
    //         row[i] = rand(0, 101);
    //       }
    //     }
    //     chart.update();
    //   });

    //   let insertRow = true;
    //   insertRemoveRowButton.onClick.listen((_) {
    //     insertRemoveRowButton.disabled = true;
    //     if (insertRow) {
    //       let values = ["New", rand(0, 101)];
    //       table.rows.insert(1, values);
    //     } else {
    //       table.rows.removeAt(1);
    //     }
    //     insertRow = !insertRow;
    //     chart.update();
    //   });
}

pub struct Window {
    pub widget: gtk::Window,
}

impl Window {
    pub fn new() -> Self {
        let widget = gtk::Window::new(gtk::WindowType::Toplevel);

        let drawing_area = Box::new(gtk::DrawingArea::new)();
        let default_size = (800.0, 400.0);
        // let padding = 30.0;

        let stream = create_stream();

        let mut options: GaugeChartOptions = Default::default();
        options.labels = Some(Default::default());
        options.title.text = Some("Gauge Chart Demo");

        //     "animation": {
        //       "easing": (f64 t) {
        //         t = 4 * t - 2;
        //         return (t * t * t - t) / 12 + .5;
        //       },
        //       "onEnd": () {
        //         changeDataButton.disabled = false;
        //         insertRemoveRowButton.disabled = false;
        //       }
        //     },
        //     "title": {"text": "Gauge Chart Demo"},

        let mut chart = GaugeChart::new(options);
        chart.set_stream(stream);

        drawing_area.connect_draw(move |area, cr| {
            let (rect, _) = area.get_allocated_size();
            let size = (rect.width as f64, rect.height as f64);
            // let chart_area: (f64, f64) = (size.0 - padding * 2.0, size.1 - padding * 2.0);

            chart.resize(size.0, size.1);

            let ctx = CairoCanvas::new(cr); // overhead
            chart.draw(&ctx);

            Inhibit(false)
        });

        widget.set_default_size(default_size.0 as i32, default_size.1 as i32);
        widget.add(&drawing_area);

        Self { widget }
    }

    pub fn show_all(&self) {
        self.widget.show_all();
    }
}
