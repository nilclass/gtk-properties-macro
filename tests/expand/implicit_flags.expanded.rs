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
                    ParamSpecString::builder("implicitly-read-only")
                        .flags(glib::ParamFlags::READABLE)
                        .build(),
                    ParamSpecString::builder("implicitly-write-only")
                        .flags(glib::ParamFlags::WRITABLE)
                        .build(),
                    ParamSpecString::builder("implicitly-read-write")
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
            1usize => self.name.borrow().clone().to_value(),
            3usize => self.name.borrow().clone().to_value(),
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
            2usize => self.name.replace(value.get().unwrap()),
            3usize => self.name.replace(value.get().unwrap()),
            _ => ::core::panicking::panic("not implemented"),
        }
    }
}
