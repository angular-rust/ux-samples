use animate::Canvas;
use charts::{Chart, RadarChart, RadarChartOptions};
use gtk::prelude::*;
use dataflow::*;

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

#[allow(dead_code)]
fn prepare() {


    // let changeDataButton = ButtonElement()..text = "Change data";
    // document.body.append(changeDataButton);

    // let insertRemoveColumnButton = ButtonElement()
    //   ..text = "Insert/remove data column";
    // document.body.append(insertRemoveColumnButton);

    // let insertRemoveRowButton = ButtonElement()..text = "Insert/remove data row";
    // document.body.append(insertRemoveRowButton);

    // fn disableAllButtons() {
    //   changeDataButton.disabled = true;
    //   insertRemoveColumnButton.disabled = true;
    //   insertRemoveRowButton.disabled = true;
    // }

    // changeDataButton.onClick.listen((_) {
    //   disableAllButtons();
    //   for (let row in table.rows) {
    //     for (let i = 1; i < table.columns.length; i++) {
    //       row[i] = rand(5, 20);
    //     }
    //   }
    //   chart.update();
    // });

    // let insertColumn = true;
    // insertRemoveColumnButton.onClick.listen((_) {
    //   disableAllButtons();
    //   if (insertColumn) {
    //     table.columns.insert(2, DataColumn("New series", num));
    //     for (let row in table.rows) {
    //       row[2] = rand(5, 20);
    //     }
    //   } else {
    //     table.columns.removeAt(2);
    //   }
    //   insertColumn = !insertColumn;
    //   chart.update();
    // });

    // let insertRow = true;
    // insertRemoveRowButton.onClick.listen((_) {
    //   disableAllButtons();
    //   if (insertRow) {
    //     let values = <Object>["New"];
    //     for (let i = 1; i < table.columns.length; i++) {
    //       values.add(rand(5, 20));
    //     }
    //     table.rows.insert(2, values);
    //   } else {
    //     table.rows.removeAt(2);
    //   }
    //   insertRow = !insertRow;
    //   chart.update();
    // });
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

        let mut options: RadarChartOptions = Default::default();
        options.channel.labels = Some(Default::default());
        options.title.text = Some("Radar Chart Demo");

        //   "animation": {
        //     "onEnd": () {
        //       changeDataButton.disabled = false;
        //       insertRemoveColumnButton.disabled = false;
        //       insertRemoveRowButton.disabled = false;
        //     }
        //   },
        //   "tooltip": {"valueFormatter": (value) => "$value units"}

        let mut chart = RadarChart::new(options);
        chart.set_stream(stream);

        drawing_area.connect_draw(move |area, cr| {
            let (rect, _) = area.get_allocated_size();
            let size = (rect.width as f64, rect.height as f64);
            // let chart_area: (f64, f64) = (size.0 - padding * 2.0, size.1 - padding * 2.0);

            chart.resize(size.0, size.1);

            let ctx = Canvas::new(cr); // overhead
            chart.draw(&ctx);

            Inhibit(false)
        });

        widget.set_default_size(default_size.0 as i32, default_size.1 as i32);
        widget.add(&drawing_area);

        Self {
            widget,
        }
    }

    pub fn show_all(&self) {
        self.widget.show_all();
    }
}
