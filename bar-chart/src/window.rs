use gio::prelude::*;
use glib::prelude::*;
use gtk::prelude::*;
use gdk::prelude::*;
use gdk_pixbuf::prelude::*;

use std::env::args;
use std::rc::Rc;

use gtk::ResponseType;

use row_data::RowData;

#[derive(Debug)]
#[repr(i32)]
enum Columns {
    Fixed = 0,
    Number,
    Severity,
    Description,
    Pulse,
    Icon,
    Active,
    Sensitive,
}

const UI: &str = include_str!("window.ui");
pub struct Window {
    // pub widget: gtk::ApplicationWindow,
    pub widget: gtk::Window,

    // contacts switcher and drop receiver
    contacts: gtk::ListBox,
    // activity (process) switcher
    switcher: gtk::Box,
    // stack of activities - aka main view for activity
    perspective: gtk::Stack,
}

impl Window {
    pub fn new() -> Self {
        // let builder = gtk::Builder::from_resource("/space/ohyo/window1.ui");

        // let builder = gtk::Builder::from_string(UI);
        // let widget: gtk::ApplicationWindow = builder
        //     .get_object("window")
        //     .expect("Failed to find the window object");

        let widget = gtk::Window::new(gtk::WindowType::Toplevel);

        let container = gtk::Box::new(gtk::Orientation::Horizontal, 3);

        let switcher = gtk::Box::new(gtk::Orientation::Vertical, 0);

        let avatar_btn = gtk::Button::from_icon_name(Some("avatar-default"), gtk::IconSize::Button);
        avatar_btn.set_relief(gtk::ReliefStyle::None);

        let top_switcher = gtk::Box::new(gtk::Orientation::Vertical, 0);
        switcher.pack_start(&avatar_btn, false, false, 0);

        let browser_btn = gtk::Button::from_icon_name(Some("image-loading"), gtk::IconSize::Button);
        browser_btn.set_relief(gtk::ReliefStyle::None);

        // let hello_btn = gtk::Button::with_label("Hello johny");
        // hello_btn.get_style_context().add_class("hello-btn");

        switcher.pack_start(&top_switcher, true, true, 0);
        switcher.pack_start(&browser_btn, false, false, 0);

        container.pack_start(&switcher, false, true, 0);

        let perspective = gtk::Stack::new();

        // /com/github/akiraux/akira/akira-banner.jpg
        let pixbuf = gdk_pixbuf::Pixbuf::from_resource("/com/github/akiraux/akira/more_vert.svg").expect("ooops");
        // let pixbuf = pixbuf.scale_simple(64, 64, gdk_pixbuf::InterpType::Bilinear).expect("ooops");
        let img = gtk::Image::from_pixbuf(Some(&pixbuf));
        
        // let img = gtk::Image::from_resource("/com/github/akiraux/akira/contact1.jpg");
        

        let mainview = gtk::Button::new();
        mainview.set_image(Some(&img));
        // let mainview = gtk::Button::with_label("Hello");
        mainview.set_relief(gtk::ReliefStyle::None);
        mainview.get_style_context().add_class("hello-btn");
        // switcher.pack_start(&hello_btn, true, true, 0);

        let message_entry = gtk::Entry::new();
        message_entry.set_icon_from_icon_name(gtk::EntryIconPosition::Primary, Some("contact-new"));

        let perspective_lauoyt = gtk::Box::new(gtk::Orientation::Vertical, 0);
        perspective_lauoyt.pack_start(&mainview, true, true, 0);
        perspective_lauoyt.pack_start(&message_entry, false, false, 0);
        
        perspective.add_named(&perspective_lauoyt, "name");

        container.pack_start(&perspective, true, true, 0);

        // let hello_btn1 = gtk::Button::with_label("Hello");
        // // hello_btn.get_style_context().add_class("hello-btn");
        // container.pack_start(&hello_btn1, true, true, 0);

        ///////////////// list

        // Create our list store and specify that the type stored in the
        // list should be the RowData GObject we define at the bottom
        let model = gio::ListStore::new(RowData::static_type());

        // And then create the UI part, the listbox and bind the list store
        // model to it. Whenever the UI needs to show a new row, e.g. because
        // it was notified that the model changed, it will call the callback
        // with the corresponding item from the model and will ask for a new
        // gtk::ListBoxRow that should be displayed.
        //
        // The gtk::ListBoxRow can contain any possible widgets.
        let contacts = gtk::ListBox::new();

        contacts.bind_model(Some(&model),
        clone!(@weak widget => @default-panic, move |item| {
            let box_ = gtk::ListBoxRow::new();
            let item = item.downcast_ref::<RowData>().expect("Row data is of wrong type");

            let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 5);

            let avatar = item.get_property("avatar").expect("avatar error");
            let avatar = avatar.get::<String>().expect("oops with string");
            let res = &format!("/com/github/akiraux/akira/{}", avatar.unwrap());

            let pixbuf = gdk_pixbuf::Pixbuf::from_resource(&res).expect("ooops");
            let pixbuf = pixbuf.scale_simple(64, 64, gdk_pixbuf::InterpType::Bilinear).expect("ooops");
            let img = gtk::Image::from_pixbuf(Some(&pixbuf));
            hbox.pack_start(&img, true, true, 0);

            // Create the label and spin button that shows the two values
            // of the item. We bind the properties for the two values to the
            // corresponding properties of the widgets so that they are automatically
            // updated whenever the item is changing. By specifying SYNC_CREATE the
            // widget will automatically get the initial value of the item set.
            //
            // In case of the spin button the binding is bidirectional, that is any
            // change of value in the spin button will be automatically reflected in
            // the item.
            let label = gtk::Label::new(None);
            item.bind_property("name", &label, "label")
                .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
                .build();
            label.set_size_request(100, 25);
            hbox.pack_start(&label, true, true, 0);

            // let spin_button = gtk::SpinButton::with_range(0.0, 100.0, 1.0);
            // item.bind_property("count", &spin_button, "value")
            //     .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE | glib::BindingFlags::BIDIRECTIONAL)
            //     .build();

            // hbox.pack_start(&spin_button, false, false, 0);

            // When the edit button is clicked, a new modal dialog is created for editing
            // the corresponding row
            // let edit_button = gtk::Button::with_label("Edit");
            // edit_button.connect_clicked(clone!(@weak widget, @strong item => move |_| {
            //     let dialog = gtk::Dialog::with_buttons(Some("Edit Item"), Some(&widget), gtk::DialogFlags::MODAL,
            //         &[("Close", ResponseType::Close)]);
            //     dialog.set_default_response(ResponseType::Close);
            //     dialog.connect_response(|dialog, _| dialog.close());

            //     let content_area = dialog.get_content_area();

            //     // Similarly to the label and spin button inside the listbox, the text entry
            //     // and spin button in the edit dialog are connected via property bindings to
            //     // the item. Any changes will be immediately reflected inside the item and
            //     // by the listbox
            //     let entry = gtk::Entry::new();
            //     item.bind_property("name", &entry, "text")
            //         .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE | glib::BindingFlags::BIDIRECTIONAL)
            //         .build();

            //     // Activating the entry (enter) will send response `ResponseType::Close` to the dialog
            //     entry.connect_activate(clone!(@weak dialog => move |_| {
            //         dialog.response(ResponseType::Close);
            //     }));
            //     content_area.add(&entry);

            //     let spin_button = gtk::SpinButton::with_range(0.0, 100.0, 1.0);
            //     item.bind_property("count", &spin_button, "value")
            //         .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE | glib::BindingFlags::BIDIRECTIONAL)
            //         .build();
            //     content_area.add(&spin_button);

            //     dialog.show_all();
            // }));
            // hbox.pack_start(&edit_button, false, false, 0);

            box_.add(&hbox);

            // // When a row is activated (select + enter) we simply emit the clicked
            // // signal on the corresponding edit button to open the edit dialog
            // box_.connect_activate(clone!(@weak edit_button => move |_| {
            //     edit_button.emit_clicked();
            // }));

            box_.show_all();

            box_.upcast::<gtk::Widget>()
        }));

