use gtk_properties_macro::properties;

struct MyObject {
    name: RefCell<String>,
}

impl ObjectImpl for MyObject {
    properties! {
        /// contains the name of this object
        #[string(nick="Object Name")]
        "name" => {
            get { self.name.borrow().clone().to_value() }
        }
    }
}
