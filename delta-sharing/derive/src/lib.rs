use proc_macro2::{Ident, Span};
use quote::{quote, quote_spanned};
use syn::{bracketed, parse_macro_input, Error, Type};

use conversions::{from_object, resource_impl, to_object, to_resource, ObjectDefs};
use parsing::HandlerParams;
use rest_handlers::{
    generate_handler_name, get_type_name, to_action, to_client, to_handler, to_request_impl,
};

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
///
/// Names extracted from path parameters must be specified in hierarchical order, e.g.:
/// share -> schema -> table
/// catalog -> schema -> table
#[proc_macro]
pub fn rest_handlers(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as HandlerParams);
    let handler_type = input.handler_type;

    let actions = input.handlers.iter().map(to_action);

    // Generate handler functions
    let handlers = input
        .handlers
        .iter()
        .map(|handler| to_handler(handler, &handler_type));

    // Generate FromRequest/FromRequestParts implementations
    let request_impls = input.handlers.iter().map(to_request_impl);

    let client_impls = input.handlers.iter().map(|h| to_client(h, &input.segments));
    let handler_name = get_type_name(&handler_type).unwrap();
    let client_name = format!("{}Client", handler_name.strip_suffix("Handler").unwrap());
    let client_name = Ident::new(&client_name, Span::call_site());

    let client = quote! {
        pub struct #client_name {
            client: ::cloud_client::CloudClient,
            base_url: ::url::Url,
        }

        impl #client_name {
            pub fn new(client: ::cloud_client::CloudClient, base_url: ::url::Url) -> Self {
                Self { client, base_url }
            }

            #(#client_impls)*
        }
    };

    let mod_name = generate_handler_name(&handler_type);
    let mod_ident = Ident::new(&mod_name, Span::call_site());

    let expanded = quote! {
        #(#actions)*

        #client

        #[cfg(feature = "axum")]
        pub use #mod_ident::*;

        #[cfg(feature = "axum")]
        pub(crate) mod #mod_ident {
            use ::axum::{RequestExt, RequestPartsExt};

            use super::*;

            #(#handlers)*
            #(#request_impls)*
        }
    };

    proc_macro::TokenStream::from(expanded)
}

#[proc_macro]
pub fn object_conversions(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as ObjectDefs);

    let to_object_impls = input.defs.iter().map(to_object);
    let from_object_impls = input.defs.iter().map(from_object);

    // Generate resource impls
    let resource_impls = input.defs.iter().map(resource_impl);

    let to_resource_impls = input.defs.iter().map(to_resource);

    let expanded = quote! {
        #(#to_object_impls)*
        #(#from_object_impls)*
        #(#resource_impls)*
        #(#to_resource_impls)*
    };

    proc_macro::TokenStream::from(expanded)
}
