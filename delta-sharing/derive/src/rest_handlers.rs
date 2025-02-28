use convert_case::{Case, Casing};
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{LitStr, Type};

use super::parsing::{FieldDef, FieldSource, HandlerDef};

/// Implements the handler function for a given handler definition.
///
/// The functions can be used to register different routes on an axum Router.
pub fn to_handler(handler: &HandlerDef, handler_type: &Type) -> proc_macro2::TokenStream {
    let handler_name = generate_handler_name(&handler.request_type);
    let fn_name = Ident::new(&handler_name, Span::call_site());
    let handler_method = Ident::new(&handler_name, Span::call_site());
    let request_type = &handler.request_type;

    match &handler.response_type {
        Some(response_type) => quote! {
            pub(crate) async fn #fn_name<T: #handler_type>(
                ::axum::extract::State(handler): ::axum::extract::State<T>,
                ::axum::extract::Extension(recipient): ::axum::extract::Extension<Recipient>,
                request: #request_type,
            ) -> Result<::axum::extract::Json<#response_type>> {
                let ctx = RequestContext { recipient };
                Ok(::axum::extract::Json(handler.#handler_method(request, ctx).await?))
            }
        },
        None => quote! {
            pub(crate) async fn #fn_name<T: #handler_type>(
                ::axum::extract::State(handler): ::axum::extract::State<T>,
                ::axum::extract::Extension(recipient): ::axum::extract::Extension<Recipient>,
                request: #request_type,
            ) -> Result<()> {
                let ctx = RequestContext { recipient };
                handler.#handler_method(request, ctx).await?;
                Ok(())
            }
        },
    }
}

