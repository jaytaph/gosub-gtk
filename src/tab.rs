use std::collections::HashMap;
use gtk::{Image};
use gtk::prelude::{BoxExt, ButtonExt};
use uuid::Uuid;

#[derive(Clone)]
pub struct GosubTab {
    /// Id of the tab
    id: uuid::Uuid,
    /// Tab is sticky and cannot be moved from the leftmost position
    sticky: bool,
    /// Tab content is private and not saved in history
    private: bool,
    /// Tab is currently active
    active: bool,
    /// Tab is currently loading
    loading: bool,
    /// URL that is loaded into the tab
    url: String,
    /// History of the tab
    history: Vec<String>,
    /// Name of the tab / title to display
    name: String,
    /// Loaded favicon of the tab
    favicon: Option<Image>,
    // Text buffer holds the text of the tab (this is the page rendered later)
    // buffer: gtk::TextBuffer,
}

impl GosubTab {
    pub fn new(url: &str, favicon: Option<Image>) -> Self {
        GosubTab {
            id: uuid::Uuid::new_v4(),
            sticky: false,
            private: false,
            active: false,
            loading: false,
            url: url.to_string(),
            history: Vec::new(),
            name: url.to_string(),
            favicon,
            // buffer: gtk::TextBuffer::new(None),
        }
    }

    pub fn id(&self) -> uuid::Uuid {
        self.id
    }

    pub fn set_sticky(&mut self, sticky: bool) {
        self.sticky = sticky;
    }

    pub fn is_sticky(&self) -> bool {
        self.sticky
    }

    pub fn set_private(&mut self, private: bool) {
        self.private = private;
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn set_loading(&mut self, loading: bool) {
        self.loading = loading;
    }

    pub fn set_url(&mut self, url: &str) {
        self.url = url.to_string();
    }

    pub fn add_to_history(&mut self, url: &str) {
        self.history.push(url.to_string());
    }

    pub fn pop_history(&mut self) -> Option<String> {
        self.history.pop()
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn set_favicon(&mut self, favicon: Option<Image>) {
        self.favicon = favicon;
    }

    // pub fn set_page_index(&mut self, index: u32) {
    //     self.page_index = Some(index);
    // }
}

pub struct GosubTabManager {
    // All known tabs in the system
    tabs: HashMap<Uuid, GosubTab>,
    // Currently active tab, if any
    active_tab: Option<Uuid>,
    // Any tabs that need repainting because they might be changed
    #[allow(dead_code)]
    dirty_tabs: Vec<Uuid>,
    // Actual ordering of the tabs in the notebook
    ordering: Vec<Uuid>,
}

impl Default for GosubTabManager {
    fn default() -> Self {
        Self::new()
    }
}

impl GosubTabManager {
    pub fn new() -> Self {
        GosubTabManager {
            tabs: HashMap::new(),
            active_tab: None,
            dirty_tabs: Vec::new(),
            ordering: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn clear_dirty(&mut self) {
        self.dirty_tabs.clear();
    }

    #[allow(dead_code)]
    pub(crate) fn dirty_tabs(&self) -> Vec<Uuid> {
        self.dirty_tabs.clone()
    }

    pub(crate) fn get_active_tab_mut(&mut self) -> Option<&mut GosubTab> {
        self.tabs.get_mut(&self.active_tab?)
    }

    pub fn set_active(&mut self, tab_id: Uuid) {
        self.active_tab = Some(tab_id);
    }

    pub fn add_tab(&mut self, tab: GosubTab, position: Option<usize>) {
        if let Some(pos) = position {
            self.ordering.insert(pos, tab.id);
        } else {
            self.ordering.push(tab.id);
        }

        self.tabs.insert(tab.id, tab);
    }

    pub fn remove_tab(&mut self, tab_id: Uuid) {
        if let Some(index) = self.ordering.iter().position(|id| id == &tab_id) {
            self.ordering.remove(index);
        }

        self.tabs.remove(&tab_id);
    }

    pub fn get_tab(&self, tab_id: Uuid) -> Option<&GosubTab> {
        self.tabs.get(&tab_id)
    }

    pub fn get_tab_mut(&mut self, tab_id: Uuid) -> Option<&mut GosubTab> {
        self.tabs.get_mut(&tab_id)
    }

    pub fn order(&self) -> Vec<Uuid> {
        self.ordering.clone()
    }
}


// /// Update the tab label with the new tab data
// pub fn update_current_tab(tab_bar: Notebook, tab_data: GosubTab) {
//     if let Some(current_page) = tab_bar.current_page() {
//         let label_vbox = create_label(tab_bar.clone(), tab_data);
//         tab_bar.set_tab_label(&tab_bar.nth_page(Some(current_page)).unwrap(), Some(&label_vbox));
//     }
// }
//
// /// Add a new tab to the tab bar at the end of the list
// pub fn add_new_tab(tab_bar: Notebook, tab_data: &GosubTab) {
//     // Add new tab at the end of the notebook list
//     insert_tab(tab_bar, tab_data, None);
// }

// Create a new tab label
pub fn create_label(tab: &GosubTab) -> gtk::Box {
    let label_vbox = gtk::Box::new(gtk::Orientation::Horizontal, 5);

    // When the tab is loading, we show a spinner
    if tab.loading {
        let spinner = gtk::Spinner::new();
        spinner.start();
        label_vbox.append(&spinner);
    } else if let Some(favicon) = tab.favicon.clone() {
        label_vbox.append(&favicon);
    }

    // Only show the title and close button if the tab is not sticky
    if ! tab.is_sticky() {
        let tab_label = gtk::Label::new(Some(tab.name.as_str()));
        label_vbox.append(&tab_label);

        let tab_btn = gtk::Button::builder()
            .has_frame(false)
            .margin_bottom(0)
            .margin_end(0)
            .margin_start(0)
            .margin_top(0)
            .build();
        let img = gtk::Image::from_icon_name("window-close-symbolic");
        tab_btn.set_child(Some(&img));

        let tab_clone = tab.clone();
        tab_btn.connect_clicked(
            move |_| {
                println!("Clicked close button for tab {}", tab_clone.id);
            }
        );

        label_vbox.append(&tab_btn);
    }

    label_vbox
}

// pub fn insert_tab(tab_bar: Notebook, tab_data: &GosubTab, position: Option<u32>) {
//     // Tab content
//     let img = gtk::Image::from_resource("/io/gosub/browser-gtk/assets/submarine.svg");
//     img.set_visible(true);
//     img.set_can_focus(false);
//     img.set_valign(gtk::Align::End);
//     img.set_margin_top(64);
//     img.set_pixel_size(500);
//
//     let content_vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
//     content_vbox.set_visible(true);
//     content_vbox.set_can_focus(false);
//     content_vbox.set_halign(gtk::Align::Center);
//     content_vbox.set_vexpand(true);
//     content_vbox.set_hexpand(true);
//     content_vbox.append(&img);
//
//     let label_vbox = create_label(tab_bar.clone(), tab_data);
//
//     let page_index = match position {
//         Some(pos) => {
//             tab_bar.insert_page(&content_vbox, Some(&label_vbox), Some(pos))
//         }
//         None => {
//             tab_bar.append_page(&content_vbox, Some(&label_vbox))
//         }
//     };
//     tab_data.page_index = page_index;
//
//     // tab_bar.nth_page(page_index).unwrap().set &content_vbox, Some(&label_vbox));
//
//     // let page_index = tab_bar.append_page(&content_vbox, Some(&label_vbox));
// }
