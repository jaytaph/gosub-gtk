use std::cell::RefCell;
use std::collections::HashMap;
use gtk4::gdk_pixbuf::Pixbuf;
use uuid::Uuid;

#[derive(Clone)]
pub struct GosubTab {
    /// Id of the tab
    id: Uuid,
    /// Tab is sticky and cannot be moved from the leftmost position
    sticky: bool,
    /// Tab content is private and not saved in history
    private: bool,
    /// Tab is currently loading
    loading: bool,
    /// URL that is loaded into the tab
    url: String,
    /// History of the tab
    history: Vec<String>,
    /// Name of the tab / title to display
    name: String,
    /// Loaded favicon of the tab
    favicon: Option<Pixbuf>,
    // Text buffer holds the text of the tab (this is the page rendered later)
    // buffer: gtk::TextBuffer,
}

impl GosubTab {
    pub fn new(url: &str, favicon: Option<Pixbuf>) -> Self {
        GosubTab {
            id: Uuid::new_v4(),
            sticky: false,
            private: false,
            loading: false,
            url: url.to_string(),
            history: Vec::new(),
            name: url.to_string(),
            favicon,
            // buffer: gtk::TextBuffer::new(None),
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
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

    pub(crate) fn is_loading(&self) -> bool {
        self.loading
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

    pub(crate) fn favicon(&self) -> Option<Pixbuf> {
        self.favicon.clone()
    }

    pub fn set_favicon(&mut self, favicon: Option<Pixbuf>) {
        self.favicon = favicon;
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum TabCommand {
    Close(u32),     // Close index
    CloseAll,       // Close all
    Move(u32, u32), // Move from index to index
    Pin(u32),       // Pin index
    Unpin(u32),     // Unpin index
    Private(u32),   // Make private tab
    Update(u32),    // Update index
    Insert(u32),    // Insert index
    Activate(u32),  // set as active
}

pub struct GosubTabManager {
    // All known tabs in the system
    tabs: HashMap<Uuid, GosubTab>,
    // Currently active tab, if any
    active_tab: RefCell<Option<Uuid>>,
    // Any tabs that need repainting because they might be changed
    #[allow(dead_code)]
    dirty_tabs: Vec<Uuid>,
    // Actual ordering of the tabs in the notebook. Used for converting page_num to tab_id
    ordering: Vec<Uuid>,

    // list of commands to execute on the tab notebook
    commands: RefCell<Vec<TabCommand>>,
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
            active_tab: RefCell::new(None),
            dirty_tabs: Vec::new(),
            ordering: Vec::new(),
            commands: RefCell::new(Vec::new()),
        }
    }

    pub(crate) fn commands(&self) -> Vec<TabCommand> {
        self.commands.borrow_mut().drain(..).collect()
    }

    pub(crate) fn page_to_tab(&self, page_index: u32) -> Option<Uuid> {
        self.ordering.get(page_index as usize).cloned()
    }

    pub(crate) fn get_active_tab(&self) -> Option<&GosubTab> {
        match self.active_tab.borrow().as_ref() {
            Some(tab_id) => self.tabs.get(tab_id),
            None => None,
        }
    }

    pub fn set_active(&self, tab_id: Uuid) {
        let page_num = self.ordering.iter().position(|&id| id == tab_id);
        println!("Setting active tab to page {} / {}", page_num.unwrap(), tab_id);
        self.active_tab.replace(Some(tab_id));

        self.commands.borrow_mut().push(TabCommand::Activate(page_num.unwrap() as u32));
    }

    pub(crate) fn update_tab(&mut self, tab_id: Uuid, url: &str, title: &str, favicon: Option<Pixbuf>) {
        let tab = self.tabs.get_mut(&tab_id).expect("Tab not found");
        tab.set_url(url);
        tab.set_name(title);
        tab.set_favicon(favicon);

        self.commands.borrow_mut().push(TabCommand::Update(self.ordering.iter().position(|id| id == &tab_id).unwrap() as u32));
    }

    pub fn add_tab(&mut self, tab: GosubTab, position: Option<usize>) {
        let pos = if let Some(pos) = position {
            self.ordering.insert(pos, tab.id);
            pos
        } else {
            self.ordering.push(tab.id);
            self.ordering.len() - 1
        };

        self.commands.borrow_mut().push(TabCommand::Insert(pos as u32));

        let tab_id = tab.id.clone();
        self.tabs.insert(tab_id, tab);

        if self.active_tab.borrow().is_none() {
            self.set_active(tab_id);
        }
    }

    pub fn remove_tab(&mut self, tab_id: Uuid) {
        if let Some(index) = self.ordering.iter().position(|id| id == &tab_id) {
            println!("removing tab at index {}", index);
            self.ordering.remove(index);
            self.commands.borrow_mut().push(TabCommand::Close(index as u32));
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

