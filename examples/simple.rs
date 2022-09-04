use gtk::glib;
use gtk::prelude::*;

glib::wrapper! {
    pub struct MyClass(ObjectSubclass<imp::MyClass>);
}

impl MyClass {
    fn new() -> Self {
        glib::Object::new(&[("name", &"Initial Name")]).unwrap()
    }
}

mod imp {
    use gtk::glib;
    use gtk::subclass::prelude::*;
    use gtk_properties_macro::properties;
    use std::cell::RefCell;

    #[derive(Default)]
    pub struct MyClass {
        name: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MyClass {
        const NAME: &'static str = "MyClass";
        type Type = super::MyClass;
        type ParentType = gtk::glib::Object;
    }

    impl ObjectImpl for MyClass {
        properties! {
            #[string]
            "name" => {
                get { self.name.borrow().clone().to_value() },
                set { self.name.replace(value.get().unwrap()); },
            },
        }
    }
}

fn main() {
    let my_object = MyClass::new();
    println!(
        "Object name before set: {:?}",
        my_object.property::<String>("name")
    );
    my_object.set_property("name", "New Name".to_string().to_value());
    println!(
        "Object name after set: {:?}",
        my_object.property::<String>("name")
    );
}
