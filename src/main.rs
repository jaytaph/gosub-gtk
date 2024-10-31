mod window;
mod tab;
mod dialog;

use gtk::prelude::*;
use gtk::{gio, glib, Application, CssProvider, Notebook};
use gtk::gdk::Display;
use crate::dialog::about::About;
use crate::tab::GosubTab;
use crate::window::BrowserWindow;

const APP_ID: &str = "io.gosub.browser-gtk";

fn main() -> glib::ExitCode {
    gio::resources_register_include!("gosub.gresource").expect("Failed to register resources.");

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);
    app.run()
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_data(include_str!("../resources/style.css"));

    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION
    );
}

// Should this functionality be inside the windows module? This has nothing to do with the application.
fn build_ui(app: &Application) {
    let window = BrowserWindow::new(app);
    window.present();
}

fn add_new_tab(tab_bar: Notebook, tab_data: GosubTab) {
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
    // tab_btn.set_action_name(Some("app.tab.close"));
    let img = gtk::Image::from_icon_name("window-close-symbolic");
    tab_btn.set_child(Some(&img));
    label_vbox.append(&tab_btn);

    let page_index = tab_bar.append_page(&content_vbox, Some(&label_vbox));

    let notebook_clone = tab_bar.clone();
    tab_btn.connect_clicked(move |_| {
        notebook_clone.remove_page(Some(page_index));
    });
}

fn show_about_dialog() {
    let about = About::new();
    about.show();
}

fn toggle_dark_mode() {
    if let Some(settings) = gtk::Settings::default() {
        let is_dark = settings.is_gtk_application_prefer_dark_theme();
        settings.set_gtk_application_prefer_dark_theme(!is_dark);
    }
}
