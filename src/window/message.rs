use uuid::Uuid;

#[derive(Debug)]
pub enum Message {
    // Open a new tab, and load a URL
    OpenTab(String),
    /// Sent when we need to load a new url into a tab
    LoadUrl(Uuid, String),

    /// Sent when a favicon has been loaded for tab X
    FaviconLoaded(Uuid, Vec<u8>),
    // Sent when a URL has been loaded for tab X
    UrlLoaded(Uuid, String),
    // Refresh tabs
    RefreshTabs(),

    // Single message to print in the log
    Log(String),
}
