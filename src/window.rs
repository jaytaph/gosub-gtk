use gtk::glib;

mod imp;
use gtk::gio;
use gtk::Application;
use gtk::subclass::prelude::ObjectSubclassIsExt;

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

        window.imp().init_tabs();

        window
    }
}


