mod window;

use gtk::prelude::*;
use gtk::{gio, glib, Application};
use window::Window;

const APP_ID: &str = "io.gosub.browser-gtk";

fn main() -> glib::ExitCode {
    gio::resources_register_include!("gosub.gresource").expect("Failed to register resources.");

    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let window = Window::new(app);
    window.present();
}