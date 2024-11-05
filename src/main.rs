mod window;
mod tab;
mod dialog;
mod favicon;
mod application;
mod settings;

use gtk4::prelude::*;
use gtk4::gio;
use crate::application::Application;

const APP_ID: &str = "io.gosub.browser-gtk";

fn main() {
    gio::resources_register_include!("gosub.gresource").expect("Failed to register resources.");

    let app = Application::new();
    app.run();
}

// fn load_css() {
//     let provider = CssProvider::new();
//     provider.load_from_data(include_str!("../resources/style.css"));
//
//     gtk::style_context_add_provider_for_display(
//         &Display::default().expect("Could not connect to a display"),
//         &provider,
//         gtk::STYLE_PROVIDER_PRIORITY_APPLICATION
//     );
// }
//
// fn build_ui(app: &Application) {
//     let window = BrowserWindow::new(app);
//     window.present();
// }