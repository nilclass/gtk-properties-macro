use std::collections::HashSet;
use quote::quote;
use proc_macro2::TokenStream as TS;
use syn::Ident;
use crate::parse::{Property, DeclarationArg};

pub fn property(id: usize, property: Property) -> (TS, Option<TS>, Option<TS>) {
    let mut param_spec = ParamSpec::new(&property);
    let mut getter: Option<TS> = None;
    let mut setter: Option<TS> = None;

    for block in property.blocks.0 {
        let name = block.name.to_string();
        let impl_block = block.block;
        match name.as_str() {
            "get" => {
                if getter.is_some() {
                    panic!("duplicate get");
                }
                getter = Some(quote! { #id => #impl_block });
            }
            "set" => {
                if setter.is_some() {
                    panic!("duplicate set");
                }
                setter = Some(quote! { #id => #impl_block });
            }
            _ => panic!("Unsupported block: {name}")
        }
    }

    match (getter.is_some(), setter.is_some()) {
        (true, false) => param_spec.flag_read_only(),
        (false, true) => param_spec.flag_write_only(),
        (true, true) => param_spec.flag_read_write(),
        (false, false) => panic!("At least one block ('get' or 'set') is required"),
    }

    (param_spec.generate(), getter, setter)
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
enum Flag {
    Readable,
    Writable,
    Readwrite,
    Construct,
    ConstructOnly,
    LaxValidation,
    StaticName,
    Private,
    StaticNick,
    StaticBlurb,
    ExplicitNotify,
    Deprecated,
}

impl Flag {
    fn generate(&self) -> TS {
        match self {
            Flag::Readable => quote! { glib::ParamFlags::READABLE },
            Flag::Writable => quote! { glib::ParamFlags::WRITABLE },
            Flag::Readwrite => quote! { glib::ParamFlags::READWRITE },
            Flag::Construct => quote! { glib::ParamFlags::CONSTRUCT },
            Flag::ConstructOnly => quote! { glib::ParamFlags::CONSTRUCT_ONLY },
            Flag::LaxValidation => quote! { glib::ParamFlags::LAX_VALIDATION },
            Flag::StaticName => quote! { glib::ParamFlags::STATIC_NAME },
            Flag::Private => quote! { glib::ParamFlags::PRIVATE },
            Flag::StaticNick => quote! { glib::ParamFlags::STATIC_NICK },
            Flag::StaticBlurb => quote! { glib::ParamFlags::STATIC_BLURB },
            Flag::ExplicitNotify => quote! { glib::ParamFlags::EXPLICIT_NOTIFY },
            Flag::Deprecated => quote! { glib::ParamFlags::DEPRECATED },
        }
    }

    fn from_ident(ident: &Ident) -> Self {
        let s = ident.to_string();
        match s.as_str() {
            "readable" => Flag::Readable,
            "writable" => Flag::Writable,
            "readwrite" => Flag::Readwrite,
            "construct" => Flag::Construct,
            "construct_only" => Flag::ConstructOnly,
            "lax_validation" => Flag::LaxValidation,
            "static_name" => Flag::StaticName,
            "private" => Flag::Private,
            "static_nick" => Flag::StaticNick,
            "static_blurb" => Flag::StaticBlurb,
            "explicit_notify" => Flag::ExplicitNotify,
            "deprecated" => Flag::Deprecated,
            _ => todo!()
        }
    }
}

enum FlagSource {
    Explicit(Ident),
    Implied,
}

struct ParamSpec {
    builder: TS,
    flags: Vec<(FlagSource, Flag)>,
    docs: Option<String>,
}

impl ParamSpec {
    fn new(property: &Property) -> Self {
        let mut flags = vec![];
        let doc_strings = property.head.doc.iter().map(|doc| doc.value()).collect::<Vec<String>>();
        let docs = if doc_strings.is_empty() { None } else { Some(doc_strings.join("::").trim_start().to_string()) };
        let name = property.name.value();

        if let Some(args) = &property.head.declaration.args {
            for arg in &args.args {
                match arg {
                    DeclarationArg::Tag(tag) => {
                        flags.push((FlagSource::Explicit(tag.clone()), Flag::from_ident(&tag)));
                    },
                    DeclarationArg::KeyVal(key, _, value) => {
                        println!("KEYVAL: {} => ???", key.to_string());
                    },
                }
            }
        }
        

        let builder = match property.head.declaration.tag.as_str() {
            "string" => quote! { ParamSpecString::builder(#name) },
            _ => unimplemented!()
        };

        ParamSpec { builder, flags, docs }
    }

    fn generate(self) -> TS {
        let ParamSpec { builder, flags, docs } = self;
        let mut aspects = vec![];
        if flags.len() > 0 {
            aspects.push(generate_flags(flags));
        }
        if let Some(blurb) = docs {
            aspects.push(quote! { .blurb(#blurb) });
        }
        quote! {
            #builder #(#aspects)*.build()
        }
    }


    fn flag_read_only(&mut self) {
        self.flags.push((FlagSource::Implied, Flag::Readable));
    }

    fn flag_write_only(&mut self) {
        self.flags.push((FlagSource::Implied, Flag::Writable));
    }

    fn flag_read_write(&mut self) {
        self.flags.push((FlagSource::Implied, Flag::Readwrite));
    }
}

fn generate_flags(flags: Vec<(FlagSource, Flag)>) -> TS {
    let mut seen_flags = HashSet::new();
    let flags: Vec<TS> = flags.into_iter()
        .filter(|(_, flag)| {
            if seen_flags.contains(flag) {
                false
            } else {
                seen_flags.insert(*flag);
                true
            }
        })
        .map(|(_, flag)| flag.generate())
        .collect();
    quote! { .flags(#(#flags)|*) }
}
