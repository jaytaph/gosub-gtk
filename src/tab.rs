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
    pub favicon: gtk::Image,
    /// Textbuffer holds the text of the tab (this is the page rendered later)
    pub buffer: gtk::TextBuffer,
    /// Button of the tab
    pub button: gtk::Button,
}

impl GosubTab {
    pub fn new(url: String) -> Self {
        GosubTab {
            is_sticky: false,
            is_private: false,
            is_active: false,
            is_loading: false,
            url,
            name: String::from("A new tab"),
            favicon: gtk::Image::new(),
            buffer: gtk::TextBuffer::new(None),
            button: gtk::Button::new(),
        }
    }
}