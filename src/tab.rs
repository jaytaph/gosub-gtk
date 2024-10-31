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
    pub favicon: gtk::Image,
    /// Text buffer holds the text of the tab (this is the page rendered later)
    pub buffer: gtk::TextBuffer,
}

impl GosubTab {
    pub fn new(url: &str) -> Self {
        GosubTab {
            is_sticky: false,
            is_private: false,
            is_active: false,
            is_loading: false,
            url: url.to_string(),
            name: url.to_string(),
            favicon: gtk::Image::new(),
            buffer: gtk::TextBuffer::new(None),
        }
    }
}