pub fn to_client(handler: &HandlerDef, route: &[String]) -> proc_macro2::TokenStream {
    let handler_name = generate_handler_name(&handler.request_type);
    let fn_name = Ident::new(&handler_name, Span::call_site());
    let handler_method = Ident::new(&handler_name, Span::call_site());
    let request_type = &handler.request_type;

    let type_name = get_type_name(request_type).unwrap();

    let path_names: Vec<_> = handler
        .fields
        .iter()
        .filter(|f| matches!(f.source, FieldSource::Path))
        .map(|f| &f.name)
        .collect();

    let mut query_params: Vec<_> = handler
        .fields
        .iter()
        .filter(|f| matches!(f.source, FieldSource::Query))
        .filter_map(|f| {
            let field_name = &f.name;
            match &f.ty {
                Type::Path(type_path) => {
                    if let Some(segment) = type_path.path.segments.first() {
                        if segment.ident == "Option" {
                            Some(quote! {
                                if let Some(val) = &req.#field_name {
                                    url.query_pairs_mut().append_pair(stringify!(#field_name), &format!("{}", val));
                                }
                            })
                        } else {
                            Some(quote! {
                                url.query_pairs_mut().append_pair(stringify!(#field_name), &format!("{}", &req.#field_name));
                            })
                        }
                    } else {
                        None
                    }
                }
                _ => None,
            }
        })
        .collect();

    let request_kind = get_request_type(&type_name);

    if matches!(request_kind, RequestType::List) {
        query_params.push(quote! {
            if let Some(val) = &req.max_results {
                url.query_pairs_mut().append_pair("max_results", &format!("{}", val));
            }
        });
        query_params.push(quote! {
            if let Some(val) = &req.page_token {
                url.query_pairs_mut().append_pair("page_token", &val);
            }
        });
    };

    let used_segments: Vec<_> = route
        .iter()
        .enumerate()
        .filter_map(|(idx, it)| {
            if idx < path_names.len() {
                Some(it.clone())
            } else {
                None
            }
        })
        .collect();
    let mut template = used_segments.join("/{}/");
    template.push_str("/{}");
    let template = LitStr::new(&template, Span::call_site());

    let url = if query_params.is_empty() {
        quote! {
            let url = base_url.join(&format!(#template, #(&req.#path_names,)*))?;
        }
    } else {
        quote! {
            let mut url = base_url.join(&format!(#template, #(&req.#path_names,)*))?;
        }
    };

    match (request_kind, &handler.response_type) {
        (RequestType::Create, Some(response_type)) => {
            quote! {
                pub async fn #fn_name(
                    client: &::cloud_client::CloudClient,
                    base_url: &::url::Url,
                    req: &#request_type,
                ) -> Result<#response_type> {
                    #url
                    #(#query_params)*
                    let result = client.post(url).json(req).send().await?.bytes().await?;
                    Ok(::serde_json::from_slice(&result)?)
                }
            }
        }
        (RequestType::Get | RequestType::List, Some(response_type)) => {
            quote! {
                pub async fn #fn_name(
                    client: &::cloud_client::CloudClient,
                    base_url: &::url::Url,
                    req: &#request_type,
                ) -> Result<#response_type> {
                    #url
                    #(#query_params)*
                    let result = client.get(url).send().await?.bytes().await?;
                    Ok(::serde_json::from_slice(&result)?)
                }
            }
        }
        (RequestType::Update, Some(response_type)) => {
            quote! {}
        }
        (RequestType::Delete, None) => {
            quote! {
                pub async fn #fn_name(
                    client: &::cloud_client::CloudClient,
                    base_url: &::url::Url,
                    req: &#request_type,
                ) -> Result<()> {
                    #url
                    #(#query_params)*
                    let result = client.delete(url).send().await?;
                    Ok(())
                }
            }
        }
        // error
        _ => panic!(),
    }
}

pub fn to_request_impl(handler: &HandlerDef) -> proc_macro2::TokenStream {
    let request_type = &handler.request_type;
    let type_name = get_type_name(request_type).unwrap();

    match get_request_type(&type_name) {
        RequestType::List => {
            // Generate paginated implementation
            generate_path_query_request_impl(request_type, &handler.fields, true)
        }
        RequestType::Create | RequestType::Update => {
            // Generate JSON body implementation
            quote! {
                impl<S: Send + Sync> ::axum::extract::FromRequest<S> for #request_type {
                    type Rejection = ::axum::response::Response;

                    async fn from_request(
                        req: ::axum::extract::Request<::axum::body::Body>,
                        _state: &S
                    ) -> Result<Self, Self::Rejection> {
                        let ::axum::extract::Json(request) = req
                            .extract()
                            .await
                            .map_err(::axum::response::IntoResponse::into_response)?;
                        Ok(request)
                    }
                }
            }
        }
        RequestType::Get | RequestType::Delete => {
            // Generate path parameter implementation
            generate_path_query_request_impl(request_type, &handler.fields, false)
        }
    }
}

pub(crate) fn to_action(handler: &HandlerDef) -> proc_macro2::TokenStream {
    let resource = &handler.resource;
    let request_type = &handler.request_type;
    let permission = &handler.permission;
    // HACK: we should probably annotate the query fields that should be extracted for
    // the resource identification, but for now we just hardcode the fields that are
    // known to be excluded.
    const KNOW_QUERY: [&str; 10] = [
        "max_results",
        "page_token",
        "force",
        "maxResults",
        "pageToken",
        "starting_timestamp",
        "startingTimestamp",
        "include_browse",
        "includeBrowses",
        "purpose",
    ];
    let field_names: Vec<_> = handler
        .fields
        .iter()
        .filter(|f| {
            matches!(f.source, FieldSource::Path)
                || !KNOW_QUERY.contains(&f.name.to_string().as_str())
        })
        .map(|f| &f.name)
        .collect();

    let resource = if field_names.is_empty() {
        quote! {
            ResourceIdent::#resource(ResourceRef::Undefined)
        }
    } else if field_names.len() == 1 {
        let field_name = &field_names[0];
        // If there is only one path parameter, it may be a fully qualified resource name
        quote! {
            ResourceIdent::#resource(ResourceRef::Name(ResourceName::from_naive_str_split(&self.#field_name)))
        }
    } else {
        quote! {
            ResourceIdent::#resource(ResourceRef::Name(ResourceName::new([#(&self.#field_names),*])))
        }
    };

    quote! {
        impl SecuredAction for #request_type {
            fn resource(&self) -> ResourceIdent {
                #resource
            }
            fn permission(&self) -> &'static Permission {
                &Permission::#permission
            }
        }
    }
}

