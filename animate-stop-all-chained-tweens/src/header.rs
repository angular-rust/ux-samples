use gtk::*;
pub struct Header {
    pub container: HeaderBar,
    pub contact_new: Button,
}

impl Header {
    pub fn new() -> Header {
        let container = HeaderBar::new();
        container.set_show_close_button(true); // show close, minimise, maximize
        container.set_title(Some("Wallet"));


        let options_btn = Button::from_icon_name(Some("image-loading"), gtk::IconSize::Button);
        options_btn.set_relief(gtk::ReliefStyle::None);
        container.pack_end(&options_btn);

        let contact_new = Button::from_icon_name(Some("contact-new"), gtk::IconSize::Button);
        contact_new.set_relief(gtk::ReliefStyle::None);

        // hello_btn
        //     .get_style_context()
        //     .add_class("destructive-action");

        container.pack_end(&contact_new);

        Header {
            container,
            contact_new,
        }
    }
}
