use proc_macro2::Ident;
use quote::{quote, quote_spanned};
use syn::{bracketed, parse_macro_input, Error, Type};

use conversions::{from_object, resource_impl, to_object, to_resource, ObjectDefs};
use parsing::HandlerParams;
use rest_handlers::{to_handler, to_request_impl};

mod conversions;
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

/// Implements handler functions for REST endpoints.
///
/// Will generate implementations for axum's `FromRequest` and `FromRequestParts` traits for the
/// specified request types as well as the boilerplate handler functions.
///
/// The handlers are generic over the provided handler trait but assume a naming convention
/// for the route specific functions to invoke on a specific route.
///
/// # Example
///
/// ```ignore
/// rest_handlers! {
///     MyHandlerTrait, // The trait that the handlers expect
///     [
///         CreateCatalogRequest, CatalogInfo;
///     ]
/// }
/// ```
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

#[proc_macro]
pub fn object_conversions(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as ObjectDefs);

    let to_object_impls = input.defs.iter().map(|object_def| to_object(object_def));
    let from_object_impls = input.defs.iter().map(|object_def| from_object(object_def));

    // Generate resource impls
    let resource_impls = input
        .defs
        .iter()
        .map(|object_def| resource_impl(object_def));

    let to_resource_impls = input.defs.iter().map(|object_def| to_resource(object_def));

    let expanded = quote! {
        #(#to_object_impls)*
        #(#from_object_impls)*
        #(#resource_impls)*
        #(#to_resource_impls)*
    };

    proc_macro::TokenStream::from(expanded)
}
