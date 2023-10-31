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
    pub statusbar: TemplateChild<Statusbar>,
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
        klass.bind_template_callbacks();
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

        // let menu_ui = Builder::from_string(include_str!("../../resources/main_menu.ui"));
        // let menu_model: gio::MenuModel = menu_ui.object("main-menu").expect("could not get main-menu");
        // let menu_bar = PopoverMenuBar::new();
        // menu_bar.bind_model(Some(&menu_model), Some("app"), true);
        //
        // let main_box: gtk::Box = self.object("main_box").expect("could not find main_box");
        // main_box.append(&menu_bar);

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


#[gtk::template_callbacks]
impl Window {
    #[template_callback]
    fn handle_prev_clicked(&self, _btn: &Button) {
        self.statusbar.push(1, "We want to view the previous page");
    }

    #[template_callback]
    fn handle_refresh_clicked(&self, _btn: &Button) {
        self.statusbar.push(1, "We want to refresh the current page");
    }
}

// Trait shared by all widgets
impl WidgetImpl for Window {}

// Trait shared by all windows
impl WindowImpl for Window {}

// Trait shared by all application windows
impl ApplicationWindowImpl for Window {}