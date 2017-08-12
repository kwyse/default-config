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
                                quote! { #ident: #value.parse().unwrap_or_default() }
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
