mod window;
mod tab;
mod dialog;
mod favicon;

use gtk::prelude::*;
use gtk::{gio, glib, Application, CssProvider};
use gtk::gdk::Display;
use crate::tab::GosubTab;
use crate::window::BrowserWindow;

const APP_ID: &str = "io.gosub.browser-gtk";

fn main() -> glib::ExitCode {
    gio::resources_register_include!("gosub.gresource").expect("Failed to register resources.");

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);
    app.run()
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_data(include_str!("../resources/style.css"));

    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION
    );
}

// Should this functionality be inside the windows module? This has nothing to do with the application.
fn build_ui(app: &Application) {
    let window = BrowserWindow::new(app);
    window.present();
}