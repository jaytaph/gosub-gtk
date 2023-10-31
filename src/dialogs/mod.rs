// Imports
use crate::appwindow::RnAppWindow;
use crate::canvas::RnCanvas;
use crate::canvaswrapper::RnCanvasWrapper;
use crate::config;
use crate::workspacebrowser::workspacesbar::RnWorkspaceRow;
use crate::{globals, RnIconPicker};
use adw::prelude::*;
use gettextrs::{gettext, pgettext};
use gtk4::{
    gio, glib, glib::clone, Builder, Button, CheckButton, ColorDialogButton, Dialog, FileDialog,
    Label, MenuButton, ResponseType, ShortcutsWindow, StringList,
};

pub(crate) fn dialog_about(appwindow: &RnAppWindow) {
    let app_icon_name = config::APP_NAME.to_string()

    let aboutdialog = adw::AboutWindow::builder()
        .modal(true)
        .transient_for(appwindow)
        .application_name(config::APP_NAME_CAPITALIZED)
        .application_icon(app_icon_name)
        .comments(gettext("Sketch and take handwritten notes"))
        .website(config::APP_WEBSITE)
        .issue_url(config::APP_ISSUES_URL)
        .support_url(config::APP_SUPPORT_URL)
        .developer_name(config::APP_AUTHOR_NAME)
        .developers(glib::StrV::from(
            config::APP_AUTHORS
                .iter()
                .map(|&s| String::from(s))
                .collect::<Vec<String>>(),
        ))
        // TRANSLATORS: 'Name <email@domain.com>' or 'Name https://website.example'
        .translator_credits(gettext("translator-credits"))
        .license_type(globals::APP_LICENSE)
        .version((String::from(config::APP_VERSION) + config::APP_VERSION_SUFFIX).as_str())
        .build();

    if config::PROFILE == "devel" {
        aboutdialog.add_css_class("devel");
    }

    aboutdialog.present();
}
