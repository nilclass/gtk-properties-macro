use gtk_properties_macro::properties;

struct MyDialog {
    ok_button: RefCell<gtk::Button>,
}

impl ObjectImpl for MyDialog {
    properties! {
        #[object(gtk::Button)]
        "ok-button" => {
            get { self.ok_button.borrow().clone().to_value() }
            set { self.ok_button.replace(value.get().expect("expected gtk::Button")); }
        }
    }
}
