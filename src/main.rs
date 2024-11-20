mod window;
mod tab;
mod dialog;
mod fetcher;
mod application;
mod utils;
mod tab_wiget;

use std::sync::OnceLock;
use adw::gdk::Display;
use adw::prelude::ApplicationExt;
use gtk4::{gio, CssProvider};
use tokio::runtime::Runtime;
use crate::application::Application;

const APP_ID: &str = "io.gosub.browser-gtk";

fn runtime() -> &'static Runtime {
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| {
        Runtime::new().expect("Setting up tokio runtime needs to succeed.")
    })
}

fn main() {
    colog::init();

    gio::resources_register_include!("gosub.gresource").expect("Failed to register resources.");

    let app = Application::new();
    app.connect_startup(|_| load_css());
    app.run();
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_data(include_str!("../resources/style.css"));

    gtk4::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display"),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION
    );
}