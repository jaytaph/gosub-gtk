use adw::{prelude::*, subclass::prelude::*, ApplicationWindow};
use adw::gtk;
use std::cell::RefCell;
use std::rc::Rc;
use glib::subclass::InitializingObject;
use gtk4::{glib, Entry, Button, Statusbar, CompositeTemplate, TextView, ToggleButton, Notebook, Image, ScrolledWindow};
use log::info;
use uuid::Uuid;
use crate::tab::{GosubTab, GosubTabManager};
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
    pub log_scroller: TemplateChild<ScrolledWindow>,
    #[template_child]
    pub log: TemplateChild<TextView>,

    pub tab_manager: Rc<RefCell<GosubTabManager>>,
}

impl BrowserWindow {
    #[allow(unused)]
    pub(crate) fn init_tabs(&self) {
        let initial_tabs = [
            // "https://duckduckgo.com",
            // "https://news.ycombinator.com",
            // "https://reddit.com",
            "https://gosub.io",
        ];

        for url in initial_tabs.iter() {
            let icon = download_favicon(url);
            self.tab_manager.borrow_mut().add_tab(GosubTab::new(url, icon), None);
        }

        self.refresh_tabs();
    }
}

#[glib::object_subclass]
impl ObjectSubclass for BrowserWindow {
    const NAME: &'static str = "BrowserWindow";
    type Type = super::BrowserWindow;
    type ParentType = gtk4::ApplicationWindow;

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

#[gtk4::template_callbacks]
impl BrowserWindow {

    #[template_callback]
    fn handle_new_tab(&self, _btn: &Button) {
        self.log("Opening a new tab");
        self.statusbar.push(1, "We want to open a new tab");

        self.tab_manager.borrow_mut().add_tab(GosubTab::new("gosub:blank", None), None);
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

        if let Some(app) = self.obj().root()
            .and_then(|w| w.downcast::<ApplicationWindow>().ok())
            .and_then(|window| window.application()) {

            app.activate_action("toggle-dark-mode", None);
        }
    }

    #[template_callback]
    fn handle_refresh_clicked(&self, _btn: &Button) {
        self.log("Refreshing the current page");

        // self.obj().activate_action("refresh-tab", None);
        self.statusbar.push(1, "We want to refresh the current page");
    }

    #[template_callback]
    fn handle_searchbar_clicked(&self, entry: &Entry) {
        let Some(page) = self.tab_bar.current_page() else {
            let mut tab = GosubTab::new(entry.text().as_str(), None);
            tab.set_loading(true);
            self.tab_manager.borrow_mut().add_tab(tab, None);

            self.refresh_tabs();
            return
        };

        self.log(format!("We are currently on tab: {}", page).as_str());
        self.log(format!("Visiting the URL {}", entry.text().as_str()).as_str());
        self.statusbar.push(1, format!("Oh yeah.. full speed ahead to {}", entry.text().as_str()).as_str());

        let binding = entry.text();
        let url = if binding.starts_with("http://") || binding.starts_with("https://") {
            binding.to_string()
        } else {
            format!("https://{}", binding)
        };
        let icon = download_favicon(url.as_str());

        let mut manager = self.tab_manager.borrow_mut();
        let Some(tab) = manager.get_active_tab_mut() else {
            self.log("No tab selected, cannot navigate to URL");
            return
        };
        tab.set_url(url.as_str());
        tab.set_name(url.as_str());
        tab.set_favicon(icon);
        drop(manager);

        self.refresh_tabs();
    }
}


impl BrowserWindow {

    pub fn log(&self, message: &str) {
        let s = format!("[{}] {}\n", chrono::Local::now().format("%X"), message);
        info!("{}", s.as_str());

        let buf = self.log.buffer();
        let mut iter = buf.end_iter();
        buf.insert(&mut iter, s.as_str());

        let mark = buf.create_mark(None, &iter, false);
        self.log.scroll_to_mark(&mark, 0.0, true, 0.0, 1.0);
    }

    #[allow(dead_code)]
    pub(crate) fn close_tab(&self, tab_id: Uuid) {
        let mut manager = self.tab_manager.borrow_mut();
        manager.remove_tab(tab_id);
        drop(manager);

        self.refresh_tabs();
    }

    pub(crate) fn refresh_tabs(&self) {
        let manager = self.tab_manager.borrow();

        let mut page_num = 0;
        for tab_id in manager.order() {
            let tab = manager.get_tab(tab_id).unwrap();

            let label = self.create_label(tab);
            if self.tab_bar.pages().n_items() <= page_num {
                // add new tab
                let default_page = self.default_page();
                self.tab_bar.append_page(&default_page, Some(&label));
            } else {
                // update existing tab
                let page_child = self.tab_bar.nth_page(Some(page_num)).unwrap();
                self.tab_bar.set_tab_label(&page_child, Some(&label));
            }

            page_num += 1;
        }
    }

    // Create a new tab label
    pub fn create_label(&self, tab: &GosubTab) -> gtk::Box {
        let label_vbox = gtk::Box::new(gtk::Orientation::Horizontal, 5);

        // When the tab is loading, we show a spinner
        if tab.is_loading() {
            let spinner = gtk::Spinner::new();
            spinner.start();
            label_vbox.append(&spinner);
        } else if let Some(favicon) = &tab.favicon() {
            label_vbox.append(&Image::from_pixbuf(Some(&favicon.clone())));
        }

        // Only show the title and close button if the tab is not sticky
        if ! tab.is_sticky() {
            let tab_label = gtk::Label::new(Some(tab.name()));
            label_vbox.append(&tab_label);

            let tab_btn = gtk::Button::builder()
                .halign(gtk::Align::End)
                .has_frame(false)
                .margin_bottom(0)
                .margin_end(0)
                .margin_start(0)
                .margin_top(0)
                .build();
            let img = Image::from_icon_name("window-close-symbolic");
            tab_btn.set_child(Some(&img));

            let window_clone = self.obj().clone();
            let tab_clone = tab.clone();
            tab_btn.connect_clicked( move | _btn| {
                info!("Clicked close button for tab {}", tab_clone.id());
                let imp = window_clone.imp(); // `imp()` is now accessible on `ApplicationWindow`
                imp.close_tab(tab_clone.id());
                imp.refresh_tabs();
            });

            label_vbox.append(&tab_btn);
        }

        label_vbox
    }

    fn default_page(&self) -> gtk::Box {
        let img = gtk::Image::from_resource("/io/gosub/browser-gtk/assets/submarine.svg");
        img.set_visible(true);
        img.set_focusable(false);
        img.set_valign(gtk::Align::End);
        img.set_margin_top(64);
        img.set_pixel_size(500);


        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
        vbox.set_visible(true);
        vbox.set_can_focus(false);
        vbox.set_halign(gtk::Align::Center);
        vbox.set_orientation(gtk::Orientation::Vertical);
        vbox.set_vexpand(true);
        vbox.set_hexpand(true);

        vbox.append(&img);

        vbox
    }
}

