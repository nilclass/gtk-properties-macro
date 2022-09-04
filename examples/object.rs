use gtk::glib;
use gtk::prelude::*;

glib::wrapper! {
    pub struct MyClass(ObjectSubclass<imp::MyClass>);
}

impl MyClass {
    fn new() -> Self {
        glib::Object::new(&[]).unwrap()
    }
}

mod imp {
    use gtk::glib;
    use gtk::subclass::prelude::*;
    use gtk_properties_macro::properties;
    use std::cell::RefCell;

    #[derive(Default)]
    pub struct MyClass {
        ok_button: RefCell<gtk::Button>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MyClass {
        const NAME: &'static str = "MyClass";
        type Type = super::MyClass;
        type ParentType = gtk::glib::Object;
    }

    impl ObjectImpl for MyClass {
        properties! {
            #[object(gtk::Button, nick = "Ok Button", blurb = "The primary button")]
            "ok-button" => {
                get { self.ok_button.borrow().clone().to_value() }
                set { self.ok_button.replace(value.get().expect("expected gtk::Button")); }
            }
            #[string(readwrite)]
            "name" => {
                get { "foobar".to_value() }
            }
        }
    }
}

fn main() {
    gtk::init().unwrap();
    let my_object = MyClass::new();
    my_object.set_property("ok-button", gtk::Button::builder().label("Accept").build());
    println!(
        "Value of the ok-button property: {:?}",
        my_object.property::<gtk::Button>("ok-button")
    );
}
