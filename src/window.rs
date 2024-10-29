mod imp;

use gtk::glib;
use gtk::gio;
use gtk::Application;
use gtk::traits::GtkWindowExt;

glib::wrapper! {
    pub struct BrowserWindow(ObjectSubclass<imp::BrowserWindow>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl BrowserWindow {
    pub fn new(app: &Application) -> Self {
        // Create new window
        let window: Self = glib::Object::builder().property("application", app)
            .build();

        window.set_default_size(1000, 850);
        // window.imp().init_tabs();

        window
    }
}

