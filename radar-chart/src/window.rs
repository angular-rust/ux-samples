#![allow(unused_variables)]
#![allow(dead_code)]

// use gio::prelude::*;
// use glib::prelude::*;
use gtk::prelude::*;
// use gdk::prelude::*;
// use gdk_pixbuf::prelude::*;

// use std::env::args;
// use std::rc::Rc;

// use gtk::ResponseType;

// use ux_charts::*;
use ux_dataflow::*;

fn create_stream() -> DataStream<'static, &'static str, i32> {
    let metadata = vec![
        Channel {
            name: "Categories",
            tag: 0,
            visible: true,
        },
        Channel {
            name: "Series 1",
            tag: 1,
            visible: true,
        },
    ];

    // Zero stream tag is allways metric
    let mut frames = vec![DataFrame {
        metric: "Monday",
        data: [(1, 8)].iter().cloned().collect(),
    }];

    frames.push(DataFrame {
        metric: "Tuesday",
        data: [(1, 17)].iter().cloned().collect(),
    });

    frames.push(DataFrame {
        metric: "Wednesday",
        data: [(1, 7)].iter().cloned().collect(),
    });

    frames.push(DataFrame {
        metric: "Thursday",
        data: [(1, 16)].iter().cloned().collect(),
    });

    frames.push(DataFrame {
        metric: "Friday",
        data: [(1, 12)].iter().cloned().collect(),
    });

    frames.push(DataFrame {
        metric: "Saturday",
        data: [(1, 5)].iter().cloned().collect(),
    });

    frames.push(DataFrame {
        metric: "Sunday",
        data: [(1, 14)].iter().cloned().collect(),
    });

    DataStream::new(metadata, frames)
}

fn prepare() {


    // let changeDataButton = ButtonElement()..text = "Change data";
    // document.body.append(changeDataButton);

    // let insertRemoveColumnButton = ButtonElement()
    //   ..text = "Insert/remove data column";
    // document.body.append(insertRemoveColumnButton);

    // let insertRemoveRowButton = ButtonElement()..text = "Insert/remove data row";
    // document.body.append(insertRemoveRowButton);

    // let container = createContainer();

    // let options = {
    //   "animation": {
    //     "onEnd": () {
    //       changeDataButton.disabled = false;
    //       insertRemoveColumnButton.disabled = false;
    //       insertRemoveRowButton.disabled = false;
    //     }
    //   },
    //   "series": {
    //     "labels": {"enabled": true}
    //   },
    //   "title": {"text": "Radar Chart Demo"},
    //   "tooltip": {"valueFormatter": (value) => "$value units"}
    // };

    // let chart = RadarChart::new(Default::default());
    // chart.draw(table, options);

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
        let size = (600.0, 400.0);
        let padding = 30.0;
        let chart_area: (f64, f64) = (size.0 - padding * 2.0, size.1 - padding * 2.0);

        let stream = create_stream();

        let x_axis = vec![0, 1, 2, 3, 4, 5, 6, 8, 9];
        let y_axis = vec![0, 3, 5, 4, 3, 6, 6, 7, 14];

        drawing_area.connect_draw(move |_, cr| {
            // Here we draw using the given Context
    
            cr.set_source_rgb(1.0 / 255.0, 46.0 / 255.0, 64.0 / 255.0); // Background color
            cr.paint();
            // Set a monospace font
            cr.select_font_face("monospace", cairo::FontSlant::Normal, cairo::FontWeight::Bold);
            cr.set_font_size(12.0);
            cr.set_line_width(1.0);
    
            let max_x = x_axis.iter().max().unwrap();
            let max_y = y_axis.iter().max().unwrap();
            let size_x = chart_area.0 / *max_x as f64;
            let size_y = chart_area.1 / *max_y as f64;
            let data_points = x_axis.iter().zip(y_axis.iter());
            let normalized_data: Vec<(f64, f64, f64)> = data_points
                .map(|(x, y)| {
                    (
                        padding + size_x * *x as f64,
                        padding + chart_area.1 - size_y * *y as f64,
                        *y as f64,
                    )
                })
                .collect();
            
            cr.set_source_rgb(79.0 / 255.0, 134.0 / 255.0, 140.0 / 255.0); // Set the grid lines color
            for y_grid_line in 0..=(*max_y as i32) {
                let y_line = y_grid_line as f64 * size_y + padding;
                cr.move_to(padding, y_line);
                cr.line_to(size.0 - padding, y_line);
                cr.stroke();
                cr.move_to(padding / 3.0, y_line);
                cr.show_text((max_y - y_grid_line).to_string().as_ref());
            }
            for x_grid_line in 0..=(*max_x as i32) {
                let x_line = x_grid_line as f64 * size_x + padding;
                cr.move_to(x_line, padding);
                cr.line_to(x_line, size.1 - padding);
                cr.stroke();
                cr.line_to(x_line - 2.0, size.1 - padding / 3.0);
                cr.show_text(x_grid_line.to_string().as_ref());
            }
    
    
            cr.set_line_width(2.0);
            cr.set_source_rgb(191.0 / 255.0, 186.0 / 255.0, 159.0 / 255.0); // Chart line/label color
            let data_window = normalized_data.windows(2);
            for points in data_window {
                let source = points[0];
                let target = points[1];
                // Draw the line 
                cr.move_to(source.0, source.1);
                cr.line_to(target.0, target.1);
                cr.stroke();
                // Draw the label
                cr.move_to(target.0 - 8.0, target.1 - 10.0);
                cr.show_text(target.2.to_string().as_ref());
            }
    
            Inhibit(false)
        });

        widget.set_default_size(size.0 as i32, size.1 as i32);
        widget.add(&drawing_area);

        Self {
            widget,
        }
    }

    pub fn show_all(&self) {
        self.widget.show_all();
    }
}
