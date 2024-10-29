mod window;
mod tab;
mod dialogs;

use gtk::prelude::*;
use gtk::{gio, glib, Application, CssProvider };
use gtk::gdk::Display;
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


    app.set_accels_for_action("app.quit", &["<Primary>Q"]);
    app.set_accels_for_action("app.open", &["<Primary>O"]);
    app.set_accels_for_action("app.save", &["<Primary>S"]);
    app.set_accels_for_action("app.save_as", &["<Primary><Shift>S"]);
    app.set_accels_for_action("app.cut", &["<Primary>X"]);
    app.set_accels_for_action("app.copy", &["<Primary>C"]);
    app.set_accels_for_action("app.paste", &["<Primary>V"]);
    app.set_accels_for_action("app.reload", &["<Primary>R"]);
    app.set_accels_for_action("app.dev_tools", &["<Primary><Shift>I"]);
    app.set_accels_for_action("app.about", &["<Primary>A"]);

    let menubar = {
        let file_menu = {
            let s1 = gio::Menu::new();
            let s2 = gio::Menu::new();

            let open_menu_item = gio::MenuItem::new(Some("Open"), Some("app.open"));
            let save_menu_item = gio::MenuItem::new(Some("Save"), Some("app.save"));
            let save_as_menu_item = gio::MenuItem::new(Some("Save As"), Some("app.save_as"));
            let quit_menu_item = gio::MenuItem::new(Some("Quit"), Some("app.quit"));

            s1.append_item(&open_menu_item);
            s1.append_item(&save_menu_item);
            s1.append_item(&save_as_menu_item);
            s2.append_item(&quit_menu_item);


            let file_menu = gio::Menu::new();
            file_menu.append_section(None, &s1);
            file_menu.append_section(None, &s2);

            file_menu
        };

        let edit_menu = {
            let cut_menu_item = gio::MenuItem::new(Some("Cut"), Some("app.cut"));
            let copy_menu_item = gio::MenuItem::new(Some("Copy"), Some("app.copy"));
            let paste_menu_item = gio::MenuItem::new(Some("Paste"), Some("app.paste"));

            let edit_menu = gio::Menu::new();
            edit_menu.append_item(&cut_menu_item);
            edit_menu.append_item(&copy_menu_item);
            edit_menu.append_item(&paste_menu_item);
            edit_menu
        };

        let view_menu = {
            let reload_menu_item = gio::MenuItem::new(Some("Reload"), Some("app.reload"));
            let dev_tools_menu_item = gio::MenuItem::new(Some("Developer Tools"), Some("app.dev_tools"));

            let view_menu = gio::Menu::new();
            view_menu.append_item(&reload_menu_item);
            view_menu.append_item(&dev_tools_menu_item);
            view_menu
        };

        let help_menu = {
            let about_menu_item = gio::MenuItem::new(Some("About"), Some("app.about"));

            let help_menu = gio::Menu::new();
            help_menu.append_item(&about_menu_item);
            help_menu
        };

        let menubar = gio::Menu::new();
        menubar.append_submenu(Some("File"), &file_menu);
        menubar.append_submenu(Some("Edit"), &edit_menu);
        menubar.append_submenu(Some("View"), &view_menu);
        menubar.append_submenu(Some("Help"), &help_menu);

        menubar
    };

    // let builder = gtk::Builder::from_resource("/io/gosub/browser-gtk/ui/main_menu.ui");
    // let menubar = builder.object::<MenuModel>("app-menu").expect("Could not find app-menu");

    app.set_menubar(Some(&menubar));
    window.set_show_menubar(true);

    window.present();
}
