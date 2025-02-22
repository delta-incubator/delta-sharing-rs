use proc_macro2::Ident;
use quote::{quote, quote_spanned};
use syn::{bracketed, parse_macro_input, Error, Type};

use parsing::HandlerParams;
use rest_handlers::{to_handler, to_request_impl};

/// Parser for macro parameters
mod parsing;
mod rest_handlers;

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
                return quote_spanned! { name.span()=> [#(#path),*] }.into();
            }
        },
        Ok(lit) => Error::new(lit.span(), "Expected a string literal"),
        Err(err) => err,
    };
    err.into_compile_error().into()
}

#[proc_macro]
pub fn rest_handlers(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as HandlerParams);
    let handler_type = input.handler_type;

    // Generate handler functions
    let handlers = input
        .handlers
        .iter()
        .map(|handler| to_handler(handler, &handler_type));

    // Generate FromRequest/FromRequestParts implementations
    let request_impls = input.handlers.iter().map(to_request_impl);

    let expanded = quote! {
        #(#handlers)*
        #(#request_impls)*
    };

    proc_macro::TokenStream::from(expanded)
}
