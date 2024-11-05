use adw::gtk;
use gtk::glib;

mod imp;

use gtk::gio;
use gtk::prelude::GtkWindowExt;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use gtk::prelude::*;
use crate::application::Application;

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

        window.imp().init_tabs();

        // // Dark mode toggle
        // let action = SimpleAction::new_stateful("toggle_darkmode", None, &false.to_variant());
        // action.connect_activate({
        //     let window_clone = window.clone();
        //     move |action, _| {
        //         let is_dark_mode = action.state().unwrap().get::<bool>().unwrap();
        //         action.set_state(&(!is_dark_mode).to_variant());
        //
        //         window_clone.imp().toggle_dark_mode();
        //     }
        // });
        // app.add_action(&action);
        //
        // // About action
        // let about_action = SimpleAction::new("about", None);
        // about_action.connect_activate({
        //     let window_clone = window.clone();
        //     move |_, _| {
        //         window_clone.imp().show_about_dialog();
        //     }
        // });
        // app.add_action(&about_action);
        // app.set_accels_for_action("app.about", &["<Primary>A"]);
        //
        //
        // let logwindow_action = SimpleAction::new("log.toggle", None);
        // logwindow_action.connect_activate({
        //     let window_clone = window.clone();
        //     move |_, _| {
        //         window_clone.imp().log_scroller.set_visible(!window_clone.imp().log_scroller.get_visible());
        //     }
        // });
        // app.add_action(&logwindow_action);
        // app.set_accels_for_action("app.log.toggle", &["<Primary>L"]);
        //
        // // Create new tab
        // let new_tab_action = SimpleAction::new("tab.new", None);
        // new_tab_action.connect_activate({
        //     let window_clone = window.clone();
        //     let tab_manager = window.imp().tab_manager.clone();
        //     move |_, _| {
        //         let tab_data = GosubTab::new("gosub:blank", None);
        //         tab_manager.borrow_mut().add_tab(tab_data, None);
        //         window_clone.imp().refresh_tabs();
        //     }
        // });
        // app.add_action(&new_tab_action);
        // app.set_accels_for_action("app.tab.new", &["<Primary>T"]);
        //
        // let tab_bar = window.imp().tab_bar.clone();
        // tab_bar.connect_page_added({
        //     let window_clone = window.clone();
        //     move |_notebook, _, page_num| {
        //         window_clone.imp().log(format!("added tab: {}", page_num).as_str());
        //     }
        // });
        //
        // tab_bar.connect_page_removed({
        //     let window_clone = window.clone();
        //     move |_notebook, _, page_num| {
        //         window_clone.imp().log(format!("removed tab: {}", page_num).as_str());
        //     }
        // });
        //
        // tab_bar.connect_page_reordered({
        //     let window_clone = window.clone();
        //     move |_notebook, _, page_num| {
        //         window_clone.imp().log(format!("reordered tab: {}", page_num).as_str());
        //     }
        // });
        //
        // tab_bar.connect_switch_page({
        //     let window_clone = window.clone();
        //     move |_notebook, _, page_num| {
        //         window_clone.imp().log(format!("switched to tab: {}", page_num).as_str());
        //         let mgr = window_clone.imp().tab_manager.borrow();
        //         let tab_id = mgr.page_to_tab(page_num);
        //         if let Some(tab_id) = tab_id {
        //             mgr.set_active(tab_id);
        //         }
        //     }
        // });
        //
        // // Called when a close-tab button is pressed
        // tab_bar.connect_local("close_tab", false, move |args| {
        //     let window_clone = window.clone();
        //     let tab_id = args[0].get::<String>().unwrap();
        //     if let Ok(tab_id) = uuid::Uuid::parse_str(&tab_id) {
        //         window_clone.imp().close_tab(tab_id);
        //         window_clone.imp().refresh_tabs();
        //     }
        //     None
        // });

        window
    }
}

