use adw::subclass::prelude::AdwApplicationImpl;
use gtk4::{gio, glib, prelude::*, subclass::prelude::*};
use gtk_macros::action;
use adw::glib::clone;
use log::info;
use crate::window::BrowserWindow;

mod imp {
    use crate::window::BrowserWindow;
    use super::*;

    pub struct Application {
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Application {
        const NAME: &'static str = "Application";
        type Type = super::Application;
        type ParentType = adw::Application;
    }

    impl Default for Application {
        fn default() -> Self {
            Self {
            }
        }
    }

    impl ObjectImpl for Application {}

    impl ApplicationImpl for Application {
        fn activate(&self) {
            info!("GtkApplication<Application>::activate");
            self.parent_activate();

            let obj = self.obj();

            if let Some(window) = obj.windows().first() {
                window.present();
                return;
            }

            let window = BrowserWindow::new(&obj);
            window.present();
        }

        fn startup(&self) {
            self.parent_startup();

            let obj = self.obj();
            obj.setup_actions();
            obj.setup_accelerators();
        }
    }

    impl GtkApplicationImpl for Application {}
    impl AdwApplicationImpl for Application {}
}

glib::wrapper! {
    pub struct Application(ObjectSubclass<imp::Application>)
        @extends gio::Application, gtk4::Application, adw::Application, @implements gio::ActionMap, gio::ActionGroup;
}

impl Application {
    pub fn new() -> Self {
        glib::Object::builder()
            .property("application-id", &"io.gosub.browser-gtk")
            .property("resource-base-path", &Some("/io/gosub/browser-gtk"))
            .build()
    }

    pub fn window(&self) -> BrowserWindow {
        self.active_window()
            .map_or_else(|| BrowserWindow::new(self), |w| w.downcast().unwrap())
    }

    fn setup_actions(&self) {
        action!(
            self,
            "quit",
            clone!(
                #[weak(rename_to=app)] self,
                move |_, _| {
                    app.quit();
                }
            )
        );

        action!(
            self,
            "new-tab",
            clone!(
                #[weak(rename_to=_app)] self,
                move |_, _| {
                    info!("New tab action triggered");
                    // app.new_tab();
                }
            )
        );

        action!(
            self,
            "close-tab",
            clone!(
                #[weak(rename_to=_app)] self,
                move |_, _| {
                    info!("Close tab action triggered");
                    // app.close_tab();
                }
            )
        );

        action!(
            self,
            "toggle-dark-mode",
            clone!(
                #[weak(rename_to=_app)] self,
                move |_, _| {
                    info!("Toggle dark mode action triggered");
                    // app.toggle_dark_mode();
                }
            )
        );

        action!(
            self,
            "show-about",
            clone!(
                #[weak(rename_to=_app)] self,
                move |_, _| {
                    info!("Show about dialog action triggered");
                    // app.show_about_dialog();
                }
            )
        );

        action!(
            self,
            "show-log",
            clone!(
                #[weak(rename_to=_app)] self,
                move |_, _| {
                    info!("Show log action triggered");
                    // app.show_log();
                }
            )
        );
    }

    fn setup_accelerators(&self) {
        self.set_accels_for_action("app.quit", &["<Primary>Q"]);
        self.set_accels_for_action("app.new-tab", &["<Primary>T"]);
        self.set_accels_for_action("app.close-tab", &["<Primary>W"]);
        self.set_accels_for_action("app.toggle-dark-mode", &["<Primary>D"]);
        self.set_accels_for_action("app.show-about", &["<Primary>A"]);
        self.set_accels_for_action("app.show-log", &["<Primary>L"]);
    }

    pub fn run(&self) {
        info!("Application started");
        ApplicationExtManual::run(self);
    }
}

impl Default for Application {
    fn default() -> Self {
        gio::Application::default()
            .unwrap()
            .downcast::<Application>()
            .unwrap()
    }
}