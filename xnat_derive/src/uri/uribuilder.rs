use std::collections::HashSet;
use proc_macro::TokenStream as TokenStream1;

use attribute_derive::FromAttr;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    parse_macro_input, parse_str, punctuated::Punctuated, Attribute, Data, DataEnum, DataStruct, DeriveInput, Expr, Fields, GenericArgument, Ident, PathArguments, Type
};

use crate::Attributes;

macro_rules! new_ambiguous_ident {
    ($e:expr, $($vars:expr),+ $(,)?) => {
        Ident::new(
            &format!($e, $($vars,)+),
            Span::call_site()
        )
    };
    ($e:expr) => {
        Ident::new($e, Span::call_site())
    }
}

/// Represents attributes passed to `UriBuilder`
/// for building some URI path
#[derive(FromAttr, Debug)]
#[attribute(ident = match_path)]
#[attribute(error(missing_field = "`{field}` not specified"))]
struct MatchPatternAttrs {
    path:     String,
    requires: Option<String>,
}

/// Represents attributes passed to `UriBuilder`
/// for building path patterns.
#[derive(Debug, PartialEq, Eq)]
pub struct MatchPatternAttrsParsed {
    pub path:     String,
    pub params:   Vec<ParamAttrsParsed>,
    pub requires: Option<Expr>
}

/// Represents attributes passed to the fields of
/// a `UriBuilder` implemented struct.
#[derive(FromAttr, Debug, Default)]
#[attribute(ident = param, aliases=[parent, root])]
#[attribute(error(missing_field = "`{field}` not specified"))]
struct ParamAttrs {
    pub name:      Option<String>,
    pub map_from:  Option<String>,
    pub requires:  Option<String>,
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
    pub requires:   Option<Expr>,
    pub is_option:  bool,
    pub is_param:   bool,
    pub is_parent:  bool,
}

/// Build a derived implementation of the target
/// struct or enum type for a `UriBuilder`.
pub fn build(input: TokenStream1) -> TokenStream1 {
    let input = parse_macro_input!(input as DeriveInput);
    match &input.data {
        Data::Struct(data) => build_struct(&input, data),
        Data::Enum(data)   => build_enum(&input, data),
        _ => panic!("unions are not supported")
    }.into()
}

