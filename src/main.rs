mod window;
mod tab;
mod dialogs;

use gtk::prelude::*;
use gtk::{gio, glib, Application, CssProvider};
use gtk::gdk::Display;
use gtk::gio::MenuModel;
use crate::window::Window;

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
    let window = Window::new(app);


    let _menubar = {
        let file_menu = {
            let about_menu_item = gio::MenuItem::new(Some("About"), Some("app.about"));
            let quit_menu_item = gio::MenuItem::new(Some("Quit"), Some("app.quit"));

            let file_menu = gio::Menu::new();
            file_menu.append_item(&about_menu_item);
            file_menu.append_item(&quit_menu_item);
            file_menu
        };

        let menubar = gio::Menu::new();
        menubar.append_submenu(Some("File"), &file_menu);

        menubar
    };

    let builder = gtk::Builder::from_resource("/io/gosub/browser-gtk/ui/main_menu.ui");
    let menubar = builder.object::<MenuModel>("app-menu").expect("Could not find app-menu");

    app.set_menubar(Some(&menubar));
    window.set_show_menubar(true);

    window.present();
}
