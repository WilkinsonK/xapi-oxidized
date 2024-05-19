use std::collections::HashSet;

use attribute_derive::FromAttr;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    parse_str, punctuated::Punctuated, Attribute, Expr, Fields, GenericArgument, Ident, PathArguments, Type
};

use crate::Attributes;

macro_rules! new_ambiguous_ident {
    ($e:expr) => {
        Ident::new($e, Span::call_site())
    };
}

/// Represents attributes passed to `UriBuilder`
/// for building some URI path
#[derive(FromAttr, Debug)]
#[attribute(ident = match_path)]
#[attribute(error(missing_field = "`{field}` not specified"))]
struct MatchPatternAttrs {
    path: String,
}

/// Represents attributes passed to `UriBuilder`
/// for building path patterns.
#[derive(Debug, PartialEq, Eq)]
pub struct MatchPatternAttrsParsed {
    pub path:   String,
    pub params: Vec<ParamAttrsParsed>,
}

/// Represents attributes passed to the fields of
/// a `UriBuilder` implemented struct.
#[derive(FromAttr, Debug)]
#[attribute(ident = param, aliases=[parent, root])]
#[attribute(error(missing_field = "`{field}` not specified"))]
struct ParamAttrs {
    pub name:      Option<String>,
    pub map_from:  Option<String>,
    pub is_parent: Option<bool>,
}

/// Public facing struct from `ParamAttrs`
/// parsing.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParamAttrsParsed {
    pub field_name: Ident,
    pub name:       String,
    pub kind:       Type,
    pub map_from:   Option<Expr>,
    pub is_option:  bool,
    pub is_parent:  bool,
}

/// Builds the match arms for the `build` URI
/// builder table.
pub fn derive_uribuilder_build_matches(paths: &[MatchPatternAttrsParsed], params: &[ParamAttrsParsed]) -> TokenStream {
    let mut match_arms = quote! {};
    for pattern in paths {
        // Edge case where a pattern presented
        // requires no parameters.
        if pattern.params.is_empty() {
            match_arms.extend(parse_match_arm_shallow(pattern, params));
        // Inclusive formatting of a pattern where
        // fields present themselves as `Some` and
        // excluding param fields that are `None`
        } else {
            match_arms.extend(parse_match_arm(pattern, params));
        }
    }
    match_arms.extend(quote! {
        _ => {
            Err(crate::uri::UriBuildError::UnrecognizedPattern.into())
        }
    });

    match_arms
}

/// Parse the path patterns declared on a
/// `UriBuilder` derived implementation.
pub fn derive_uribuilder_parse_paths(attrs: &Attributes, params: &[ParamAttrsParsed]) -> Vec<MatchPatternAttrsParsed> {
    attrs
        .iter()
        .filter(filter_match_paths)
        .map(MatchPatternAttrs::from_attribute)
        .map(|a| {
            let mut parsed = MatchPatternAttrsParsed{
                path: a.unwrap().path,
                params: vec![]
            };

            // Pair parameter metadata to the
            // path.
            parsed
                .path
                .split('/')
                .filter(|p| p.contains(['{', '}']))
                .map(|param_name| param_name.replace(['{', '}'], ""))
                .for_each(|param_name| {
                    let found = params
                        .iter()
                        .find(|p| p.name == param_name);
                    if let Some(f) = found {
                        parsed.params.push(f.to_owned())
                    }
                });

            parsed
        })
        .collect()
}

/// Parse the param declared fields on  a
/// `UriBuilder` derived implemenation.
pub fn derive_uribuilder_parse_params(fields: &Fields) -> Vec<ParamAttrsParsed> {
    match fields {
        Fields::Named(f) => {
            f.named.to_owned()
        },
        Fields::Unit => Punctuated::new(),
        _ => panic!("unnamed fields are not currently supported")
    }
    .iter()
    .filter_map(|f| {
        let ident = f.ident.clone().unwrap();
        let attrs = f
            .attrs
            .iter()
            .find(filter_params)
            .and_then(|a| {
                let mut parsed = ParamAttrs::from_attribute(a)
                    .unwrap();
                parsed.is_parent = parse_attr_is_parent(a).into();
                parsed.into()
            });
        let attrs = attrs?;
        let kind  = f.ty.clone();

        let is_parent = attrs.is_parent.unwrap_or_default();
        let is_option = parse_is_optional_type(&kind);
        let name = attrs.name.unwrap_or(ident.to_string());
        let kind = if is_option {
            parse_optional_type(&kind)
        } else {
            &kind
        }.to_owned();
        let map_from = attrs
            .map_from
            .map(|mf| parse_str::<Expr>(&mf).expect("must be a parsable expression"));

        Some(ParamAttrsParsed {
            field_name: ident,
            name,
            kind,
            map_from,
            is_option,
            is_parent,
        })
    })
    .collect()
}


