mod window;
mod tab;
mod dialog;

use gtk::prelude::*;
use gtk::{gio, glib, Application, CssProvider };
use gtk::gdk::Display;
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

fn build_ui(app: &Application) {
    let window = BrowserWindow::new(app);

    let builder = gtk::Builder::from_resource("/io/gosub/browser-gtk/ui/main_menu.ui");
    let menubar = builder.object::<gio::MenuModel>("app-menu").expect("Could not find app-menu");

    app.set_menubar(Some(&menubar));
    window.set_show_menubar(true);

    let action = gio::SimpleAction::new_stateful("toggle_darkmode", None, &false.to_variant());
    action.connect_activate(move |action, _| {
        let is_dark_mode = action.state().unwrap().get::<bool>().unwrap();
        action.set_state(&(!is_dark_mode).to_variant());

        toggle_dark_mode();
    });
    app.add_action(&action);


    window.present();
}

fn toggle_dark_mode() {
    if let Some(settings) = gtk::Settings::default() {
        let is_dark = settings.is_gtk_application_prefer_dark_theme();
        settings.set_gtk_application_prefer_dark_theme(!is_dark);
    }
}
