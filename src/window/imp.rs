use adw::{prelude::*, subclass::prelude::*, ColorScheme, StyleManager};
use adw::gtk;
use std::sync::Arc;
use adw::glib::subclass::Signal;
use glib::subclass::InitializingObject;
use gtk4::{glib, Entry, Button, Statusbar, CompositeTemplate, TextView, ToggleButton, Notebook, Image, ScrolledWindow};
use log::info;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use async_channel::{Receiver, Sender};
use uuid::Uuid;
use crate::tab::{GosubTab, GosubTabManager, TabCommand};
use crate::{fetcher, runtime};
use crate::utils::convert_to_pixbuf;
use crate::window::message::Message;

#[derive(CompositeTemplate)]
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

    // Other stuff that are non-widgets
    pub tab_manager: Arc<Mutex<GosubTabManager>>,
    pub sender: Arc<Sender<Message>>,
    pub receiver: Arc<Receiver<Message>>,
}

impl Default for BrowserWindow {
    fn default() -> Self {
        let (tx, rx) = async_channel::unbounded::<Message>();
        Self {
            searchbar: TemplateChild::default(),
            tab_bar: TemplateChild::default(),
            statusbar: TemplateChild::default(),
            log_scroller: TemplateChild::default(),
            log: TemplateChild::default(),

            tab_manager: Arc::new(Mutex::new(GosubTabManager::new())),
            sender: Arc::new(tx),
            receiver: Arc::new(rx),
        }
    }
}

impl BrowserWindow {
    pub(crate) fn get_sender(&self) -> Arc<Sender<Message>> {
        self.sender.clone()
    }

