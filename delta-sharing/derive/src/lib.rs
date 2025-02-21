use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use quote::quote_spanned;
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, Data, DataStruct, DeriveInput, Error, Fields, Meta, PathArguments, Type,
};

/// Parses a dot-delimited column name into an array of field names. See
/// `delta_kernel::expressions::column_name::column_name` macro for details.
#[proc_macro]
pub fn parse_column_name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let is_valid = |c: char| c.is_ascii_alphanumeric() || c == '_' || c == '.';
    let err = match syn::parse(input) {
        Ok(syn::Lit::Str(name)) => match name.value().chars().find(|c| !is_valid(*c)) {
            Some(bad_char) => Error::new(name.span(), format!("Invalid character: {bad_char:?}")),
            _ => {
                let path = name.value();
                let path = path.split('.').map(proc_macro2::Literal::string);
                return quote_spanned! { name.span() => [#(#path),*] }.into();
            }
        },
        Ok(lit) => Error::new(lit.span(), "Expected a string literal"),
        Err(err) => err,
    };
    err.into_compile_error().into()
}
