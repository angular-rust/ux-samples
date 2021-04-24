use gtk::*;
pub struct Header {
    pub container: HeaderBar,
}

impl Header {
    pub fn new() -> Header {
        let container = HeaderBar::new();
        container.set_show_close_button(true);
        container.set_title(Some("UX Framework - BarChart (GTK)"));

        Header {
            container,
        }
    }
}
