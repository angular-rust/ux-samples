use gtk::prelude::*;
use std::process;

use super::header::Header;
use super::window::Window;

const CSS: &str = include_str!("styles/gtk.css");

pub struct App {
    pub window: Window,
    pub header: Header,
}

impl App {
    pub fn new() -> App {
        if gtk::init().is_err() {
            println!("failed to init GTK");
            process::exit(1);
        }

        let provider = gtk::CssProvider::new();
        let display = gdk::Display::get_default().unwrap();
        let screen = display.get_default_screen();

        gtk::StyleContext::add_provider_for_screen(
            &screen,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_USER,
        );

        if let Err(err) = provider.load_from_data(CSS.as_bytes()) {
            println!("CSS ERR{}", err);
        }

        let window = Window::new();
        let header = Header::new();

        window.widget.set_titlebar(Some(&header.container));
        window.widget.connect_delete_event(move |_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        //return
        App { window, header }
    }
}
