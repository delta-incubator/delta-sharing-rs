use super::*;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Comma,
    LitStr, Token,
};

/// Custom keywords for the macro
mod kw {
    ::syn::custom_keyword!(with);
    ::syn::custom_keyword!(path);
    ::syn::custom_keyword!(query);
}

/// Source of a field, e.g. path or query parameters
pub enum FieldSource {
    Path,
    Query,
}

/// Single “with” field definition, e.g.  foo: path as u32  or  bar: query as String
pub struct FieldDef {
    pub name: Ident,
    pub source: FieldSource,
    pub ty: Type,
}

impl Parse for FieldDef {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        input.parse::<Token![:]>()?;
        let source = if input.peek(kw::path) {
            input.parse::<kw::path>()?;
            FieldSource::Path
        } else if input.peek(kw::query) {
            input.parse::<kw::query>()?;
            FieldSource::Query
        } else {
            return Err(input.error("expected `path` or `query`"));
        };
        input.parse::<Token![as]>()?;
        let ty = input.parse()?;
        Ok(FieldDef { name, source, ty })
    }
}

/// One “handler definition” inside the macro, e.g.:
///   CreateFooRequest with [...], FooResponse, FooPermission;
///   or
///   DeleteFooRequest;
pub struct HandlerDef {
    pub request_type: Type,
    pub response_type: Option<Type>,
    pub fields: Vec<FieldDef>,
    pub permission: Type,
    pub resource: Type,
}

impl Parse for HandlerDef {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let request_type = input.parse()?;

        input.parse::<Comma>()?;
        let resource = input.parse()?;

        input.parse::<Comma>()?;
        let permission = input.parse()?;

        let mut response_type = None;
        if input.peek(Comma) {
            input.parse::<Comma>()?;
            response_type = Some(input.parse()?);
        }

        // Parse optional field definitions
        let mut fields = Vec::new();
        if input.peek(kw::with) {
            input.parse::<kw::with>()?;
            let content;
            syn::bracketed!(content in input);
            let field_defs = Punctuated::<FieldDef, Comma>::parse_terminated(&content)?;
            fields = field_defs.into_iter().collect();
        }

        Ok(HandlerDef {
            request_type,
            response_type,
            fields,
            permission,
            resource,
        })
    }
}

/// The top-level “(MyHandler, […])” portion of the macro.
pub struct HandlerParams {
    pub handler_type: Type,
    pub segments: Vec<String>,
    pub handlers: Vec<HandlerDef>,
}

impl Parse for HandlerParams {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let handler_type = input.parse()?;
        input.parse::<Token![,]>()?;

        let route: LitStr = input.parse()?;
        input.parse::<Token![,]>()?;
        let segments: Vec<_> = route.value().split('/').map(|v| v.to_string()).collect();

        let content;
        bracketed!(content in input);
        let handlers = Punctuated::<HandlerDef, Token![;]>::parse_terminated(&content)?
            .into_iter()
            .collect();

        Ok(HandlerParams {
            handler_type,
            segments,
            handlers,
        })
    }
}