    pub(crate) fn get_receiver(&self) -> Arc<Receiver<Message>> {
        self.receiver.clone()
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
    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            vec![
                Signal::builder("update-tabs")
                    .build(),
            ]
        });

        SIGNALS.as_ref()
    }

    fn constructed(&self) {
        self.parent_constructed();
        self.log("Browser created...");
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

        info!("Toggle dark mode action triggered");
        let mgr = StyleManager::default();
        if mgr.is_dark() {
            mgr.set_color_scheme(ColorScheme::ForceLight);
        } else {
            mgr.set_color_scheme(ColorScheme::ForceDark);
        }
    }

    #[template_callback]
    fn handle_refresh_clicked(&self, _btn: &Button) {
        self.log("Refreshing the current page");
        self.statusbar.push(1, "We want to refresh the current page");
    }

    #[template_callback]
    async fn handle_searchbar_clicked(&self, entry: &Entry) {
        let tab_id = self.tab_manager.lock().unwrap().get_active_tab().unwrap().id().clone();

        self.log(format!("Visiting the URL {}", entry.text().as_str()).as_str());
        self.statusbar.push(1, format!("Oh yeah.. full speed ahead to {}", entry.text().as_str()).as_str());

        let binding = entry.text();
        if binding.starts_with("about:") {
            // About: pages are special, we don't need to prefix them with a protocol
            self.sender.send(Message::LoadUrl(tab_id, binding.to_string())).await.unwrap();
            return;
        } else if binding.starts_with("http://") || binding.starts_with("https://") {
            // https:// and http:// protocols are loaded as-is
            self.sender.send(Message::LoadUrl(tab_id, binding.to_string())).await.unwrap();
        } else {
            // No protocol, we use https:// as a prefix
            let url = format!("https://{}", binding);
            self.sender.send(Message::LoadUrl(tab_id, url)).await.unwrap();
        };

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

    async fn refresh_tabs_async(&self) {
        let mut manager = self.tab_manager.lock().unwrap();
        let commands = manager.commands();
        drop(manager);

        println!("Entering refresh_tabs_async ------------------------------------");

        dbg!(&commands);

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

                    let label = self.create_tab_label(tab.is_loading(), &tab);
                    let default_page = self.generate_default_page();
                    let page_id = self.tab_bar.insert_page(&default_page, Some(&label), Some(page_num));

                    // We can reorder tab, unless it's pinned/sticky
                    if let Some(page) = self.tab_bar.nth_page(Some(page_id)) {
                        self.tab_bar.set_tab_reorderable(&page, !tab.is_sticky());
                    }
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

                    let scrolled_window = gtk4::ScrolledWindow::builder()
                        .hscrollbar_policy(gtk4::PolicyType::Never)
                        .vscrollbar_policy(gtk4::PolicyType::Automatic)
                        .vexpand(true)
                        .build();

                    let content = TextView::builder()
                        .editable(false)
                        .wrap_mode(gtk4::WrapMode::Word)
                        .build();
                    content.buffer().set_text(&tab.content());
                    scrolled_window.set_child(Some(&content));

                    let tab_label = self.create_tab_label(false, &tab);

                    // We need to remove the page, and read it in order to change the page content. Also,
                    // we must make sure we select the tab again.
                    self.tab_bar.remove_page(Some(page_num));
                    let page_id = self.tab_bar.insert_page(&scrolled_window, Some(&tab_label), Some(page_num));
                    self.tab_bar.set_current_page(Some(page_num));

                    // We can reorder tab, unless it's pinned/sticky
                    if let Some(page) = self.tab_bar.nth_page(Some(page_id)) {
                        self.tab_bar.set_tab_reorderable(&page, !tab.is_sticky());
                    }

                }
            }
        }

        let mut manager = self.tab_manager.lock().unwrap();
        let commands = manager.commands();
        drop(manager);
        dbg!(&commands);

        println!("Exiting refresh_tabs_async ------------------------------------");
    }

    /// generates a tab label based on the tab info
    fn create_tab_label(&self, is_loading: bool, tab: &GosubTab) -> gtk::Box {
        let label_vbox = gtk::Box::new(gtk::Orientation::Horizontal, 5);

        // When the tab is loading, we show a spinner
        if is_loading {
            let spinner = gtk::Spinner::new();
            spinner.start();
            label_vbox.append(&spinner);
        } else if let Some(favicon) = &tab.favicon() {
            label_vbox.append(&Image::from_pixbuf(Some(&favicon.clone())));
        }

        // Only show the title and close button if the tab is not sticky
        if !tab.is_sticky() {
            let tab_label = gtk::Label::new(Some(tab.title()));
            label_vbox.append(&tab_label);

            let tab_btn = Button::builder()
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
            let tab_id = tab.id().clone();
            tab_btn.connect_clicked(move |_| {
                info!("Clicked close button for tab {}", tab_id);
                window_clone.imp().close_tab(tab_id);
                _ = window_clone.imp().get_sender().send_blocking(Message::RefreshTabs());
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

    fn load_favicon_async(&self, tab_id: Uuid) {
        info!("Fetching favicon for tab: {}", tab_id);

        let manager = self.tab_manager.lock().unwrap();
        let tab = manager.get_tab(tab_id).unwrap();
        let url = tab.url().to_string();
        drop(manager);

        let sender_clone = self.get_sender().clone();
        runtime().spawn(async move {
            let favicon = if url.starts_with("about:") {
                // about: pages do not have a favicon (or maybe a default one?)
                Vec::new()
            } else {
                fetcher::fetch_favicon(url.as_str()).await
            };
            sender_clone.send(Message::FaviconLoaded(tab_id, favicon)).await.unwrap();
        });
    }

    fn load_url_async(&self, tab_id: Uuid) {
        let manager = self.tab_manager.lock().unwrap();
        let tab = manager.get_tab(tab_id).unwrap();
        let url = tab.url().to_string();
        drop(manager);

        let sender_clone = self.get_sender().clone();
        runtime().spawn(async move {
            if url.starts_with("about:") {
                let html_content = load_about_url(url);
                sender_clone.send(Message::UrlLoaded(tab_id, html_content)).await.unwrap();
                return;
            }

            match fetcher::fetch_url_body(&url).await {
                Ok(content) => {
                    let html_content = String::from_utf8_lossy(content.as_slice());
                    // we get a Cow.. and we clone it into the url?
                    sender_clone.send(Message::UrlLoaded(tab_id, html_content.to_string())).await.unwrap();
                }
                Err(e) => {
                    log::error!("Failed to fetch URL: {}", e);
                    sender_clone.send(Message::Log(format!("Failed to fetch URL: {}", e))).await.unwrap();
                }
            }
        });
    }

    /// Handles all message coming from the async (tokio) tasks
    pub async fn handle_message(&self, message: Message) {
        info!("Received a message: {:?}", message);

        match message {
            Message::RefreshTabs() => {
                self.refresh_tabs();
            }
            Message::OpenTab(url) => {
                let tab = GosubTab::new(url.as_str(), url.as_str());
                let tab_id = tab.id();

                // let tab_label = self.create_tab_label(true, &tab);
                // let page_num = self.tab_bar.append_page(
                //     &self.generate_default_page(),
                //     Some(&tab_label),
                // );
                // self.tab_bar.set_current_page(Some(page_num));

                let mut manager = self.tab_manager.lock().unwrap();
                manager.add_tab(tab, None);
                manager.notify_tab_changed(tab_id);
                drop(manager);
                self.refresh_tabs();

                self.load_favicon_async(tab_id);
                self.load_url_async(tab_id);
            }
            Message::LoadUrl(tab_id, url) => {
                self.log("Loading URL...");

                // Update information in the given tab with the new url
                let mut manager = self.tab_manager.lock().unwrap();
                let mut tab = manager.get_tab(tab_id).unwrap().clone();

                let _page_num = manager.get_page_num_by_tab(tab_id).unwrap();
                tab.set_favicon(None);
                tab.set_title(url.as_str());
                tab.set_url(url.as_str());
                tab.set_loading(true);

                manager.update_tab(tab_id, &tab);
                drop(manager);

                // // Create loading label and add it to the tab bar
                // let tab_label = self.create_tab_label(true, &tab);
                // let page_child = self.tab_bar.nth_page(Some(page_num)).unwrap();
                // self.tab_bar.set_tab_label(&page_child, Some(&tab_label));

                self.refresh_tabs();

                // Now, load favicon and url content
                self.load_favicon_async(tab_id);
                self.load_url_async(tab_id);

            }
            Message::FaviconLoaded(tab_id, favicon) => {
                if favicon.is_empty() {
                    self.log(format!("no favicon found for tab {}", tab_id).as_str());
                    return;
                }

                let manager = self.tab_manager.lock().unwrap();
                let mut tab = manager.get_tab(tab_id).unwrap().clone();
                drop(manager);

                match convert_to_pixbuf(favicon.as_slice()) {
                    Ok(pixbuf) => tab.set_favicon(Some(pixbuf)),
                    Err(e) => {
                        log::error!("Failed to convert favicon to pixbuf: {}", e);
                        self.log(format!("Failed to convert favicon to pixbuf: {}", e).as_str());
                    }
                }

                let mut manager = self.tab_manager.lock().unwrap();
                manager.update_tab(tab_id, &tab);
                drop(manager);

                self.refresh_tabs();

                // let tab_label = self.create_tab_label(false, &tab);
                //
                // let manager = self.tab_manager.lock().unwrap();
                // let page_num = manager.get_page_num_by_tab(tab_id).unwrap();
                // drop(manager);
                //
                // let page_child = self.tab_bar.nth_page(Some(page_num)).unwrap();
                // self.tab_bar.set_tab_label(&page_child, Some(&tab_label));
            }
            Message::UrlLoaded(tab_id, html_content) => {
                let mut manager = self.tab_manager.lock().unwrap();
                let mut tab = manager.get_tab(tab_id).unwrap().clone();
                tab.set_content(html_content.clone());
                tab.set_loading(false);
                manager.update_tab(tab_id, &tab);
                drop(manager);

                self.refresh_tabs();
            }
            Message::Log(msg) => {
                self.log(msg.as_str());
            }
        }
    }
}

fn load_about_url(url: String) -> String {
    match url.as_str() {
        "about:blank" => {
            r#"
            <html>
                <head>
                    <title>Blank page</title>
                </head>
                <body>
                    <h1>Blank page</h1>
                    <p>This is a blank page</p>
                </body>
            </html>
            "#
            .to_string()
        }
        _ => {
            r#"
            <html>
                <head>
                    <title>Unknown about: page</title>
                </head>
                <body>
                    <h1>Unknown about: page</h1>
                    <p>This is an unknown about: page</p>
                </body>
            </html>
            "#
            .to_string()
        }
    }
}