use gio::prelude::*;
use glib::prelude::*;
use gtk::prelude::*;
use gdk::prelude::*;
use gdk_pixbuf::prelude::*;

use std::env::args;
use std::rc::Rc;

use gtk::ResponseType;

pub struct Window {
    pub widget: gtk::Window,
}

impl Window {
    pub fn new() -> Self {
        let widget = gtk::Window::new(gtk::WindowType::Toplevel);

        Self {
            widget,
        }
    }

    pub fn show_all(&self) {
        self.widget.show_all();
    }
}
