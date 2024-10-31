use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;
use glib::subclass::InitializingObject;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, Entry, Button, Statusbar, CompositeTemplate, TextView, ToggleButton, Notebook, Image};
use gtk::gdk_pixbuf::{Colorspace, Pixbuf};
use reqwest::blocking::Client;
use crate::tab::GosubTab;
use crate::{add_new_tab, toggle_dark_mode};

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
    /// Actual tabs information
    pub tabs: Rc<RefCell<Vec<GosubTab>>>,
}

impl BrowserWindow {
    #[allow(unused)]
    pub(crate) fn init_tabs(&self) {
        let mut tabs = Vec::new();

        let initial_tabs = [
            "https://duckduckgo.com",
            "https://news.ycombinator.com",
            "https://reddit.com",
            "https://gosub.io",
        ];

        for url in initial_tabs.iter() {
            // Load the favicon from the website
            let icon = download_favicon(url);
            let gt = GosubTab::new(url, icon);
            tabs.push(gt.clone());
            add_new_tab(self.tab_bar.clone(), gt.clone());
        }

        self.tabs.replace(tabs);
    }
}

fn download_favicon(url: &str) -> Option<Image> {
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:89.0) Gecko/20100101 Firefox/89.0")
        .timeout(Duration::from_secs(5))
        .build().ok()?;

    let response = client.get(format!("{}{}", url, "/favicon.ico")).send().ok()?;
    let buf = response.bytes().ok()?;

    let Ok(img) = image::load_from_memory(&buf) else {
        println!("Failed to load favicon into buffer (image)");
        return None;
    };

    // Convert to RGBA format if not already
    let rgba_image = img.to_rgba8();
    let (width, height) = rgba_image.dimensions();
    let pixels = rgba_image.into_raw();

    // Create a Pixbuf from the raw RGBA data
    let pixbuf = Pixbuf::from_mut_slice(
        pixels,
        Colorspace::Rgb,
        true, // Has alpha channel
        8,    // Bits per channel
        width as i32,
        height as i32,
        width as i32 * 4,
    );

    Some(Image::from_pixbuf(Some(&pixbuf)))
}

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

        add_new_tab(self.tab_bar.clone(), GosubTab::new("gosub:blank", None));
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
        toggle_dark_mode();
        self.statusbar.push(1, "We want to toggle dark mode");
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
}