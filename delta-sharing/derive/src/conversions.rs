use proc_macro2::Ident;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Expr, LitBool, Token, Type,
};

pub struct ObjectDefs {
    pub defs: Vec<ObjectDef>,
}

impl Parse for ObjectDefs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let defs = Punctuated::<ObjectDef, Token![;]>::parse_terminated(input)?;
        Ok(Self {
            defs: defs.into_iter().collect(),
        })
    }
}

/// Parsed object definition
///
/// This struct represents the parsed object definition
/// the user provided definition to be parsed:
/// ```ignore
/// TypeName, Label, [Path, Names], optional
/// ```
pub struct ObjectDef {
    pub ty: Type,
    pub label: Expr,
    pub name: Ident,
    pub path_names: Vec<Ident>,
    pub is_optional: bool,
}

impl Parse for ObjectDef {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ty = input.parse()?;
        input.parse::<Token![,]>()?;
        let label = input.parse()?;
        input.parse::<Token![,]>()?;
        let name = input.parse()?;
        input.parse::<Token![,]>()?;

        let content;
        syn::bracketed!(content in input);
        let field_defs = Punctuated::<Ident, Token![,]>::parse_terminated(&content)?;
        let path_names: Vec<_> = field_defs.into_iter().collect();

        let is_optional = if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            input.parse::<LitBool>()?.value
        } else {
            false
        };

        Ok(Self {
            name,
            ty,
            label,
            path_names,
            is_optional,
        })
    }
}

pub(crate) fn from_object(obj: &ObjectDef) -> proc_macro2::TokenStream {
    let target_ty = &obj.ty;
    let id_field_name = &obj.name;

    let id_assignment = if obj.is_optional {
        quote! {
            res.#id_field_name = Some(object.id.hyphenated().to_string());
        }
    } else {
        quote! {
            res.#id_field_name = object.id.hyphenated().to_string();
        }
    };

    quote! {
        impl TryFrom<Object> for #target_ty {
            type Error = Error;

            fn try_from(object: Object) -> Result<Self, Self::Error> {
                let props = object
                    .properties
                    .ok_or_else(|| Error::generic("expected properties"))?;
                let mut res: #target_ty = ::serde_json::from_value(props)?;
                #id_assignment
                Ok(res)
            }
        }
    }
}

pub(crate) fn to_object(obj: &ObjectDef) -> proc_macro2::TokenStream {
    let target_ty = &obj.ty;
    let id_field_name = &obj.name;

    let id_field = if obj.is_optional {
        quote! {
            let id = obj
                .#id_field_name
                .as_ref()
                .map(|id| ::uuid::Uuid::parse_str(id))
                .transpose()?
                .unwrap_or_else(|| ::uuid::Uuid::nil());
        }
    } else {
        quote! {
            let id = ::uuid::Uuid::parse_str(&obj.#id_field_name).unwrap_or_else(|_| ::uuid::Uuid::nil());
        }
    };

    quote! {
        impl TryFrom<#target_ty> for Object {
            type Error = Error;

            fn try_from(obj: #target_ty) -> Result<Self, Self::Error> {
                #id_field
                Ok(Object {
                    id,
                    name: obj.resource_name(),
                    label: obj.resource_label().clone(),
                    properties: Some(::serde_json::to_value(obj)?),
                    updated_at: None,
                    created_at: chrono::Utc::now(),
                })
            }
        }
    }
}

pub(crate) fn to_resource(obj: &ObjectDef) -> proc_macro2::TokenStream {
    let object_ty = &obj.ty;

    quote! {
        impl From<#object_ty> for Resource {
            fn from(obj: #object_ty) -> Self {
                Resource::#object_ty(obj)
            }
        }

        impl TryFrom<Resource> for #object_ty {
            type Error = Error;

            fn try_from(resource: Resource) -> Result<Self, Self::Error> {
                match resource {
                    Resource::#object_ty(value) => Ok(value),
                    _ => Err(Error::generic(concat!("Resource is not a ", stringify!(#object_ty)))),
                }
            }
        }
    }
}

pub(crate) fn resource_impl(obj: &ObjectDef) -> proc_macro2::TokenStream {
    let object_ty = &obj.ty;
    let label = &obj.label;
    let path_names = &obj.path_names;
    let id_field_name = &obj.name;

    let resource_ref = if obj.is_optional {
        quote! {
            self
                .#id_field_name
                .as_ref()
                .and_then(|id| ::uuid::Uuid::parse_str(id).ok())
                .map(|id| ResourceRef::Uuid(id))
                .unwrap_or_else(|| ResourceRef::Name(self.resource_name()))
        }
    } else {
        quote! {
            ::uuid::Uuid::parse_str(&self.#id_field_name)
                .ok()
                .map(|id| ResourceRef::Uuid(id))
                .unwrap_or_else(|| ResourceRef::Name(self.resource_name()))
        }
    };

    quote! {
        impl ResourceExt for #object_ty {
            fn resource_label(&self) -> &ObjectLabel {
                &#label
            }
            fn resource_name(&self) -> ResourceName {
                ResourceName::new([#(&self.#path_names),*])
            }
            fn resource_ref(&self) -> ResourceRef {
                #resource_ref
            }
        }
    }
}
