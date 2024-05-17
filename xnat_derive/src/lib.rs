extern crate proc_macro;
mod uri;
mod version;

use proc_macro::TokenStream;

use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, Attribute, Data, DeriveInput};

use uri::{
    derive_adminuri_validate_paths_by_params, derive_uribuilder_build_matches, derive_uribuilder_parse_params, derive_uribuilder_parse_paths
};
use crate::version::{
    derive_version_parse_root_uri,
    derive_version_parse_legacy
};

/// Alias for `Vec<syn::Attribute>`.
type Attributes = Vec<Attribute>;

/// Generates the methods required to implement a
/// `AdminUri` or `AdminUri` trait, allowing for
/// a type to represent the administrative
/// endpoints available.
#[proc_macro_derive(AdminUri, attributes(adminuri))]
pub fn derive_adminuri(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident,
        data: _,
        generics,
        attrs,
        ..
    } = parse_macro_input!(input as DeriveInput);
    let where_clause = &generics.where_clause;

    let mut gen = quote! {
        impl #generics AdminUri for #ident #generics #where_clause {}
    };

    // Conditionally implement legacy endpoints.
    if derive_version_parse_legacy(&attrs) {
        gen.extend(quote! {
            impl #generics AdminUriLegacy for #ident #generics #where_clause {}
        });
    }
    gen.into()
}

/// Generates the methods required to implement a
/// `UriBuilder` trait, allowing the type to
/// construct URI paths.
#[proc_macro_derive(UriBuilder, attributes(parent, match_path, param))]
pub fn derive_uribuilder(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident,
        data,
        generics,
        attrs,
        ..
    } = parse_macro_input!(input as DeriveInput);
    let where_clause = &generics.where_clause;

    let params = match data {
        Data::Struct(d) => derive_uribuilder_parse_params(&d.fields),
        _ => panic!("enums and unions are not currently supported")
    };
    let match_paths = derive_uribuilder_parse_paths(&attrs, &params);
    derive_adminuri_validate_paths_by_params(&match_paths, &params);
    let match_arms = derive_uribuilder_build_matches(&match_paths, &params);

    let mut gen = quote! {
        impl #generics UriBuilder for #ident #generics #where_clause {
            fn build(&self) -> anyhow::Result<String> {
                match self {
                    #match_arms
                }
            }
        }
    };

    // Construct `with_{param}` methods to allow
    // pre-construction declaration of parameters.
    let mut with_methods = quote! {};
    for param in params {
        let method_name = Ident::new(&format!("with_{}", &param.name), Span::call_site());
        let field_name  = &param.field_name;
        let kind = &param.kind;

        if param.is_option && !param.is_parent {
            with_methods.extend(quote! {
                /// Generated method to set the
                /// `#field_name` of `#ident`
                pub fn #method_name(mut self, value: &#kind) -> Self {
                    self.#field_name = Some(value.to_owned());
                    self
                }
            })
        } else if param.is_option && param.is_parent {
            with_methods.extend(quote! {
                /// Generated method to set the
                /// `#field_name` of `#ident`
                pub fn with_parent(mut self, value: #kind) -> Self {
                    self.#field_name = Some(value);
                    self
                }
            })
        }
    }
    gen.extend(quote! {
        impl #generics #ident #generics #where_clause {
            #with_methods
        }
    });

    // Impl `std::fmt::Display` to qualify
    // builder for being the potential victim of
    // being joined as a parent builder.
    gen.extend(quote! {
        impl #generics std::fmt::Display for #ident #generics #where_clause {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self
                    .build()
                    .expect("build must produce a string"))
            }
        }
    });
    gen.into()
}

/// Generates the methods required to implement a
/// `Version` trait, allowing the type to
/// represent some API version.
#[proc_macro_derive(Version, attributes(version))]
pub fn derive_version(input: TokenStream) -> TokenStream {
    // Get general information related to the
    // derive input, including the raw details,
    // generic declarations and attributes passed
    // through version() calls.
    let DeriveInput {
        ident,
        data: _,
        generics,
        attrs,
        ..
    } = parse_macro_input!(input as DeriveInput);
    let where_clause = &generics.where_clause;

    // Determine the `root_uri` attribute to be
    // passed to the actual derived
    // implementation.
    let root_uri = derive_version_parse_root_uri(&attrs)
        .unwrap_or_else(|_| ident.to_string().to_lowercase());

    quote! {
        impl #generics Version for #ident #generics #where_clause {
            fn root_uri(&self) -> String {
                String::from(#root_uri)
            }
        }
    }.into()
}
