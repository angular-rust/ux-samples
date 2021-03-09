pub mod app;
pub mod header;
pub mod window;

use app::App;

fn main() {
    // Initilize the application with the default config
    let app = App::new();
    app.window.show_all();
    gtk::main();
}