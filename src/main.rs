mod window;
mod tab;
mod dialog;
mod favicon;
mod application;

use gtk4::gio;
use crate::application::Application;

const APP_ID: &str = "io.gosub.browser-gtk";

fn main() {
    colog::init();

    gio::resources_register_include!("gosub.gresource").expect("Failed to register resources.");

    let app = Application::new();
    app.run();
}