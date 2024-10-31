mod window;
mod tab;
mod dialog;

use gtk::{Button, Stack, StackSwitcher, StackTransitionType};
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
    // stack.set_transition_type(StackTransitionType::SlideLeftRight);
    stack.set_transition_type(StackTransitionType::Crossfade);
    stack.set_transition_duration(100);

    let stack_switcher = StackSwitcher::new();
    stack_switcher.set_stack(Some(&stack));

    let add_page = |stack: &Stack, title: &str| {

        // let viewport = gtk::Viewport::new(None, None);
        let scrolled_window = gtk::ScrolledWindow::new();
        scrolled_window.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
        // scrolled_window.set_child(Some(&viewport));

        let page_content = gtk::Box::new(gtk::Orientation::Vertical, 10);
        page_content.append(&scrolled_window);

        // let label = Label::new(Some(&format!("This is the {}", title)));
        // let button = Button::with_label("Button on tab");
        // page_content.append(&label);
        // page_content.append(&button);

        // let tab_label = Label::new(Some(title));
        //
        // let tab_close = Button::from_icon_name("window-close-symbolic");
        // tab_close.set_property("has_frame", false);
        // tab_close.set_halign(gtk::Align::End);
        // tab_close.set_valign(gtk::Align::Center);
        // tab_close.set_margin_start(4);
        // tab_close.set_margin_end(4);
        // tab_close.set_margin_top(4);
        // tab_close.set_margin_bottom(4);
        // tab_close.set_has_tooltip(true);
        // tab_close.set_tooltip_text(Some("Close tab"));
        // tab_close.connect_clicked(clone!(@weak stack => move |_| {
        //     let current_page = stack.visible_child().unwrap();
        //     stack.remove(&current_page);
        // }));
        //
        // let tab_favicon = Image::from_resource("/io/gosub/browser-gtk/assets/gosub.svg");
        // tab_favicon.set_halign(gtk::Align::Start);
        // tab_favicon.set_valign(gtk::Align::Center);
        // tab_favicon.set_margin_start(4);
        // tab_favicon.set_margin_end(4);
        // tab_favicon.set_margin_top(4);
        // tab_favicon.set_margin_bottom(4);
        //
        // let tab = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        // tab.append(&tab_favicon);
        // tab.append(&tab_label);
        // tab.append(&tab_close);

        // stack.add_child(&tab);

        let page = stack.add_titled(&page_content, Some(title), title);
        page.set_title(title);
        page
    };

    add_page(&stack, "First tab");
    add_page(&stack, "Second tab");

    let add_tab_button = Button::builder()
        .icon_name("list-add-symbolic")
        .has_frame(false)
        .has_tooltip(true)
        .tooltip_text("Add a new tab")
        .build();

    add_tab_button.set_property("has_frame", false);
    add_tab_button.connect_clicked(clone!(@weak stack => move |_| {
        let tab_count = stack.pages().n_items();
        let new_tab_title = format!("Tab {}", tab_count + 1);
        add_page(&stack, &new_tab_title);

        stack.set_visible_child_name(&new_tab_title);
    }));

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);

    hbox.append(&stack_switcher);
    hbox.append(&add_tab_button);

    vbox.append(&hbox);
    vbox.append(&stack);


    // window.set_child(Some(&vbox));

    // let button = Button::with_label("+");
    // let page = stack.add_named(&button, Some("+"));
    // page.set_title("+");


    // let add_button = Button::with_label("+");
    // add_button.connect_clicked(clone!(@weak stack => move |_| {
    //     let tab_count = stack.pages().n_items();
    //     let new_tab_title = format!("Tab {}", tab_count + 1);
    //     add_page(&stack, &new_tab_title);
    //
    //     stack.set_visible_child_name(&new_tab_title);
    // }));

    // vbox.append(&add_button);

    // window.set_child(Some(&vbox));
    window.present();
}

fn toggle_dark_mode() {
    if let Some(settings) = gtk::Settings::default() {
        let is_dark = settings.is_gtk_application_prefer_dark_theme();
        settings.set_gtk_application_prefer_dark_theme(!is_dark);
    }
}
