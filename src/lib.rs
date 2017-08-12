//! `specified_default_derive` enables you to choose the defaults implemented
//! by the `Default` trait. After it is derived, the object will behave
//! exactly as it would had the standard `Default` trait been derived. Nested
//! objects must implement either `SpecifiedDefault` or `Default`.
//!
//! If you don't provide an override, the existing default value for that
//! type will be used.
//!
//! # Examples
//!
//! ```
//! # #[macro_use] extern crate specified_default_derive;
//! #
//! # fn main() {
//! #[derive(SpecifiedDefault)]
//! struct MyStruct {
//!     #[default = "640"]
//!     width: u32,
//!     #[default = "480"]
//!     height: u32,
//!
//!     scenes: u32,
//! }
//!
//! let result = MyStruct::default();
//! assert_eq!(result.width, 640);
//! assert_eq!(result.height, 480);
//! assert_eq!(result.scenes, 0);
//! # }
//! ```
extern crate proc_macro;
#[macro_use] extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use syn::{Body, VariantData};

#[doc(hidden)]
#[proc_macro_derive(SpecifiedDefault, attributes(default))]
pub fn specify_defaults(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();
    let gen = impl_specified_defaults(&ast);

    gen.parse().unwrap()
}

fn impl_specified_defaults(ast: &syn::DeriveInput) -> quote::Tokens {
    if let Body::Struct(VariantData::Struct(ref fields)) = ast.body {
        const ATTRIBUTE_NAME: &'static str = "default";

        let fields = fields.iter()
            .map(|field| {
                let ident = field.ident.as_ref();
                let attrs = field.attrs.clone();

                match attrs.iter().find(|attr| attr.value.name() == ATTRIBUTE_NAME) {
                    Some(attr) => {
                        if let syn::MetaItem::NameValue(_, ref lit) = attr.value {
                            if let syn::Lit::Str(ref value, _) = *lit {
                                quote! { #ident: #value.parse().expect(&format!("Failed to parse {}", #value)) }
                            } else {
                                panic!("#[derive(SpecifiedDefault)] only supports string literal attributes");
                            }
                        } else {
                            panic!("#[derive(SpecifiedDefault)] only supports named value attributes");
                        }
                    },
                    None => quote! { #ident: Default::default() }
                }
            })
            .collect::<Vec<_>>();

        let name = &ast.ident;
        quote! {
            impl Default for #name {
                fn default() -> #name {
                    #name {
                        #(#fields),*
                    }
                }
            }
        }
    } else {
        panic!("#[derive(SpecifiedDefault)] only supports structs");
    }
}
