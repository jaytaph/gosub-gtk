use gtk::glib;

mod imp;
use gtk::gio;
use gtk::Application;
use gtk::gio::SimpleAction;
use gtk::prelude::GtkWindowExt;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use crate::{show_about_dialog, toggle_dark_mode};
use crate::GosubTab;
use crate::add_new_tab;
use gtk::prelude::*;

// This wrapper must be in a different module than the implementation, because both will define a
// `struct BrowserWindow` and they would clash. In this case, the browser window is a subclass of
// it's implementation. It's all a bit messy tbh...
glib::wrapper! {
    pub struct BrowserWindow(ObjectSubclass<imp::BrowserWindow>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl BrowserWindow {
    pub fn new(app: &Application) -> Self {
        let window: Self = glib::Object::builder().property("application", app).build();

        window.set_resizable(true);
        window.set_decorated(true);
        window.set_default_size(800, 600);

        let builder = gtk::Builder::from_resource("/io/gosub/browser-gtk/ui/main_menu.ui");
        let menubar = builder.object::<gio::MenuModel>("app-menu").expect("Could not find app-menu");

        app.set_menubar(Some(&menubar));
        window.set_show_menubar(true);

        // Dark mode toggle
        let action = SimpleAction::new_stateful("toggle_darkmode", None, &false.to_variant());
        action.connect_activate(move |action, _| {
            let is_dark_mode = action.state().unwrap().get::<bool>().unwrap();
            action.set_state(&(!is_dark_mode).to_variant());

            toggle_dark_mode();
        });
        app.add_action(&action);

        // About action
        let about_action = SimpleAction::new("about", None);
        about_action.connect_activate(move |_, _| {
            show_about_dialog();
        });
        app.add_action(&about_action);
        app.set_accels_for_action("app.about", &["<Primary>A"]);

        // Create new tab
        let new_tab_action = SimpleAction::new("tab.new", None);
        new_tab_action.connect_activate({
            let tab_bar = window.imp().tab_bar.clone();
            let tabs = window.imp().tabs.clone();
            move |_, _| {
                let tab_data = GosubTab::new("https://duckduckgo.com", None);
                tabs.borrow_mut().push(tab_data.clone());
                add_new_tab(tab_bar.clone(), tab_data);
            }
        });
        app.add_action(&new_tab_action);
        app.set_accels_for_action("app.tab.new", &["<Primary>T"]);

        // Custom stuff we need to do after the window has been created
        window.imp().init_tabs();

        window
    }
}

