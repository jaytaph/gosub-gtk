use crate::dialog::about::About;
use crate::window::BrowserWindow;
use crate::APP_ID;
use adw::glib::clone;
use adw::subclass::prelude::GtkApplicationImpl;
use adw::{ColorScheme, StyleManager};
use gtk4::{gio, glib, prelude::*, subclass::prelude::*};
use gtk_macros::action;
use log::info;

mod imp {
    use super::*;
    use crate::window::BrowserWindow;

    pub struct Application {
        // Here be settings structure and such
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Application {
        const NAME: &'static str = "Application";
        type Type = super::Application;
        type ParentType = gtk4::Application;
    }

    impl Default for Application {
        fn default() -> Self {
            Self {}
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
            info!("GtkApplication<Application>::startup");
            self.parent_startup();

            let obj = self.obj();
            obj.setup_actions();
            obj.setup_accelerators();
        }
    }

    impl GtkApplicationImpl for Application {}
    // impl AdwApplicationImpl for Application {}
}

glib::wrapper! {
    pub struct Application(ObjectSubclass<imp::Application>)
        @extends gio::Application, gtk4::Application, adw::Application, @implements gio::ActionMap, gio::ActionGroup;
}

impl Application {
    pub fn new() -> Self {
        glib::Object::builder()
            .property("application-id", APP_ID)
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
                #[weak(rename_to=app)]
                self,
                move |_, _| {
                    app.quit();
                }
            )
        );

        // action!(
        //     self,
        //     "open-new-tab",
        //     clone!(
        //         #[weak(rename_to=_app)] self,
        //         move |_, _| {
        //             info!("New tab action triggered");
        //             // app.new_tab();
        //         }
        //     )
        // );

        // action!(
        //     self,
        //     "close-tab",
        //     clone!(
        //         #[weak(rename_to=_app)] self,
        //         move |_, _| {
        //             info!("Close tab action triggered");
        //             // app.close_tab();
        //         }
        //     )
        // );

        action!(
            self,
            "toggle-dark-mode",
            clone!(
                #[weak(rename_to=_app)]
                self,
                move |_, _| {
                    info!("Toggle dark mode action triggered");
                    let mgr = StyleManager::default();
                    if mgr.is_dark() {
                        mgr.set_color_scheme(ColorScheme::ForceLight);
                    } else {
                        mgr.set_color_scheme(ColorScheme::ForceDark);
                    }
                }
            )
        );

        action!(
            self,
            "show-about",
            clone!(
                #[weak(rename_to=_app)]
                self,
                move |_, _| {
                    info!("Show about dialog action triggered");
                    let about = About::new();
                    about.show();
                }
            )
        );

        // action!(
        //     self,
        //     "toggle-log",
        //     clone!(
        //         #[weak(rename_to=_app)] self,
        //         move |_, _| {
        //             info!("Show log action triggered");
        //             // app.imp().local_command_line()
        //         }
        //     )
        // );
    }

    fn setup_accelerators(&self) {
        // Global application accelerators
        self.set_accels_for_action("app.quit", &["<Primary>Q"]);
        self.set_accels_for_action("app.toggle-dark-mode", &["<Primary>D"]);
        self.set_accels_for_action("app.show-about", &["<Primary>A"]);

        // // Window accelerators. This should be only when the window is active
        // self.set_accels_for_action("app.open-new-tab", &["<Primary>T"]);
        // self.set_accels_for_action("app.close-tab", &["<Primary>W"]);
        // self.set_accels_for_action("app.toggle-log", &["<Primary>L"]);
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
