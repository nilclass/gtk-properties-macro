use crate::parse::{join_path, DeclarationArg, Property};
use proc_macro2::TokenStream as TS;
use quote::{format_ident, quote};
use std::collections::HashSet;
use syn::{Path, spanned::Spanned};

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
            _ => panic!("Unsupported block: {name}"),
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

enum FlagSource {
    Explicit(Path),
    Implied,
}

struct ParamSpec {
    name: String,
    builder: TS,
    builder_steps: Vec<TS>,
    flags: Vec<(FlagSource, Flag)>,
    docs: Option<String>,
}

impl ParamSpec {
    fn new(property: &Property) -> Self {
        let mut flags: Vec<(FlagSource, Flag)> = vec![];
        let mut builder_steps: Vec<TS> = vec![];
        let doc_strings = property
            .head
            .doc
            .iter()
            .map(|doc| doc.value())
            .collect::<Vec<String>>();
        let docs = if doc_strings.is_empty() {
            None
        } else {
            Some(doc_strings.join("\n").trim_start().to_string())
        };
        let name = property.name.value();

        let mut args: Vec<DeclarationArg> = property
            .head
            .declaration
            .args
            .as_ref()
            .map(|args| args.args.iter().cloned().collect())
            .unwrap_or_default();

        let builder = {
            let type_tag = property.head.declaration.tag.as_str();
            match type_tag {
                "boolean" | "char" | "double" | "float" | "int" | "int64" | "long" | "string" => {
                    let type_name = format_ident!(
                        "ParamSpec{}{}",
                        &type_tag[0..1].to_uppercase(),
                        &type_tag[1..]
                    );
                    quote! { #type_name::builder(#name) }
                }
                "object" => {
                    if args.len() == 0 {
                        panic!(
                            "property of type 'object' requires an object type as first argument"
                        );
                    }
                    let object_type = if let DeclarationArg::Tag(tag) = args.remove(0) {
                        tag
                    } else {
                        panic!("Expected object type, not key/val")
                    };
                    quote! { ParamSpecObject::builder(#name, #object_type::static_type()) }
                }
                _ => unimplemented!("not yet implemented: {}", type_tag),
            }
        };

        for arg in &args {
            match arg {
                DeclarationArg::Tag(tag) => {
                    flags.push((FlagSource::Explicit(tag.clone()), Flag::from_path(&tag)));
                }
                DeclarationArg::KeyVal(key, _, value) => {
                    builder_steps.push(quote! { .#key(#value) });
                }
            }
        }

        ParamSpec {
            name,
            builder,
            builder_steps,
            flags,
            docs,
        }
    }

    fn generate(self) -> TS {
        let ParamSpec {
            name: _,
            builder,
            builder_steps,
            flags,
            docs,
        } = self;
        let mut aspects = vec![];
        if flags.len() > 0 {
            aspects.push(generate_flags(flags));
        }
        if let Some(blurb) = docs {
            aspects.push(quote! { .blurb(#blurb) });
        }
        quote! {
            #builder #(#aspects)* #(#builder_steps)* .build()
        }
    }

    fn flag_read_only(&mut self) {
        self.check_flag_conflict("set", self.flags.iter().find(|(_, flag)| {
            match *flag {
                Flag::Writable | Flag::Readwrite | Flag::Construct | Flag::ConstructOnly => true,
                _ => false
            }
        }));
        self.flags.push((FlagSource::Implied, Flag::Readable));
    }

    fn flag_write_only(&mut self) {
        self.check_flag_conflict("get", self.flags.iter().find(|(_, flag)| {
            match *flag {
                Flag::Readable | Flag::Readwrite => true,
                _ => false
            }
        }));
        self.flags.push((FlagSource::Implied, Flag::Writable));
    }

    fn flag_read_write(&mut self) {
        self.flags.push((FlagSource::Implied, Flag::Readwrite));
    }

    fn check_flag_conflict(&self, block_name: &str, conflict: Option<&(FlagSource, Flag)>) {
        if let Some((FlagSource::Explicit(source), _)) = conflict {
            let flag_name = join_path(source);
            source.span().unwrap()
                .error(format!("Property {:?} is marked {}, but does not have a '{block_name}' block", self.name, flag_name))
                .help(format!("Remove {:?} flag, or add a '{block_name}' block below", flag_name))
                .emit();
        }
    }
}

fn generate_flags(flags: Vec<(FlagSource, Flag)>) -> TS {
    let mut seen_flags = HashSet::new();
    let flags: Vec<TS> = flags
        .into_iter()
        .filter(|(_, flag)| {
            if seen_flags.contains(flag) {
                false
            } else {
                seen_flags.insert(*flag);
                true
            }
        })
        .map(|(_, flag)| flag.to_token_stream())
        .collect();
    quote! { .flags(#(#flags)|*) }
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
    fn to_token_stream(&self) -> TS {
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

    fn from_path(path: &Path) -> Self {
        let s = join_path(path);
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
            _ => unimplemented!("Unsupported flag: {}", s),
        }
    }
}
