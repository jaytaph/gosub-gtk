use std::cell::RefCell;
use std::rc::Rc;
use glib::subclass::InitializingObject;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, Entry, Button, Statusbar, CompositeTemplate, TextView, ToggleButton, Notebook};
use crate::tab::{GosubTab, GosubTabManager};
use crate::dialog::about::About;
use crate::favicon::download_favicon;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/io/gosub/browser-gtk/ui/window.ui")]
pub struct BrowserWindow {
    #[template_child]
    pub searchbar: TemplateChild<Entry>,
    #[template_child]
    pub tab_bar: TemplateChild<Notebook>,
    #[template_child]
    pub statusbar: TemplateChild<Statusbar>,
    #[template_child]
    pub log: TemplateChild<TextView>,

    pub tab_manager: Rc<RefCell<GosubTabManager>>,
}

impl BrowserWindow {
    #[allow(unused)]
    pub(crate) fn init_tabs(&self) {
        let initial_tabs = [
            "https://duckduckgo.com",
            "https://news.ycombinator.com",
            "https://reddit.com",
            "https://gosub.io",
        ];

        for url in initial_tabs.iter() {
            let icon = download_favicon(url);
            self.tab_manager.borrow_mut().add_tab(GosubTab::new(url, icon));
        }

        // self.tabs.replace(tabs);
    }
}


// let notebook_clone = tab_bar.clone();
// tab_btn.connect_clicked(move |_| {
//     notebook_clone.remove_page(Some(page_index));
// });


#[glib::object_subclass]
impl ObjectSubclass for BrowserWindow {
    const NAME: &'static str = "BrowserWindow";
    type Type = super::BrowserWindow;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for BrowserWindow {
    fn constructed(&self) {
        self.parent_constructed();

        self.log("Browser created...");

        // Initialize the status bar
        self.statusbar.push(1, "Ready to roll...");
    }
}

impl WidgetImpl for BrowserWindow {}

impl WindowImpl for BrowserWindow {}

impl ApplicationWindowImpl for BrowserWindow {}

#[gtk::template_callbacks]
impl BrowserWindow {

    #[template_callback]
    fn handle_new_tab(&self, _btn: &Button) {
        self.log("Opening a new tab");
        self.statusbar.push(1, "We want to open a new tab");

        self.tab_manager.borrow_mut().add_tab(GosubTab::new("gosub:blank", None));
    }

    #[template_callback]
    fn handle_close_tab(&self, _btn: &Button) {
        self.log("Closing the current tab");
        self.statusbar.push(1, "We want to close the current tab");
    }

    #[template_callback]
    fn handle_prev_clicked(&self, _btn: &Button) {
        self.log("Going back to the previous page");
        self.statusbar.push(1, "We want to view the previous page");
    }

    #[template_callback]
    fn handle_toggle_darkmode(&self, _btn: &ToggleButton) {
        self.log("Toggling dark mode");
        self.toggle_dark_mode();
        self.statusbar.push(1, "We want to toggle dark mode");
    }

    #[template_callback]
    fn handle_refresh_clicked(&self, _btn: &Button) {
        self.log("Refreshing the current page");
        self.statusbar.push(1, "We want to refresh the current page");
    }

    #[template_callback]
    fn handle_searchbar_clicked(&self, entry: &Entry) {
        let Some(page) = self.tab_bar.current_page() else {
            let mut tab = GosubTab::new(entry.text().as_str(), None);
            tab.set_loading(true);
            self.tab_manager.borrow_mut().add_tab(tab);

            self.refresh_tabs();
            return
        };

        self.log(format!("We are currently on tab: {}", page).as_str());
        self.log(format!("Visiting the URL {}", entry.text().as_str()).as_str());
        self.statusbar.push(1, format!("Oh yeah.. full speed ahead to {}", entry.text().as_str()).as_str());

        let binding = entry.text();
        let url = binding.as_str();
        let icon = download_favicon(url);

        let mut manager = self.tab_manager.borrow_mut();
        let Some(tab) = manager.get_active_tab_mut() else {
            self.log("No tab selected, cannot navigate to URL");
            return
        };

        tab.set_url(url);
        tab.set_favicon(icon);
        tab.mark_as_dirty();

        self.refresh_tabs();

        // update_current_tab(self.tab_bar.clone(), tab);
    }

    // #[template_callback]
    // fn handle_tab_add_clicked(&self) {
    //     self.log("Adding new tab");
    //     self.add_tab();
    // }
}


impl BrowserWindow {

    fn log(&self, message: &str) {
        let buf = self.log.buffer();
        let mut iter = buf.end_iter();
        buf.insert(&mut iter, format!("[{}] {}\n", chrono::Local::now().format("%X"), message).as_str());
    }


    pub(crate) fn show_about_dialog(&self) {
        let about = About::new();
        about.show();
    }

    pub(crate) fn toggle_dark_mode(&self) {
        if let Some(settings) = gtk::Settings::default() {
            let is_dark = settings.is_gtk_application_prefer_dark_theme();
            settings.set_gtk_application_prefer_dark_theme(!is_dark);
        }
    }

    pub(crate) fn refresh_tabs(&self) {
        let manager = self.tab_manager.borrow();
        let tabs = manager.tabs();

        for (index, tab) in tabs.iter().enumerate() {
            let label = create_label(tab);
            self.tab_bar.set_tab_label_text(self.tab_bar.nth_page(Some(index)).unwrap(), tab.url());
        }
    }


}