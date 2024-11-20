use std::cell::RefCell;
use gtk4::{glib, prelude::*};
use gtk4::subclass::prelude::ObjectSubclassIsExt;
use crate::tab::TabId;

glib::wrapper! {
    pub struct TabWidget(ObjectSubclass<imp::TabWidget>)
        @extends gtk4::Widget;
}

impl TabWidget {
    pub fn new(tab_id: TabId, child: &impl IsA<gtk4::Widget>) -> TabWidget {
        let mut obj = glib::Object::new::<TabWidget>();
        obj.imp().tab_id = tab_id;
        obj.imp().child = RefCell::new(Some(child));

        obj
    }
}

mod imp {
    use std::cell::RefCell;
    use gtk4::{glib, Widget};
    use uuid::Uuid;
    use gtk4::glib::prelude::*;
    use gtk4::subclass::prelude::*;
    use crate::tab::TabId;

    pub struct TabWidget {
        pub tab_id: TabId,
        pub child: RefCell<Option<&impl IsA<Widget>>>,
        // pub child: RefCell<Option<gtk4::Widget>>,
    }

    impl Default for TabWidget {
        fn default() -> Self {
            Self {
                tab_id: Uuid::new_v4(),
                child: RefCell::new(None),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TabWidget {
        const NAME: &'static str = "TabWidget";
        type Type = super::TabWidget;
        type ParentType = gtk4::Widget;

        fn class_init(klass: &mut Self::Class) {
            klass.set_css_name("tab-widget");
        }

        // fn new(tab_id: Uuid, child: &impl IsA<gtk4::Widget>) -> Self {
        //     Self {
        //         tab_id,
        //         child: RefCell::new(Some(child.upcast())),
        //     }
        // }
    }

    impl WidgetImpl for TabWidget {}

    impl ObjectImpl for TabWidget {
        fn properties() -> &'static [gtk4::glib::ParamSpec] {
            use once_cell::sync::Lazy;

            static PROPERTIES: Lazy<Vec<gtk4::glib::ParamSpec>> = Lazy::new(|| {
                vec![gtk4::glib::ParamSpecString::builder("tab_id")
                    .nick("Tab ID")
                    .blurb("The unique identifier for the tab")
                    .read_only()
                    .build()]
            });
            PROPERTIES.as_ref()
        }

        fn set_property(&self, _id: usize, _value: &gtk4::glib::Value, pspec: &gtk4::glib::ParamSpec) {
            match pspec.name() {
                "tab-id" => {
                    // let tab_id = value.get().expect("type check failed");
                    // self.tab_id = tab_id;
                }
                _ => unimplemented!(),
            }
        }

        fn property(&self, _id: usize, pspec: &gtk4::glib::ParamSpec) -> gtk4::glib::Value {
            match pspec.name() {
                "tab-id" => self.tab_id.to_string().to_value(),
                _ => unimplemented!(),
            }
        }
    }

    // impl WidgetImpl for TabWidget {
    //     const NAME: &'static str = "TabWidget";
    //     type ParentType = gtk4::Widget;
    //     type Instance = glib::subclass::simple::InstanceStruct<Self>;
    //     type Class = glib::subclass::simple::ClassStruct<Self>;
    //
    //     glib::object_subclass!();
    //
    //     fn new() -> Self {
    //         Self {
    //             tab_id: Uuid::new_v4(),
    //             child: gtk4::Widget::new(),
    //         }
    //     }
    // }
}