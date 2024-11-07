use adw::{prelude::*, subclass::prelude::*, ApplicationWindow};
use adw::gtk;
use std::sync::Arc;
use adw::glib::subclass::Signal;
use glib::subclass::InitializingObject;
use gtk4::{glib, Entry, Button, Statusbar, CompositeTemplate, TextView, ToggleButton, Notebook, Image, ScrolledWindow};
use gtk4::glib::clone;
use log::info;
use once_cell::sync::Lazy;
use tokio::task::LocalSet;
use std::sync::Mutex;
use uuid::Uuid;
use crate::tab::{GosubTab, GosubTabManager, TabCommand};
use crate::fetcher::{fetch_favicon, fetch_url_body};

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
    /// Tab manager to handle all tabs
    pub tab_manager: Arc<Mutex<GosubTabManager>>,
}

impl BrowserWindow {
    pub(crate) fn init_tabs(&self) {
        let initial_tabs = [
            // "https://duckduckgo.com",
            "https://news.ycombinator.com",
            // "https://reddit.com",
            // "https://gosub.io",
        ];

        let mut tab_ids = Vec::new();

        let mut manager = self.tab_manager.lock().unwrap();

        for url in initial_tabs.iter() {
            let tab_id = manager.add_tab(GosubTab::new(url, None), None);
            tab_ids.push(tab_id);
        }
        drop(manager);

        self.refresh_tabs();
        // self.obj().emit_by_name::<()>("update-tabs", &[]);

        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();


        let mut set = LocalSet::new();


        for tab_id in tab_ids {
            self.async_load_favicon(&mut set, tab_id);
            self.async_load_url(&mut set, tab_id);
        }

        rt.block_on(set)
    }
}


//
// #[derive(glib::SharedBoxed)]
// struct Favicon(Vec<u8>, u32, u32);
//
//
// enum Event {
//     SetFavicon {
//         buf: Vec<u8>,
//         width: u32,
//         height: u32,
//     },
//     Alert {
//         message: String
//     },
//
// }
//


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
        self.statusbar.push(1, "Ready to roll...");
    }

    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            vec![
                Signal::builder("update-tabs")
                    .build(),
            ]
        });

        SIGNALS.as_ref()
    }
}

impl WidgetImpl for BrowserWindow {}
impl WindowImpl for BrowserWindow {}
impl ApplicationWindowImpl for BrowserWindow {}

#[gtk4::template_callbacks]
impl BrowserWindow {

    #[template_callback]
    fn handle_new_tab(&self, _btn: &Button) {
        todo!("not yet implemented");
    }

    #[template_callback]
    fn handle_close_tab(&self, _btn: &Button) {
        todo!("not yet implemented");
    }

    #[template_callback]
    fn handle_prev_clicked(&self, _btn: &Button) {
        todo!("not yet implemented");
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
        self.statusbar.push(1, "We want to refresh the current page");
    }

    #[template_callback]
    fn handle_searchbar_clicked(&self, entry: &Entry) {
        let tab_id = self.tab_manager.lock().unwrap().get_active_tab().unwrap().id().clone();

        self.log(format!("Visiting the URL {}", entry.text().as_str()).as_str());
        self.statusbar.push(1, format!("Oh yeah.. full speed ahead to {}", entry.text().as_str()).as_str());

        let binding = entry.text();
        let url = if binding.starts_with("http://") || binding.starts_with("https://") {
            binding.to_string()
        } else {
            format!("https://{}", binding)
        };

        let mut mgr = self.tab_manager.lock().unwrap();
        let tab = mgr.get_tab_mut(tab_id).unwrap();
        tab.set_url(url.as_str());

        mgr.notify_tab_changed(tab_id);
        drop(mgr);

        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let mut set = LocalSet::new();

        self.async_load_favicon(&mut set, tab_id);
        self.async_load_url(&mut set, tab_id);

        rt.block_on(set);
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
        let mut manager = self.tab_manager.lock().unwrap();
        if manager.tab_count() == 1 {
            self.log("Cannot close the last tab");
            return
        }
        manager.remove_tab(tab_id);
    }
    pub(crate) fn refresh_tabs(&self) {
        let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

        rt.block_on(self.refresh_tabs_async())
    }

