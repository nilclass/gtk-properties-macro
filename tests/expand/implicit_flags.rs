use gtk_properties_macro::properties;

struct MyObject {
    name: RefCell<String>,
}

impl ObjectImpl for MyObject {
    properties! {
        #[string]
        "implicitly-read-only" => {
            get { self.name.borrow().clone().to_value() }
        },
        #[string]
        "implicitly-write-only" => {
            set { self.name.replace(value.get().unwrap()) }
        },
        #[string]
        "implicitly-read-write" => {
            get { self.name.borrow().clone().to_value() }
            set { self.name.replace(value.get().unwrap()) }
        }
    }
}
