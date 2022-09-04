use syn::{
    braced, parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token, Attribute, Ident, Lit, LitStr, Path, Result, Token,
};

pub struct Properties(pub LooselySeparated<Property>);

impl Parse for Properties {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Properties(input.parse()?))
    }
}

pub struct Property {
    pub head: Head,
    pub name: LitStr,
    pub arrow: Token![=>],
    pub brace: token::Brace,
    pub blocks: LooselySeparated<Block>,
}

impl Parse for Property {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(Property {
            head: input.parse()?,
            name: input.parse()?,
            arrow: input.parse()?,
            brace: braced!(content in input),
            blocks: content.parse()?,
        })
    }
}

struct DocString {
    _eq: Token![=],
    value: LitStr,
}

impl Parse for DocString {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(DocString {
            _eq: input.parse()?,
            value: input.parse()?,
        })
    }
}

pub struct Head {
    pub doc: Vec<LitStr>,
    pub declaration: Declaration,
}

impl Parse for Head {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut doc = vec![];
        let mut declaration = None;
        let attrs: Vec<Attribute> = input.call(Attribute::parse_outer)?;
        for attr in attrs {
            let path = join_path(&attr.path);
            if path.as_str() == "doc" {
                let doc_string: DocString = syn::parse2(attr.tokens)?;
                doc.push(doc_string.value);
            } else {
                if declaration.is_some() {
                    let span = attr.path.segments.iter().next().unwrap().ident.span();
                    return Err(syn::Error::new(span, format!("Duplicate type declaration")));
                }
                declaration = Some(Declaration {
                    tag: path,
                    args: if attr.tokens.is_empty() {
                        None
                    } else {
                        Some(syn::parse2(attr.tokens)?)
                    },
                });
            }
        }
        if declaration.is_none() {
            Err(syn::Error::new(
                input.span(),
                format!("Missing declaration!"),
            ))
        } else {
            Ok(Head {
                doc,
                declaration: declaration.unwrap(),
            })
        }
    }
}

pub fn join_path(path: &Path) -> String {
    path.segments
        .iter()
        .map(|segment| segment.ident.to_string())
        .collect::<Vec<String>>()
        .join("::")
}

pub struct Declaration {
    pub tag: String,
    pub args: Option<DeclarationArgs>,
}

pub struct DeclarationArgs {
    pub paren: token::Paren,
    pub args: Punctuated<DeclarationArg, Token![,]>,
}

impl Parse for DeclarationArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(DeclarationArgs {
            paren: parenthesized!(content in input),
            args: Punctuated::parse_separated_nonempty(&content)?,
        })
    }
}

#[derive(Clone)]
pub enum DeclarationArg {
    // either a flag (readable, construct, ...), or a type name (for properties with ParamSpecObject)
    Tag(Path),
    // key = value flags which compile to `.#key(#value)` calls on the builder
    KeyVal(Path, Token![=], Lit),
}

impl Parse for DeclarationArg {
    fn parse(input: ParseStream) -> Result<Self> {
        let key: Path = input.parse()?;
        if input.peek(Token![=]) {
            let eq: Token![=] = input.parse()?;
            let value: Lit = input.parse()?;
            Ok(DeclarationArg::KeyVal(key, eq, value))
        } else {
            Ok(DeclarationArg::Tag(key))
        }
    }
}

// a "get" or "set" block for a single property
pub struct Block {
    pub name: Ident,
    pub block: syn::Block,
}

impl Parse for Block {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Block {
            name: input.parse()?,
            block: input.parse()?,
        })
    }
}

// Parses the entire stream, as a sequence of `T`s.
// If there is a comma (,) or semicolon (;) between
// occurrences, that's fine, but not required.
pub struct LooselySeparated<T: Parse>(pub Vec<T>);

impl<T: Parse> Parse for LooselySeparated<T> {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut items = vec![];
        while !input.is_empty() {
            items.push(input.parse()?);
            if input.peek(Token![,]) {
                let _: Token![,] = input.parse()?;
            } else if input.peek(Token![;]) {
                let _: Token![;] = input.parse()?;
            }
        }
        Ok(LooselySeparated(items))
    }
}
