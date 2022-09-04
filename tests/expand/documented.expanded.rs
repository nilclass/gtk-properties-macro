use gtk_properties_macro::properties;
struct MyObject {
    name: RefCell<String>,
}
impl ObjectImpl for MyObject {
    fn properties() -> &'static [gtk::glib::ParamSpec] {
        use once_cell::sync::Lazy;
        use gtk::glib::*;
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    ParamSpecString::builder("name")
                        .flags(glib::ParamFlags::READABLE)
                        .blurb("contains the name of this object")
                        .nick("Object Name")
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
            1usize => self.name.borrow().clone().to_value(),
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
            _ => ::core::panicking::panic("not implemented"),
        }
    }
}
