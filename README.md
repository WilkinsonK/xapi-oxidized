# Oxidized XNAT REST
Interacts with a remote XNAT via REST exposing the **XAPI** as
bindings in Rust.

## Under the Hood
OXR uses a number of dependencies. See the below as reference to the
projects used in this one.

- [surf](https://github.com/http-rs/surf) HTTP backend client
- [netrc-rs](https://github.com/yjhmelody/netrc-rs) .netrc file parser
