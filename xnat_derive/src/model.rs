use proc_macro::TokenStream as TokenStream1;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, GenericParam, Lifetime, LifetimeParam};

use crate::get_crate_ident;

macro_rules! new_ambiguous_lifetime {
    ($i:literal) => {
        GenericParam::Lifetime(LifetimeParam::new(Lifetime::new($i, Span::call_site())))
    };
}

/// Build a derived implementation of the target
/// tuple struct for `ModelProperty` required
/// traits.
pub fn build_property(input: TokenStream1) -> TokenStream1 {
    let DeriveInput {
        generics,
        ident,
        data,
        ..
    } = parse_macro_input!(input as DeriveInput);
    let where_clause = &generics.where_clause;

    let field = match data {
        Data::Struct(d) => {
            match d.fields {
                Fields::Unnamed(f) => {
                    f.unnamed.first().cloned().expect("at least one field")
                },
                _ => panic!("non-unit structs are not currently supported")
            }
        },
        Data::Enum(_) => panic!("enums are not currently supported"),
        Data::Union(_) => panic!("unions are not currently supported")
    };

    let crate_ident = get_crate_ident();
    let visitor = quote! {
        #crate_ident::models::common::ModelPropertyVisitor::<#field>
    };

    let mut generics_lhs = generics.clone();
    let de_lifetime = new_ambiguous_lifetime!("'de");
    generics_lhs.params.insert(0, de_lifetime.clone());

    quote! {
        impl #generics_lhs serde::Deserialize<#de_lifetime> for #ident #generics #where_clause {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>
            {
                Ok(Self(deserializer.deserialize_any(#visitor::default())?))
            }
        }

        impl #generics #crate_ident::models::common::ModelField<#field> for #ident #generics
            #where_clause
        {
            fn property(&self) -> &#field {
                &self.0
            }
        }
    }.into()
}
