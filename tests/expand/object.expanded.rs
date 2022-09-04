use gtk_properties_macro::properties;
struct MyDialog {
    ok_button: RefCell<gtk::Button>,
}
impl ObjectImpl for MyDialog {
    fn properties() -> &'static [gtk::glib::ParamSpec] {
        use once_cell::sync::Lazy;
        use gtk::glib::*;
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    ParamSpecObject::builder("ok-button", gtk::Button::static_type())
                        .flags(glib::ParamFlags::READWRITE)
                        .build(),
                ]),
            )
        });
        PROPERTIES.as_ref()
    }
    fn property(
        &self,
        object: &Self::Type,
        id: usize,
        pspec: &gtk::glib::ParamSpec,
    ) -> gtk::glib::Value {
        use gtk::glib::prelude::*;
        match id {
            1usize => self.ok_button.borrow().clone().to_value(),
            _ => ::core::panicking::panic("not implemented"),
        }
    }
    fn set_property(
        &self,
        object: &Self::Type,
        id: usize,
        value: &gtk::glib::Value,
        pspec: &gtk::glib::ParamSpec,
    ) {
        use gtk::glib::prelude::*;
        match id {
            1usize => {
                self.ok_button.replace(value.get().expect("expected gtk::Button"));
            }
            _ => ::core::panicking::panic("not implemented"),
        }
    }
}
