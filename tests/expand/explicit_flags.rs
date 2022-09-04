use gtk_properties_macro::properties;

struct MyObject {
    name: RefCell<String>,
}

impl ObjectImpl for MyObject {
    properties! {
        #[string(readable, explicit_notify)]
        "name" => {
            get { self.name.borrow().clone().to_value() }
        }
    }
}