        let scrolled_window = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
        scrolled_window.add(&contacts);
        scrolled_window.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Automatic);
        ///////////
        let search_contact = gtk::Entry::new();
        search_contact.set_icon_from_icon_name(gtk::EntryIconPosition::Primary, Some("edit-find"));

        let contacts_and_search = gtk::Box::new(gtk::Orientation::Vertical, 3);
        contacts_and_search.pack_start(&scrolled_window, true, true, 0);
        contacts_and_search.pack_start(&search_contact, false, true, 0);

        container.pack_start(&contacts_and_search, false, true, 0);

        widget.add(&container);

        let names = vec!["Dofia", "Saena", "Ryanna", "Phaela", "Eissa", "Eimir", "Marches", "Myrril", "Jai", "Vyn"];


        for i in 0..10 {
            let name = names.get(i).unwrap();
            // &format!("Name {}",  i)
            model.append(&RowData::new(name, &format!("contact{}.jpg",  i+1),  (i*10) as u32));
        }

        Self {
            widget,
            contacts,
            switcher,
            perspective,
        }
    }

    pub fn show_all(&self) {
        self.widget.show_all();
    }
}

// Our GObject subclass for carrying a name and count for the ListBox model
//
// Both name and count are stored in a RefCell to allow for interior mutability
// and are exposed via normal GObject properties. This allows us to use property
// bindings below to bind the values with what widgets display in the UI
mod row_data {
    use super::*;

