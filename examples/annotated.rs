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
        // `properties!` must be called within a `impl ObjectImpl` block.
        //
        // It implements the three functions necessary for properties to work:
        // - fn properties() -> &'static [ParamSpec]
        // - fn property(&self, &Self::Type, usize, ParamSpec) -> Value
        // - set_property(&self, &Self::Type, usize, Value, ParamSpec)
        //
        // `property` and `set_property` use the `id` to figure out which
        //
        properties! {
            // Declares a property named "name", having:
            // - param type: ParamSpecString
            // - Nick: "Object Name"
            // - Blurb: "name of this object"
            // - Flags: G_PARAM_READWRITE | G_PARAM_CONSTRUCT | G_PARAM_EXPLICIT_NOTIFY

            /// Name of this object
            #[string(construct, explicit_notify, nick = "Object Name")]
            "name" => {
                // The 'get' and 'set' blocks will become separate match arms
                // within `fn property` and `fn set_property` respectively.
                // They have access to the following parameters of said functions:
                // - self: the "inner" object
                // - object: the "outer" object
                // - id: current property id
                // - pspec: current property spec

                // The 'get' block must evaluate to a `glib::Value`
                get {
                    self.name.borrow().clone().to_value()
                }

                // The 'set' block has access to the `value` parameter, and must evaluate to `()`.
                set {
                    let old_value = self.name.borrow().clone();
                    self.name.replace(value.get().unwrap());

                    // With the explicit_notify flag, we can for example notify only if the
                    // property value really changed:
                    if self.name.borrow().as_str() != old_value.as_str() {
                        object.notify("name");
                    }
                }
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
