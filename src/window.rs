use adw::gio::SimpleAction;
use adw::gtk;
use gtk4::glib::closure_local;
use gtk::glib;

mod imp;

use crate::application::Application;
use crate::tab::GosubTab;
use gtk::gio;
use gtk::prelude::GtkWindowExt;
use gtk::prelude::*;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use log::info;

// This wrapper must be in a different module than the implementation, because both will define a
// `struct BrowserWindow` and they would clash. In this case, the browser window is a subclass of
// its implementation.
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
        let menubar = builder
            .object::<gio::MenuModel>("app-menu")
            .expect("Could not find app-menu");

        app.set_menubar(Some(&menubar));
        window.set_show_menubar(true);

        Self::connect_actions(app, &window);
        Self::connect_accelerators(app, &window);

        window.imp().init_tabs();

        window
    }

    fn connect_accelerators(app: &Application, _window: &Self) {
        app.set_accels_for_action("app.open-new-tab", &["<Primary>T"]);
        app.set_accels_for_action("app.close-tab", &["<Primary>W"]);
        app.set_accels_for_action("app.toggle-log", &["<Primary>L"]);
    }

    fn connect_actions(app: &Application, window: &Self) {
        let logwindow_action = SimpleAction::new("toggle-log", None);
        logwindow_action.connect_activate({
            let window_clone = window.clone();
            move |_, _| {
                window_clone
                    .imp()
                    .log_scroller
                    .set_visible(!window_clone.imp().log_scroller.get_visible());
            }
        });
        app.add_action(&logwindow_action);

        let window_clone = window.clone();
        window.connect_closure(
            "update-tabs",
            false,
            closure_local!(move |_: BrowserWindow| {
                info!("Refreshing tabs handler called");
                window_clone.imp().refresh_tabs();
            }),
        );

        // Create new tab
        let new_tab_action = SimpleAction::new("open-new-tab", None);
        new_tab_action.connect_activate({
            let window_clone = window.clone();
            let tab_manager = window.imp().tab_manager.clone();
            move |_, _| {
                let tab_data = GosubTab::new("gosub:blank", None);
                tab_manager.lock().unwrap().add_tab(tab_data, None);
                window_clone.imp().refresh_tabs();
            }
        });
        app.add_action(&new_tab_action);

        let tab_bar = window.imp().tab_bar.clone();
        tab_bar.connect_page_added({
            let window_clone = window.clone();
            move |_notebook, _, page_num| {
                window_clone
                    .imp()
                    .log(format!("added tab: {}", page_num).as_str());
            }
        });

        tab_bar.connect_page_removed({
            let window_clone = window.clone();
            move |_notebook, _, page_num| {
                window_clone
                    .imp()
                    .log(format!("removed tab: {}", page_num).as_str());
            }
        });

        tab_bar.connect_page_reordered({
            let window_clone = window.clone();
            move |_notebook, _, page_num| {
                window_clone
                    .imp()
                    .log(format!("reordered tab: {}", page_num).as_str());
            }
        });

        tab_bar.connect_switch_page({
            let window_clone = window.clone();
            move |_notebook, _, page_num| {
                window_clone
                    .imp()
                    .log(format!("switched to tab: {}", page_num).as_str());
                let mgr = window_clone.imp().tab_manager.lock().unwrap();
                let tab_id = mgr.page_to_tab(page_num);
                if let Some(tab_id) = tab_id {
                    mgr.set_active(tab_id);

                    let name = mgr.get_tab(tab_id).unwrap().name();
                    window_clone.set_title(Some(name));
                }
            }
        });
    }
}
