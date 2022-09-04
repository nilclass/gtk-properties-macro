use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

mod generate;
mod parse;

#[proc_macro]
pub fn properties(tokens: TokenStream) -> TokenStream {
    let properties = parse_macro_input!(tokens as parse::Properties);

    let mut param_specs: Vec<proc_macro2::TokenStream> = vec![];
    let mut getters: Vec<proc_macro2::TokenStream> = vec![];
    let mut setters: Vec<proc_macro2::TokenStream> = vec![];

    for (index, property) in properties.into_iter().enumerate() {
        let id = index + 1;
        let (param_spec, getter, setter) = generate::property(id, property);
        param_specs.push(param_spec);
        if let Some(getter) = getter {
            getters.push(getter);
        }
        if let Some(setter) = setter {
            setters.push(setter);
        }
    }

    TokenStream::from(quote! {
        fn properties() -> &'static [gtk::glib::ParamSpec] {
            use once_cell::sync::Lazy;
            use gtk::glib::*;
            static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                vec![#(#param_specs),*]
            });
            PROPERTIES.as_ref()
        }

        fn property(&self, object: &Self::Type, id: usize, pspec: &gtk::glib::ParamSpec) -> gtk::glib::Value {
            use gtk::glib::prelude::*;
            match id {
                #(#getters),*
                _ => unimplemented!()
            }
        }

        fn set_property(&self, object: &Self::Type, id: usize, value: &gtk::glib::Value, pspec: &gtk::glib::ParamSpec) {
            use gtk::glib::prelude::*;
            match id {
                #(#setters),*
                _ => unimplemented!()
            }
        }
    })
}

#[test]
fn test_expansion() {
    macrotest::expand("tests/expand/*.rs");
}
