use glib::subclass::InitializingObject;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, Entry, Button, Statusbar, CompositeTemplate};

// ANCHOR: object
// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/io/gosub/browser-gtk/window.ui")]
pub struct Window {
    #[template_child]
    pub searchbar: TemplateChild<Entry>,
    #[template_child]
    pub tab_1: TemplateChild<Button>,
    #[template_child]
    pub tab_2: TemplateChild<Button>,
    #[template_child]
    pub tab_3: TemplateChild<Button>,
    #[template_child]
    pub tab_4: TemplateChild<Button>,
    #[template_child]
    pub tab_add: TemplateChild<Button>,
    #[template_child]
    pub statusbar: TemplateChild<Statusbar>
}
// ANCHOR_END: object

// ANCHOR: subclass
// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Window {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "GosubMainWindow";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}
// ANCHOR_END: subclass

// ANCHOR: object_impl
// Trait shared by all GObjects
impl ObjectImpl for Window {
    fn constructed(&self) {
        // Call "constructed" on parent
        self.parent_constructed();

        self.statusbar.push(1, "Ready to roll...");

        let statusbar = self.statusbar.clone();

        self.tab_1.connect_clicked(move |button| {
            statusbar.push(2, "hell world should be visible in the button now");

            // Set the label to "Hello World!" after the button has been clicked on
            button.set_label("Hello World!");
        });
    }
}
// ANCHOR_END: object_impl

// Trait shared by all widgets
impl WidgetImpl for Window {}

// Trait shared by all windows
impl WindowImpl for Window {}

// Trait shared by all application windows
impl ApplicationWindowImpl for Window {}