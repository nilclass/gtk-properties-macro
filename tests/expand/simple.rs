use gtk_properties_macro::properties;

struct MyObject {
    name: RefCell<String>,
}

impl ObjectImpl for MyObject {
    properties! {
        #[string]
        "name" => {
            get { self.name.borrow().clone().to_value() }
            set { self.name.replace(value.get().unwrap()) }
        }
    }
}
