use std::cell::RefCell;
use std::rc::Rc;
use glib::subclass::InitializingObject;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, Entry, Button, Statusbar, CompositeTemplate, TextView};
use crate::tab::GosubTab;

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
    #[template_child]
    pub log: TemplateChild<TextView>,

    pub tabs: Rc<RefCell<Vec<GosubTab>>>,
}

impl Window {
    pub(crate) fn init_tabs(&self) {

        let mut tabs = Vec::new();
        tabs.push(GosubTab::new(String::from("https://duckduckgo.com")));
        tabs.push(GosubTab::new(String::from("https://news.ycombinator.com")));
        tabs.push(GosubTab::new(String::from("https://www.reddit.com")));
        tabs.push(GosubTab::new(String::from("https://www.gosub.io")));
        tabs.push(GosubTab::new(String::from("https://www.gosub.io")));
        tabs.push(GosubTab::new(String::from("https://www.gosub.io")));
        tabs.push(GosubTab::new(String::from("https://www.gosub.io")));
        tabs.push(GosubTab::new(String::from("https://www.gosub.io")));
        tabs.push(GosubTab::new(String::from("https://www.gosub.io")));
        tabs.push(GosubTab::new(String::from("https://www.gosub.io")));
        tabs.push(GosubTab::new(String::from("https://www.gosub.io")));

        self.tabs.replace(tabs);
    }

    fn log_stuff(&self) {
        let len = self.tabs.borrow().len();

        self.log(format!("There are {} tabs available", len).as_str());
    }
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
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

impl ObjectImpl for Window {
    fn constructed(&self) {
        self.parent_constructed();

        // self.tabs = Vec::new();
        // self.tabs.push(GosubTab::new(String::from("https://news.ycombinator.com")));
        // self.tabs.push(GosubTab::new(String::from("https://www.gosub.io")));
        // self.tabs.push(GosubTab::new(String::from("https://www.youtube.com")));
        // self.tabs.push(GosubTab::new(String::from("https://www.google.com")));


        // let menu_ui = Builder::from_string(include_str!("../../resources/main_menu.ui"));
        // let menu_model: gio::MenuModel = menu_ui.object("main-menu").expect("could not get main-menu");
        // let menu_bar = PopoverMenuBar::new();
        // menu_bar.bind_model(Some(&menu_model), Some("app"), true);s
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


#[gtk::template_callbacks]
impl Window {
    fn log(&self, message: &str) {
        let buf = self.log.buffer();
        let mut iter = buf.end_iter();
        buf.insert(&mut iter, format!("[{}] {}\n", chrono::Local::now().format("%X"), message).as_str());
    }

    #[template_callback]
    fn handle_prev_clicked(&self, _btn: &Button) {
        self.log("Going back to the previous page");

        self.log_stuff();

        self.statusbar.push(1, "We want to view the previous page");
    }

    #[template_callback]
    fn handle_refresh_clicked(&self, _btn: &Button) {
        self.log("Refreshing the current page");

        self.statusbar.push(1, "We want to refresh the current page");
    }

    #[template_callback]
    fn handle_searchbar_clicked(&self, entry: &Entry) {
        self.log(format!("Visiting the URL {}", entry.text().as_str()).as_str());
        self.statusbar.push(1, format!("Oh yeah.. full speed ahead to {}", entry.text().as_str()).as_str());
    }

    #[template_callback]
    fn handle_tab_add_clicked(&self) {
        self.log("Adding new tab");
    }
}

impl WidgetImpl for Window {}
impl WindowImpl for Window {}
impl ApplicationWindowImpl for Window {}