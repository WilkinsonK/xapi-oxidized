# XAPI Oxidized Derive #
This crate defines the *magic* behind the URI build mappings found in
`oxinate_core` and the root crate, `oxinat`. Primarily this crate is
used in the core library to generate relevant methods that eventually
result in the URI building system in `oxinat`.

At this time, the derive macros do not support unit structs at the
top-level of structural definitions, but does support for regular
structs and enumerations.

```rust
use oxinat_derive::UriBuilder;

#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "some/uri")]
#[match_path(path = "some/uri/{some_parameter}")]
struct SomeUriBuilder {
    #[param]
    some_parameter: Option<String>
}
```

## Attribute Directives ##
As briefly described above, there are several attribute modifiers used
to more deeply define how, at build-time, a URI may be constructed.
It is important to know that match arms are constructed in order of
each `#[match_path]` declaration.

### Match Path ###
The first, and most important directive, will be the `#[match_path]`
macro. This tells `oxinat_derive` how to construct build the resulting
`build` method that will be defined at compile time. It comes with two
attributes, `path` and `requires` (optional).

- `path` is required. This allows `oxinat_derive` to know what and how
   pattern(s) should be constructed for the builder.
- `requires` is an optional attribute. It accepts a string of some
   boolean expression (e.g. ```"2+2 == 4"```). This modifies the match
   arm to only be valid on the condition. **NOTE**: At this scope, the
   attribute will match at the 'self' level of the builder.

### Parent and Param ###
Both `#[parent]` and `#[param]` behave effectively the same way when
constructing match arms for the eventual `build` method. However,
there are differences when constructing certain methods that allow for
pre-build customization.

All-in-all, it will look the same on the surface. Before URI
construction, methods will be implemented for these fields which allow
a user to set the inner values.

For example:

```rust
use oxinat_derive::UriBuilder;

#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/some/uri")]
#[match_path(path = "{parent}/some/uri/{some_parameter}")]
struct SomeUriBuilder {
    #[param]
    some_parameter: Option<String>
    #[parent]
    parent: Option<String>
}

// Under the hood, these methods will be implemented.
impl SomeUriBuilder {
    pub fn with_some_parameter(mut self, value: String) -> Self {
        ...
    }

    pub fn with_parent(mut self, value: String) -> Self {
        ...
    }
}
```

In addition to generating methods, declaring fields as either a parent
or a param allows for some customization with additional attributes.

- `requires` works similar to how it behaves at the `#[match_path]`
   level, but instead at the parameter level.
- `map_from` allows for specific formatting/manipulation of a value
   during build-time of the URI.

Both of these directives behave the same as
`#[match_path(requires = "..")]`. This means that it will accept a
stringed boolean expression, such as a closure, function or macro.