fn build_enum(input: &DeriveInput, data: &DataEnum) -> TokenStream {
    let mut match_arms = quote! {};
    data.variants.iter().for_each(|variant| {
        let params      = parse_params(&variant.fields);
        let match_paths = parse_paths(&variant.attrs, &params);
        validate_paths_by_params(&match_paths, &params);

        let ident = &variant.ident;
        if params.is_empty() {
            // We will treat non-parameterized
            // variants with a general,
            // homogenous, case.
            let default_uri = ident.to_string().to_lowercase();
            match_arms.extend(quote! { Self::#ident => #default_uri.to_string(), });
            return;
        }
        for pattern in match_paths {
            let path = &pattern.path;
            let mut lhs = quote! {};
            let rhs = quote! { format!(#path) };

            params.iter().enumerate().for_each(|(idx, p)| {
                let index_name = new_ambiguous_ident!("p{}", idx);
                if p.is_option && pattern.params.contains(p) {
                    lhs.extend(quote! { Some(#index_name), })
                } else if p.is_option {
                    lhs.extend(quote! { None, })
                } else {
                    lhs.extend(quote! { #index_name, })
                }
            });
            match_arms.extend(quote! { Self::#ident(#lhs) => #rhs, })
        }
    });

    let (ident, generics) = (&input.ident, &input.generics);
    let where_clause      = &generics.where_clause;
    let gen = quote! {
        impl #generics UriBuilder for #ident #generics #where_clause {
            fn build(&self) -> anyhow::Result<String> {
                Ok(self.to_string().into())
            }
        }

        impl #generics std::fmt::Display for #ident #generics #where_clause {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
            {
                write!(f, "{}", match self {
                    #match_arms
                })
            }
        }
    };
    gen
}

/// Implements necessary methods for a struct to
/// qualify as a `UriBuilder`.
fn build_struct(input: &DeriveInput, data: &DataStruct) -> TokenStream {
    let where_clause = &input.generics.where_clause;
    let params       = parse_params(&data.fields);
    let match_paths  = parse_paths(&input.attrs, &params);
    validate_paths_by_params(&match_paths, &params);

    let (ident, generics) = (&input.ident, &input.generics);
    let match_arms = build_matches(&match_paths, &params);
    let mut gen = quote! {
        impl #generics UriBuilder for #ident #generics #where_clause {
            fn build(&self) -> anyhow::Result<String> {
                match self {
                    #match_arms
                }
            }
        }
    };
    gen.extend(build_methods(input, &params));
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
    gen
}

/// Builds the match arms for the `build` URI
/// builder table.
fn build_matches(paths: &[MatchPatternAttrsParsed], params: &[ParamAttrsParsed]) -> TokenStream {
    let mut match_arms = quote! {};
    for pattern in paths {
        // Edge case where a pattern presented
        // requires no parameters.
        if pattern.params.is_empty() {
            match_arms.extend(build_match_arm_shallow(pattern, params));
        // Inclusive formatting of a pattern where
        // fields present themselves as `Some` and
        // excluding param fields that are `None`
        } else {
            match_arms.extend(build_match_arm(pattern, params));
        }
    }
    match_arms.extend(quote! {
        _ => {
            Err(crate::uri::UriBuildError::UnrecognizedPattern.into())
        }
    });

    match_arms
}

/// Performs a deep construction of a match arm
/// used for the `match` statement responsible for
/// the eventual building of a `URI` from the 
/// implementation of a `UriBuilder`.
fn build_match_arm(pattern: &MatchPatternAttrsParsed, params: &[ParamAttrsParsed]) -> TokenStream {
    let path = &pattern.path;
    let mut lhs = quote! {};
    let mut rhs = quote! {};
    // We must break out the inner impl of the RHS
    // in order to allow field mapping where it is
    // required.
    let mut rhs_inner = quote! {};

    params.iter().filter(|p| p.is_param).enumerate().for_each(|(idx, p)| {
        let field_name = &p.field_name;
        let param_name = new_ambiguous_ident!(&p.name);
        let index_name = new_ambiguous_ident!("p{}", idx);
        // Simply apply the parameter field if no
        // special formatting is required.
        if !pattern.params.contains(p) && p.is_option {
            lhs.extend(quote! { #field_name: None, });
            return
        } else if !pattern.params.contains(p) {
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

        if let Some(rq) = &p.requires {
            rhs_inner.extend(quote! {
                if !#rq(#param_name) {
                    return Err(crate::uri::UriBuildError::Validation.into())
                }
            })
        }
    });

    let mut conditional = quote! {};
    if let Some(rq) = &pattern.requires {
        conditional.extend(quote! { if #rq(self) })
    }

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
    quote! { Self { #lhs } #conditional => Ok(#rhs), }
}

/// Performs a shallow construction of a match arm
/// used for the `match` statement responsible for
/// the eventual building of a `URI` from the 
/// implementation of a `UriBuilder`.
fn build_match_arm_shallow(pattern: &MatchPatternAttrsParsed, params: &[ParamAttrsParsed]) -> TokenStream {
    let path = &pattern.path;
    let mut lhs = quote! {};
    let rhs = quote! { String::from(#path) };
    params.iter().filter(|p| p.is_param).for_each(|p| {
        let field_name = &p.field_name;
        lhs.extend(quote! { #field_name: None, })
    });
    lhs.extend(quote! { .. });
    quote! { Self { #lhs } => Ok(#rhs), }
}

/// Builds the `with_xx` methods associated with a
/// `UriBuilder` derivitive implmentation. Allows
/// for pre-construction declaration of
/// parameters.
fn build_methods(input: &DeriveInput, params: &[ParamAttrsParsed]) -> TokenStream {
    let mut with_methods = quote! {};
    params.iter().for_each(|param| {
        let method_name = new_ambiguous_ident!("with_{}", &param.name);
        let field_name  = &param.field_name;
        let kind        = &param.kind;

        if param.is_parent {
            with_methods.extend(quote! {
                /// Generated method to set the
                /// `#field_name` of `#ident`
                pub fn with_parent(mut self, value: #kind) -> Self {
                    self.#field_name = Some(value);
                    self
                }
                /// Creates a new instance of this
                /// type, presetting the parent
                /// to the passed value.
                pub fn from_parent(value: #kind) -> Self
                where
                    Self: Default,
                {
                    Self::default().with_parent(value)
                }
            })
        } else if param.is_option {
            with_methods.extend(quote! {
                /// Generated method to set the
                /// `#field_name` of `#ident`
                pub fn #method_name<V: Clone + Into<#kind>>(mut self, value: &V) -> Self {
                    self.#field_name = Some((*value).to_owned().into());
                    self
                }
            })
        } else {
            with_methods.extend(quote! {
                pub fn #method_name<V: Clone + Into<#kind>>(mut self, value: &V) -> Self {
                    self.#field_name = value.to_owned().into();
                    self
                }
            })
        }
    });

    let (ident, generics) = (&input.ident, &input.generics);
    let where_clause      = &generics.where_clause;
    quote! {
        impl #generics #ident #generics #where_clause {
            #with_methods
        }
    }
}

/// Filters out attributes that are not
/// `match_path` attribute modifiers.
fn filter_match_paths(attr: &&Attribute) -> bool {
    attr.meta.path().segments[0].ident == "match_path"
}

/// Filters out attributes that are not `param` or
/// `parent` attribute modifiers
fn filter_params(attr: &&Attribute) -> bool {
    ["param", "parent"]
        .contains(&attr.meta.path().segments[0].ident.to_string().as_str())
}

/// Determines if the attribute is a `parent`
/// attribute modifier.
fn is_parent(attr: &Attribute) -> bool {
    attr.meta.path().segments[0].ident == "parent"
}

/// Determines if the type is an `Option`
/// declaration.
fn is_optional_type(kind: &Type) -> bool {
    match &kind {
        Type::Path(tp) => tp.path.segments[0].ident == "Option",
        _ => panic!("expected a type path")
    }
}

/// Extrudes the nested type declaration from an
/// option declaration.
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

/// Parse the param declared fields on a
/// `UriBuilder` derived implemenation.
fn parse_params(fields: &Fields) -> Vec<ParamAttrsParsed> {
    match fields {
        Fields::Named(f) => {
            f.named.to_owned()
        },
        Fields::Unit => Punctuated::new(),
        Fields::Unnamed(f) => {
            f.unnamed.to_owned()
        }
    }
    .iter()
    .enumerate()
    .map(|(idx, f)| {
        let ident = f
        .ident
        .clone()
        .unwrap_or(new_ambiguous_ident!("p{}", idx));
    let is_param = !f
        .attrs
        .iter()
        .filter(filter_params)
        .collect::<Vec<_>>()
        .is_empty();
    let attrs = f
            .attrs
            .iter()
            .find(filter_params)
            .and_then(|a| {
                let mut parsed = ParamAttrs::from_attribute(a)
                    .unwrap();
                parsed.is_parent = is_parent(a).into();
                parsed.into()
            });
        let attrs = attrs.unwrap_or_default();
        let kind  = f.ty.clone();

        let is_parent = attrs.is_parent.unwrap_or_default();
        let is_option = is_optional_type(&kind);
        let name = attrs
            .name
            .as_ref().unwrap_or(&ident.to_string())
            .to_owned();
        let kind = if is_option {
            parse_optional_type(&kind)
        } else {
            &kind
        }.to_owned();
        let map_from = attrs
            .map_from
            .as_ref()
            .map(|mf| parse_str::<Expr>(mf).expect("must be a parsable expression"));
        let requires = attrs
            .requires
            .as_ref()
            .map(|rq| parse_str::<Expr>(rq).expect("must be a parsable expression"));

        ParamAttrsParsed {
            field_name: ident,
            name,
            kind,
            map_from,
            requires,
            is_option,
            is_parent,
            is_param,
        }
    })
    .collect()
}


/// Parse the path patterns declared on a
/// `UriBuilder` derived implementation.
fn parse_paths(attrs: &Attributes, params: &[ParamAttrsParsed]) -> Vec<MatchPatternAttrsParsed> {
    attrs
        .iter()
        .filter(filter_match_paths)
        .map(MatchPatternAttrs::from_attribute)
        .map(|a| {
            let a = a.unwrap();
            let requires = a
                .requires
                .map(|rq| parse_str::<Expr>(&rq).expect("must be a parsable expression"));
            let mut parsed = MatchPatternAttrsParsed{
                path: a.path,
                params: vec![],
                requires
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

/// Validate all params have been declared
/// inline with path patterns.
fn validate_paths_by_params(paths: &[MatchPatternAttrsParsed], params: &[ParamAttrsParsed]) {
    let mut param_map = HashSet::new();
    for match_path in paths.iter() {
        match_path.path.split('/').for_each(|p| {
            if p.contains(['{', '}']) {
                param_map.insert(p.replace(['{', '}'], ""));
            }
        })
    }
    if !param_map.iter().all(|n| params.iter().any(|p| p.name == *n)) {
        let fields = param_map
            .iter()
            .filter(|n| !params.iter().any(|p| p.name == **n))
            .map(|p| p.to_owned())
            .collect::<Vec<_>>()
            .join(", ");
        panic!("missing parameter(s) declared in path patterns: ({fields})")
    }
}