/// Validate all params have been declared
/// inline with path patterns.
pub fn derive_uribuilder_validate_paths_by_params(paths: &[MatchPatternAttrsParsed], params: &[ParamAttrsParsed]) {
    let mut param_map = HashSet::new();
    for match_path in paths.iter() {
        match_path.path.split('/').for_each(|p| {
            if p.contains(['{', '}']) {
                param_map.insert(p.replace(['{', '}'], ""));
            }
        })
    }
    if !param_map.iter().all(|n| params.iter().any(|p| p.name == *n)) {
        panic!("missing parameter(s) declared in path patterns")
    }
}

fn filter_match_paths(attr: &&Attribute) -> bool {
    attr.meta.path().segments[0].ident == "match_path"
}

fn filter_params(attr: &&Attribute) -> bool {
    ["param", "parent"]
        .contains(&attr.meta.path().segments[0].ident.to_string().as_str())
}

fn parse_is_optional_type(kind: &Type) -> bool {
    match &kind {
        Type::Path(tp) => tp.path.segments[0].ident == "Option",
        _ => panic!("expected a type path")
    }
}

fn parse_attr_is_parent(attr: &Attribute) -> bool {
    attr.meta.path().segments[0].ident == "parent"
}

fn parse_match_arm(pattern: &MatchPatternAttrsParsed, params: &[ParamAttrsParsed]) -> TokenStream {
    let path = &pattern.path;
    let mut lhs = quote! {};
    let mut rhs = quote! {};
    // We must break out the inner impl of the RHS
    // in order to allow field mapping where it is
    // required.
    let mut rhs_inner = quote! {};

    params.iter().enumerate().for_each(|(idx, p)| {
        let field_name = &p.field_name;
        let param_name = new_ambiguous_ident!(&p.name);
        let index_name = new_ambiguous_ident!(&format!("p{idx}"));
        // Simply apply the parameter field if no
        // special formatting is required.
        if !pattern.params.contains(p) {
            lhs.extend(quote! { #field_name: None, });
            return
        }
        // Apply mapper function to format a value
        // from user-defined func.
        if let Some(mf) = &p.map_from {
            lhs.extend(quote! { #field_name: Some(#index_name), });
            rhs_inner.extend(quote! {
                let mapper = #mf; let #param_name = mapper(#index_name);
            });
        } else {
            lhs.extend(quote! { #field_name: Some(#param_name), });
        }
    });
    // Finalize RHS after determination of
    // whether extra formatting rules are
    // required.
    rhs.extend(quote! {
        {
            #rhs_inner format!(#path)
        }
    });
    // Round off match pattern by ignoring any
    // non-parameter fields.
    lhs.extend(quote! { .. });
    // Construct the full match arm from the left
    // and right hand sides.
    let gen = quote! { Self { #lhs } => Ok(#rhs), };
    gen
}

fn parse_match_arm_shallow(pattern: &MatchPatternAttrsParsed, params: &[ParamAttrsParsed]) -> TokenStream {
    let path = &pattern.path;
    let mut lhs = quote! {};
    let rhs = quote! { String::from(#path) };
    params.iter().for_each(|p| {
        let field_name = &p.field_name;
        lhs.extend(quote! { #field_name: None, })
    });
    lhs.extend(quote! { .. });
    quote! { Self { #lhs } => Ok(#rhs), }
}

fn parse_optional_type(kind: &Type) -> &Type {
    // If the initial `kind` parsed from
    // the field is an Option, we want
    // to instead get the underlying type.
    match &kind {
        Type::Path(tp) => {
            // Assuming the syntax is the
            // expected pattern, we can
            // then extract the underlying
            // type.
            match &tp.path.segments[0].arguments {
                PathArguments::AngleBracketed(ab) => {
                    match &ab.args[0] {
                        GenericArgument::Type(ty) => ty,
                        _ => panic!("expected a type")
                    }
                },
                _ => panic!("unexpected syntax")
            }
        },
        _ => unreachable!()
    }
}
