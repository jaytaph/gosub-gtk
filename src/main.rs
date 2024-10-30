mod window;
mod tab;
mod dialog;

use gtk::{Button, Label, Stack, StackSwitcher, StackTransitionType};
use gtk::prelude::*;
use gtk::prelude::{ButtonExt};
use gtk::{gio, glib, Application, CssProvider};
use gtk::gdk::Display;
use gtk::glib::clone;
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

fn build_ui(app: &Application) {
    let window = BrowserWindow::new(app);
    window.set_resizable(true);
    window.set_decorated(true);
    window.set_show_menubar(true);
    window.set_default_size(800, 600);

    let builder = gtk::Builder::from_resource("/io/gosub/browser-gtk/ui/main_menu.ui");
    let menubar = builder.object::<gio::MenuModel>("app-menu").expect("Could not find app-menu");

    app.set_menubar(Some(&menubar));
    window.set_show_menubar(true);

    // Dark mode toggle
    let action = gio::SimpleAction::new_stateful("toggle_darkmode", None, &false.to_variant());
    action.connect_activate(move |action, _| {
        let is_dark_mode = action.state().unwrap().get::<bool>().unwrap();
        action.set_state(&(!is_dark_mode).to_variant());

        toggle_dark_mode();
    });
    app.add_action(&action);


    // Tabs
    let stack = Stack::new();
    stack.set_transition_type(StackTransitionType::SlideLeftRight);
    stack.set_transition_duration(500);

    let stack_switcher = StackSwitcher::new();
    stack_switcher.set_stack(Some(&stack));

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    vbox.append(&stack_switcher);
    vbox.append(&stack);

    let add_page = |stack: &Stack, title: &str| {
        let page_content = gtk::Box::new(gtk::Orientation::Vertical, 10);

        let label = Label::new(Some(&format!("This is the {}", title)));
        let button = Button::with_label("Button on tab");
        page_content.append(&label);
        page_content.append(&button);

        let page = stack.add_titled(&page_content, Some(title), title);
        page.set_title(title);
        page
    };

    add_page(&stack, "First tab");
    add_page(&stack, "Second tab");

    let add_button = Button::with_label("+");
    add_button.connect_clicked(clone!(@weak stack => move |_| {
        let tab_count = stack.pages().n_items();
        let new_tab_title = format!("Tab {}", tab_count + 1);
        add_page(&stack, &new_tab_title);

        stack.set_visible_child_name(&new_tab_title);
    }));

    vbox.append(&add_button);
    window.set_child(Some(&vbox));

    window.present();
    window.set_resizable(true);
    window.set_decorated(true);
}

fn toggle_dark_mode() {
    if let Some(settings) = gtk::Settings::default() {
        let is_dark = settings.is_gtk_application_prefer_dark_theme();
        settings.set_gtk_application_prefer_dark_theme(!is_dark);
    }
}
