use std::cell::RefCell;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::glib;
use gtk::glib::Properties;
use crate::gosub_tab::TabData;

#[derive(Properties, Default)]
#[properties(wrapper_type = super::TabObject)]
pub struct TabObject {
    #[property(name="tab_data", get, set, type=TabData, member = data)]
    pub data: RefCell<TabData>
}

#[glib::object_subclass]
impl ObjectSubclass for TabObject {
    const NAME: &'static str = "TabObject";
    type Type = super::TabObject;
}

#[glib::derived_properties]
impl ObjectImpl for TabObject {}