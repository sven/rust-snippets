//! Example crate to demonstrate parsing of structure fields and attributes
//!
//! After the parsing is done a new method is added to the marked struct named "print_meta". This
//! new method can then be called by the real application and prints out the struct name, all
//! member fields and their assigned attributes.
//!
//! ```no_run
//! #[derive(DeriveStruct, Default)]
//! struct ParseMe {
//!     #[derive_struct_attr(name = "ElementString", no = 123)]
//!     element_string: String,
//!     #[derive_struct_attr(name = "ElementU8")]
//!     element_u8: u8,
//! }
//! ```
//!
//! Calling the `print_meta()` method on this structure will give this result:
//!
//! ```no_run
//! Struct name: ParseMe
//!   * Field: element_string
//!     * Attribute: name = Str(LitStr { token: "ElementString" })
//!     * Attribute: no = Int(LitInt { token: 123 })
//!   * Field: element_u8
//!     * Attribute: name = Str(LitStr { token: "ElementU8" })
//! ```

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::Lit;
use syn::{braced, parse_macro_input, token, Attribute, Ident, Result, Token, Type};

/// Field attribute data
/// #[derive_struct(key = value, key = value, ...)]
struct ItemFieldAttr {
    /// Field attribute key name
    ident: Ident,

    /// Unused equal token
    _equal: Token![=],

    /// Field attribute value literal
    value: Lit,
}

/// Field data
struct ItemField {
    /// Field attributes and doc comments
    attrs: Vec<ItemFieldAttr>,

    /// Unused public token
    _pub_token: Option<Token![pub]>,

    /// Unused field separator
    _colon_token: Token![:],

    /// Field name
    ident: Ident,

    /// Unused field type
    _ty: Type,
}

/// Struct item description
struct ItemStruct {
    /// Unused struct attributes and doc comments
    _attrs: Vec<Attribute>,

    /// Unused optional public keyword
    _pub_token: Option<Token![pub]>,

    /// Unused struct keyword
    _struct_token: Token![struct],

    /// Struct name
    ident: Ident,

    /// Unused brace token
    _brace_token: token::Brace,

    /// Struct fields separated by commas
    fields: Punctuated<ItemField, Token![,]>,
}

impl Parse for ItemFieldAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(ItemFieldAttr {
            ident: input.parse()?,
            _equal: input.parse()?,
            value: input.parse()?,
        })
    }
}

impl Parse for ItemField {
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs_raw: Vec<Attribute> = input.call(Attribute::parse_outer)?;
        let mut attrs: Vec<ItemFieldAttr> = Vec::new();

        for attr in attrs_raw {
            // skip unhandled attributes like "doc" etc
            if attr.path.get_ident().unwrap() != "derive_struct_attr" {
                continue;
            }

            attr.parse_args_with(Punctuated::<ItemFieldAttr, Token![,]>::parse_terminated)?
                .into_iter()
                .for_each(|attr| {
                    attrs.push(attr);
                });
        }

        Ok(ItemField {
            attrs,
            _pub_token: input.parse()?,
            ident: input.parse()?,
            _colon_token: input.parse()?,
            _ty: input.parse()?,
        })
    }
}

impl Parse for ItemStruct {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(ItemStruct {
            _attrs: input.call(Attribute::parse_outer)?,
            _pub_token: input.parse()?,
            _struct_token: input.parse()?,
            ident: input.parse()?,
            _brace_token: braced!(content in input),
            fields: content.parse_terminated(ItemField::parse)?,
        })
    }
}

/// Parse a struct and its member fields
#[proc_macro_derive(DeriveStruct, attributes(derive_struct_attr))]
pub fn derive_struct(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as ItemStruct);

    let mut res = format!("Struct name: {}\n", input.ident);
    for field in input.fields {
        res += &format!("  * Field: {}\n", field.ident);
        for attr in field.attrs {
            res += &format!("    * Attribute: {} = {:?}\n", attr.ident, attr.value);
        }
    }

    let struct_name = input.ident;
    let result = quote! {
        impl #struct_name {
            fn print_meta(&self) {
                println!("{}", #res);
            }
        }
    };

    result.into()
}