    pub(crate) async fn refresh_tabs_async(&self) {


        let manager = self.tab_manager.lock().unwrap();
        let commands = manager.commands();
        drop(manager);

        for cmd in commands {
            println!("Processing command: {:?}", cmd);
            match cmd {
                TabCommand::Activate(page_num) => {
                    self.tab_bar.set_current_page(Some(page_num));
                }
                TabCommand::Insert(page_num) => {
                    let manager = self.tab_manager.lock().unwrap();
                    let tab = manager.get_tab(manager.page_to_tab(page_num).unwrap()).unwrap().clone();
                    drop(manager);

                    let label = self.generate_label(&tab);
                    let default_page = self.generate_default_page();
                    self.tab_bar.insert_page(&default_page, Some(&label), Some(page_num));
                }
                TabCommand::Close(page_num) => {
                    self.tab_bar.remove_page(Some(page_num));
                }
                TabCommand::CloseAll => {
                    for _ in 0..self.tab_bar.pages().n_items() {
                        self.tab_bar.remove_page(Some(0));
                    }
                }
                TabCommand::Move(src, dst) => {
                    let page = self.tab_bar.nth_page(Some(src)).unwrap();
                    self.tab_bar.reorder_child(&page, Some(dst));
                }
                TabCommand::Pin(_) => {}
                TabCommand::Unpin(_) => {}
                TabCommand::Private(_) => {}
                TabCommand::Update(page_num) => {
                    let manager = self.tab_manager.lock().unwrap();
                    let tab = manager.get_tab(manager.page_to_tab(page_num).unwrap()).unwrap().clone();
                    drop(manager);
                    let label = self.generate_label(&tab);
                    let page_child = self.tab_bar.nth_page(Some(page_num)).unwrap();
                    self.tab_bar.set_tab_label(&page_child, Some(&label));
                }
            }
        }
    }

    // generate tab label
    pub fn generate_label(&self, tab: &GosubTab) -> gtk::Box {
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

    fn generate_default_page(&self) -> gtk4::Box {
        let img = Image::from_resource("/io/gosub/browser-gtk/assets/submarine.svg");
        img.set_visible(true);
        img.set_focusable(false);
        img.set_valign(gtk::Align::Center);
        img.set_margin_top(64);
        img.set_pixel_size(500);
        img.set_hexpand(true);

        let vbox = gtk4::Box::new(gtk::Orientation::Vertical, 0);
        vbox.set_visible(true);
        vbox.set_can_focus(false);
        vbox.set_halign(gtk::Align::Center);
        vbox.set_vexpand(true);
        vbox.set_hexpand(true);

        vbox.append(&img);

        vbox
    }

    fn async_load_url(&self, set: &mut LocalSet, tab_id: Uuid) {
        set.spawn_local(clone!(
            #[weak(rename_to=window)]
            self,
            async move {
                info!("Fetching URL for tab: {}", tab_id);
                window.load_url(tab_id).await;
            }
        ));
    }

    fn async_load_favicon(&self, set: &mut LocalSet, tab_id: Uuid) {
        set.spawn_local(clone!(
            #[weak(rename_to=window)]
            self,
            async move {
                info!("Fetching favicon for tab: {}", tab_id);

                {
                    let mut manager = window.tab_manager.lock().unwrap();
                    let mut tab = manager.get_tab(tab_id).unwrap().clone();
                    tab.set_loading(true);
                    manager.update_tab(tab_id, &tab);
                }
                window.refresh_tabs_async().await;

                let url = {
                    let tab = window.tab_manager.lock().unwrap().get_tab(tab_id).unwrap().clone();
                    tab.url().to_string()
                };

                let icon_pixbuf = fetch_favicon(&url).await;
                info!("Fetched favicon for tab: {}", tab_id);

                {
                    let mut manager = window.tab_manager.lock().unwrap();
                    let mut tab = manager.get_tab(tab_id).unwrap().clone();
                    tab.set_favicon(icon_pixbuf);
                    tab.set_loading(false);
                    manager.update_tab(tab_id, &tab);
                }

                // Repaint the tabs
                info!("Favicon Emitting update-tabs signal");


                window.refresh_tabs_async().await;
                // window.obj().emit_by_name::<()>("update-tabs", &[]);
            }
        ));
    }

    async fn load_url(&self, tab_id: Uuid) {
        let mut mgr = self.tab_manager.lock().unwrap();
        let tab = mgr.get_tab_mut(tab_id).unwrap();
        let url = tab.url().to_string();

        // Add / update tab and set the spinner to loading
        mgr.notify_tab_changed(tab_id);
        self.log(format!("Loading URL: {}", url).as_str());

        drop(mgr);

        self.log(format!("Fetching URL: {}", url).as_str());
        match fetch_url_body(&url).await {
            Ok(body) => {
                self.log(format!("Fetched URL: {}", url).as_str());
                let mut mgr = self.tab_manager.lock().unwrap();
                let tab = mgr.get_tab_mut(tab_id).unwrap();
                let body = String::from_utf8(body).unwrap();
                tab.set_content(body);
            }
            Err(e) => {
                self.log(format!("Failed to fetch URL: {}", e).as_str());
            }
        }

        let mut mgr = self.tab_manager.lock().unwrap();
        let tab = mgr.get_tab_mut(tab_id).unwrap();
        tab.set_loading(false);
        mgr.notify_tab_changed(tab_id);
    }
}

