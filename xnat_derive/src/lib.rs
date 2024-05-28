extern crate proc_macro;
mod uri;
mod version;

use proc_macro::TokenStream;

use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, Attribute, DeriveInput};

use uri::uribuilder;
use crate::version::{
    derive_version_parse_legacy, derive_version_parse_root_uri, derive_version_parse_root_uri_legacy
};

/// Shortcut to avoid repetive usage of the same
/// derive parsing boilerplate. Exposes the
/// declared fields from an input parsed as
/// `syn::DeriveInput`
macro_rules! derive_input_boilerplate {
    ($($field:ident),+ $(,)?; from $input:ident) => {
        let DeriveInput {
            $(
                $field,
            )+
            ..
        } = parse_macro_input!($input as DeriveInput);
    };
}

/// Alias for `Vec<syn::Attribute>`.
type Attributes = Vec<Attribute>;

/// Generates the methods required to implement a
/// `AdminUri` or `AdminUriLegacy` trait, allowing
/// for a type to represent the administrative
/// endpoints available.
#[proc_macro_derive(AdminUri)]
pub fn derive_adminuri(input: TokenStream) -> TokenStream {
    derive_input_boilerplate!(generics, ident, attrs; from input);
    let where_clause = &generics.where_clause;

    // Conditionally implement legacy endpoints.
    let mut gen = quote! {};
    if !derive_version_parse_legacy(&attrs) {
        gen.extend(quote! {
            impl #generics AdminUri for #ident #generics #where_clause {}
        });
    }
    gen.extend(quote! {
        impl #generics AdminUriLegacy for #ident #generics #where_clause {}
    });
    gen.into()
}

/// Generates the methods required to implement a
/// `AuthUri` trait, allowing for a type to
/// represent the user authentication endpoints.
#[proc_macro_derive(AuthUri)]
pub fn derive_authuri(input: TokenStream) -> TokenStream {
    derive_input_boilerplate!(generics, ident; from input);
    let where_clause = &generics.where_clause;

    quote! {
        impl #generics AuthUriLegacy for #ident #generics #where_clause {}
    }.into()
}

/// Generates the methods required to implement a
/// `EventsUri` trait, allowing for a type to
/// represent the XNAT event system.
#[proc_macro_derive(EventUri)]
pub fn derive_eventuri(input: TokenStream) -> TokenStream {
    derive_input_boilerplate!(generics, ident; from input);
    let where_clause = &generics.where_clause;

    quote! {
        impl #generics EventsUri for #ident #generics #where_clause {}
    }.into()
}

/// Generates the methods required to implement a
/// `SystemUri` trait, allowing for a type to
/// represent the administrative endpoints
/// available.
#[proc_macro_derive(SystemUri)]
pub fn derive_sysuri(input: TokenStream) -> TokenStream {
    derive_input_boilerplate!(generics, ident; from input);
    let where_clause = &generics.where_clause;

    quote! {
        impl #generics SystemUri for #ident #generics #where_clause {}
    }.into()
}

/// Generates the methods required to implement a
/// `UsersUri` trait, allowing for a type to
/// represent the user administrative endpoints
/// available.
#[proc_macro_derive(UsersUri)]
pub fn derive_usersuri(input: TokenStream) -> TokenStream {
    derive_input_boilerplate!(attrs, generics, ident; from input);
    let where_clause = &generics.where_clause;

    let mut gen = quote! {};
    if !derive_version_parse_legacy(&attrs) {
        gen.extend(quote! {
            impl #generics UsersUri for #ident #generics #where_clause {}
        });
    }
    gen.extend(quote! {
        impl #generics UsersUriLegacy for #ident #generics #where_clause {}
    });
    gen.into()
}

/// Generates an alias for `UriBuilder` and other
/// common traits required by subsequent
/// implementations.
///
/// Specifically, generates the alias from the
/// given `name` and then produces a declarative
/// macro, derived from the `name`.
/// 
/// 
/// e.g.
/// ```no_compile
/// use oxinat_derive::uri_builder_alias;
/// 
/// uri_builder_alias!(AliasedUriBuilder);
/// // Supports non-generics as a single pattern.
/// ImplAliasedUriBuilder! {
///     (String),
///     .. // variadic declarations.
/// }
/// // patterns that require generics need to
/// // currently be declared separately...
/// ImplAliasedUriBuilder! {
///     (TypeToImpAliasedUriBuilder<Parent>, Parent),
///     .. // variadic declarations.
/// }
/// ```
#[proc_macro]
pub fn uri_builder_alias(input: TokenStream) -> TokenStream {
    let ident = parse_macro_input!(input as Ident);
    let trait_doc = "This is an alias trait for traits common in subsequent implmentations.";
    let impl_doc  = &format!("Generate implementations of `{ident}`.");
    let impl_name = Ident::new(&format!("Impl{ident}"), Span::call_site());
    quote! {
        #[doc=#trait_doc]
        pub trait #ident: UriBuilder + Clone + Debug {}
        #[doc=#impl_doc]
        macro_rules! #impl_name {
            ($(($kind:ty)),+ $(,)?) => {
                $(impl #ident for $kind {})+
            };
            ($(($kind:ty, $parent:ident)),+ $(,)?) => {
                $( 
                    impl<$parent> #ident for $kind
                    where
                        $parent: #ident,
                    {}
                )+
            };
        }
    }.into()
}

/// Generates the methods required to implement a
/// `UriBuilder` trait, allowing the type to
/// construct URI paths.
/// 
/// Currently we do not support the implementation
/// against unions.
#[proc_macro_derive(UriBuilder, attributes(parent, match_path, param, validator))]
pub fn derive_uribuilder(input: TokenStream) -> TokenStream {
    uribuilder::build(input)
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
    derive_input_boilerplate!(attrs, generics, ident; from input);
    let where_clause = &generics.where_clause;

    // Determine the `root_uri` attribute to be
    // passed to the actual derived
    // implementation.
    let root_uri = derive_version_parse_root_uri(&attrs)
        .unwrap_or_else(|_| ident.to_string().to_lowercase());
    let root_uri_legacy = derive_version_parse_root_uri_legacy(&attrs).unwrap();

    quote! {
        impl #generics Version for #ident #generics #where_clause {
            fn root_uri(&self) -> String {
                String::from(#root_uri)
            }

            fn root_uri_legacy(&self) -> String {
                String::from(#root_uri_legacy)
            }
        }
    }.into()
}