/// Extracts the final segment of a Type’s path, e.g. SomeModule::FooBar => "FooBar".
fn get_type_name(ty: &Type) -> Option<String> {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return Some(segment.ident.to_string());
        }
    }
    None
}

/// Generate handler function name from request type
pub(crate) fn generate_handler_name(request_type: &Type) -> String {
    let request_name =
        get_type_name(request_type).unwrap_or_else(|| panic!("Invalid request type"));

    let base_name = request_name
        .strip_suffix("Request")
        .unwrap_or(&request_name);

    base_name.to_case(Case::Snake)
}

// Helper enum to categorize request types
enum RequestType {
    List,
    Create,
    Update,
    Get,
    Delete,
}

fn get_request_type(type_name: &str) -> RequestType {
    // Heuristics:
    //   “ListFooRequest” => List
    //   “CreateBarRequest” => Create
    //   “UpdateBazRequest” => Update
    //   “GetSomethingRequest” => Get
    //   “DeleteSomethingRequest” => Delete
    if type_name.starts_with("List") {
        RequestType::List
    } else if type_name.starts_with("Create") {
        RequestType::Create
    } else if type_name.starts_with("Update") {
        RequestType::Update
    } else if type_name.starts_with("Get") {
        RequestType::Get
    } else if type_name.starts_with("Delete") {
        RequestType::Delete
    } else {
        // Default to Get if pattern doesn't match
        RequestType::Get
    }
}

fn generate_path_query_request_impl(
    request_type: &Type,
    fields: &[FieldDef],
    paginated: bool,
) -> proc_macro2::TokenStream {
    let path_fields: Vec<_> = fields
        .iter()
        .filter(|f| matches!(f.source, FieldSource::Path))
        .collect();
    let query_fields: Vec<_> = fields
        .iter()
        .filter(|f| matches!(f.source, FieldSource::Query))
        .collect();

    let path_types: Vec<_> = path_fields.iter().map(|f| &f.ty).collect();
    let path_names: Vec<_> = path_fields.iter().map(|f| &f.name).collect();
    let mut query_names: Vec<_> = query_fields.iter().map(|f| &f.name).collect();
    let mut query_types: Vec<_> = query_fields.iter().map(|f| &f.ty).collect();

    // Add pagination fields
    let max_results_ident = Ident::new("max_results", Span::call_site());
    let page_token_ident = Ident::new("page_token", Span::call_site());
    let max_results_type = Type::Verbatim(quote! { Option<i32> });
    let page_token_type = Type::Verbatim(quote! { Option<String> });
    if paginated {
        query_names.push(&max_results_ident);
        query_names.push(&page_token_ident);
        query_types.push(&max_results_type);
        query_types.push(&page_token_type);
    }

    let path_ext = (!path_names.is_empty())
        .then(|| {
            quote! {
                use ::axum::extract::Path;
                let Path((#(#path_names),*)) = parts.extract::<Path<(#(#path_types),*)>>().await?;
            }
        })
        .unwrap_or_default();

    let query_ext = (!query_names.is_empty()).then(|| quote! {
        use ::axum::extract::Query;
        #[derive(::serde::Deserialize)]
        struct QueryParams {
            #(
                #query_names: #query_types,
            )*
        }
        let Query(QueryParams { #(#query_names,)* }) = parts.extract::<Query<QueryParams>>().await?;
    }).unwrap_or_default();

    quote! {
        impl<S: Send + Sync> ::axum::extract::FromRequestParts<S> for #request_type {
            type Rejection = Error;

            async fn from_request_parts(
                parts: &mut ::axum::http::request::Parts,
                _state: &S
            ) -> Result<Self, Self::Rejection> {
                #path_ext
                #query_ext
                Ok(#request_type {
                    #(#path_names,)*
                    #(#query_names,)*
                })
            }
        }
    }
}
