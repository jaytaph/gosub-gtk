use gtk::{Image, Notebook, Widget};
use gtk::glib::Cast;
use gtk::prelude::{BoxExt, ButtonExt, WidgetExt};

#[derive(Clone)]
pub struct GosubTab {
    /// Tab is sticky and cannot be moved from the leftmost position
    pub is_sticky: bool,
    /// Tab content is private and not saved in history
    pub is_private: bool,
    /// Tab is currently active
    pub is_active: bool,
    /// Tab is currently loading
    pub is_loading: bool,
    /// URL that is loaded into the tab
    pub url: String,
    /// Name of the tab / title to display
    pub name: String,
    /// Loaded favicon of the tab
    pub favicon: Option<Image>,
    /// Text buffer holds the text of the tab (this is the page rendered later)
    pub buffer: gtk::TextBuffer,

    // The number of the page in the notebook, if it is added
    pub page_index: Option<u32>,
}

impl GosubTab {
    pub fn new(url: &str, favicon: Option<Image>) -> Self {
        GosubTab {
            is_sticky: false,
            is_private: false,
            is_active: false,
            is_loading: false,
            url: url.to_string(),
            name: url.to_string(),
            favicon,
            buffer: gtk::TextBuffer::new(None),
            page_index: None,
        }
    }

    pub fn set_page_index(&mut self, index: u32) {
        self.page_index = Some(index);
    }
}


/// Update the tab label with the new tab data
pub fn update_current_tab(tab_bar: Notebook, tab_data: GosubTab) {
    if let Some(current_page) = tab_bar.current_page() {
        let label_vbox = create_label(tab_bar.clone(), tab_data);
        tab_bar.set_tab_label(&tab_bar.nth_page(Some(current_page)).unwrap(), Some(&label_vbox));
    }
}

/// Add a new tab to the tab bar at the end of the list
pub fn add_new_tab(tab_bar: Notebook, tab_data: &GosubTab) {
    // Add new tab at the end of the notebook list
    insert_tab(tab_bar, tab_data, None);
}

// Create a new tab label
fn create_label(tab_bar: Notebook, tab_data: GosubTab) -> gtk::Box {
    // Tab label
    let label_vbox = gtk::Box::new(gtk::Orientation::Horizontal, 5);

    if let Some(favicon) = tab_data.favicon {
        label_vbox.append(&favicon);
    }
    let tab_label = gtk::Label::new(Some(tab_data.name.as_str()));
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
    label_vbox.append(&tab_btn);

    // Connect the close button to remove the tab
    let notebook_clone = tab_bar.clone();
    tab_btn.connect_clicked(
        // let tab_data_clone = tab_data.clone();
        move |btn| {
            println!("Clicked close button");
            if let Some(page_index) = tab_data_clone.page_index {
                notebook_clone.remove_page(Some(page_index));
                tab_data.page_index = None;
            }
        }
    });

    label_vbox
}

pub fn insert_tab(tab_bar: Notebook, tab_data: &GosubTab, position: Option<u32>) {
    // Tab content
    let img = gtk::Image::from_resource("/io/gosub/browser-gtk/assets/submarine.svg");
    img.set_visible(true);
    img.set_can_focus(false);
    img.set_valign(gtk::Align::End);
    img.set_margin_top(64);
    img.set_pixel_size(500);

    let content_vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    content_vbox.set_visible(true);
    content_vbox.set_can_focus(false);
    content_vbox.set_halign(gtk::Align::Center);
    content_vbox.set_vexpand(true);
    content_vbox.set_hexpand(true);
    content_vbox.append(&img);

    let label_vbox = create_label(tab_bar.clone(), tab_data);

    let page_index = match position {
        Some(pos) => {
            tab_bar.insert_page(&content_vbox, Some(&label_vbox), Some(pos))
        }
        None => {
            tab_bar.append_page(&content_vbox, Some(&label_vbox))
        }
    };
    tab_data.page_index = page_index;

    // tab_bar.nth_page(page_index).unwrap().set &content_vbox, Some(&label_vbox));

    // let page_index = tab_bar.append_page(&content_vbox, Some(&label_vbox));
}
