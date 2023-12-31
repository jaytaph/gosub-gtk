mod imp;

use gtk::glib;
use gtk::prelude::*;

use gtk::gio;
use gtk::glib::subclass::prelude::*;
use gtk::Application;


glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Buildable;

        // @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        // @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
        //             gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;


}

impl Window {
    pub fn new(app: &Application) -> Self {
        // Create new window
        let window: Self = glib::Object::builder().property("application", app)
            .build();

        window.set_default_size(1000, 850);
        window.imp().init_tabs();

        window
    }
}