    use glib::subclass;
    use glib::subclass::prelude::*;
    use glib::translate::*;

    // Implementation sub-module of the GObject
    mod imp {
        use super::*;
        use std::cell::RefCell;

        // The actual data structure that stores our values. This is not accessible
        // directly from the outside.
        pub struct RowData {
            pub name: RefCell<Option<String>>,
            pub avatar: RefCell<Option<String>>,
            pub count: RefCell<u32>,
        }

        // GObject property definitions for our two values
        static PROPERTIES: [subclass::Property; 3] = [
            subclass::Property("name", |name| {
                glib::ParamSpec::string(
                    name,
                    "Name",
                    "Name",
                    None, // Default value
                    glib::ParamFlags::READWRITE,
                )
            }),
            subclass::Property("avatar", |avatar| {
                glib::ParamSpec::string(
                    avatar,
                    "Avatar",
                    "Avatar",
                    None, // Default value
                    glib::ParamFlags::READWRITE,
                )
            }),
            subclass::Property("count", |name| {
                glib::ParamSpec::uint(
                    name,
                    "Count",
                    "Count",
                    0,
                    100,
                    0, // Allowed range and default value
                    glib::ParamFlags::READWRITE,
                )
            }),
        ];

        // Basic declaration of our type for the GObject type system
        impl ObjectSubclass for RowData {
            const NAME: &'static str = "RowData";
            type ParentType = glib::Object;
            type Instance = subclass::simple::InstanceStruct<Self>;
            type Class = subclass::simple::ClassStruct<Self>;

            glib_object_subclass!();

            // Called exactly once before the first instantiation of an instance. This
            // sets up any type-specific things, in this specific case it installs the
            // properties so that GObject knows about their existence and they can be
            // used on instances of our type
            fn class_init(klass: &mut Self::Class) {
                klass.install_properties(&PROPERTIES);
            }

            // Called once at the very beginning of instantiation of each instance and
            // creates the data structure that contains all our state
            fn new() -> Self {
                Self {
                    name: RefCell::new(None),
                    avatar: RefCell::new(None),
                    count: RefCell::new(0),
                }
            }
        }

        // The ObjectImpl trait provides the setters/getters for GObject properties.
        // Here we need to provide the values that are internally stored back to the
        // caller, or store whatever new value the caller is providing.
        //
        // This maps between the GObject properties and our internal storage of the
        // corresponding values of the properties.
        impl ObjectImpl for RowData {
            glib_object_impl!();

            fn set_property(&self, _obj: &glib::Object, id: usize, value: &glib::Value) {
                let prop = &PROPERTIES[id];

                match *prop {
                    subclass::Property("name", ..) => {
                        let name = value
                            .get()
                            .expect("type conformity checked by `Object::set_property`");
                        self.name.replace(name);
                    },
                    subclass::Property("avatar", ..) => {
                        let avatar = value
                            .get()
                            .expect("type conformity checked by `Object::set_property`");
                        self.avatar.replace(avatar);
                    },
                    subclass::Property("count", ..) => {
                        let count = value
                            .get_some()
                            .expect("type conformity checked by `Object::set_property`");
                        self.count.replace(count);
                    },
                    _ => unimplemented!(),
                }
            }

            fn get_property(&self, _obj: &glib::Object, id: usize) -> Result<glib::Value, ()> {
                let prop = &PROPERTIES[id];

                match *prop {
                    subclass::Property("name", ..) => Ok(self.name.borrow().to_value()),
                    subclass::Property("avatar", ..) => Ok(self.avatar.borrow().to_value()),
                    subclass::Property("count", ..) => Ok(self.count.borrow().to_value()),
                    _ => unimplemented!(),
                }
            }
        }
    }

    // Public part of the RowData type. This behaves like a normal gtk-rs-style GObject
    // binding
    glib_wrapper! {
        pub struct RowData(Object<subclass::simple::InstanceStruct<imp::RowData>, subclass::simple::ClassStruct<imp::RowData>, RowDataClass>);

        match fn {
            get_type => || imp::RowData::get_type().to_glib(),
        }
    }

    // Constructor for new instances. This simply calls glib::Object::new() with
    // initial values for our two properties and then returns the new instance
    impl RowData {
        pub fn new(name: &str, avatar: &str, count: u32) -> RowData {
            glib::Object::new(Self::static_type(), &[("name", &name), ("avatar", &avatar), ("count", &count)])
                .expect("Failed to create row data")
                .downcast()
                .expect("Created row data is of wrong type")
        }
    }
}
