//! Derive `CloneFrom`, which generates a `Clone` implementation with `clone` and `clone_from`.

use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, parse_quote, Data, DeriveInput, Fields, GenericParam, Generics, Index, Type,
};

#[proc_macro_derive(CloneFrom)]
pub fn derive_clone_from_fn(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    derive_clone_from_inner(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn derive_clone_from_inner(input: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let name = input.ident;

    // Add `Clone` bound to generic arguments
    let mut generics = input.generics;
    add_trait_bounds(&mut generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Generate the inner part of `clone` and `clone_from`
    let (clone, clone_from) = generate_clone(&input.data)?;

    let expanded = quote! {
        #[automatically_derived]
        impl #impl_generics Clone for #name #ty_generics #where_clause {
            #[inline]
            fn clone(&self) -> Self {
                #[allow(clippy::clone_on_copy)]
                #clone
            }
            #[inline]
            fn clone_from(&mut self, other: &Self) {
                #[allow(clippy::clone_on_copy)]
                #clone_from
            }
        }
    };
    Ok(expanded.into())
}

/// Add the `Clone` trait bound to generic parameters.
///
/// Unconditionally adds a `Clone` bound to all generic parameters.
fn add_trait_bounds(generics: &mut Generics) {
    for param in &mut generics.params {
        if let GenericParam::Type(typ) = param {
            typ.bounds.push(parse_quote!(Clone));
        }
    }
}

/// Generate the `clone` and `clone_from` implementation.
///
/// Regular fields are cloned (using `clone` or `clone_from`), references are assigned.
fn generate_clone(data: &Data) -> syn::Result<(TokenStream, TokenStream)> {
    match data {
        Data::Struct(data) => match &data.fields {
            // Structs with named fields
            Fields::Named(fields) => {
                let clone = fields.named.iter().map(|field| {
                    let name = &field.ident;
                    match field.ty {
                        Type::Reference(_) => {
                            quote_spanned!(field.span() => #name: self.#name)
                        }
                        _ => {
                            quote_spanned!(field.span() => #name: self.#name.clone())
                        }
                    }
                });
                let clone = quote! {
                    Self {
                        #(#clone),*
                    }
                };

                let clone_from = fields.named.iter().map(|field| {
                    let name = &field.ident;
                    match field.ty {
                        Type::Reference(_) => {
                            quote_spanned!(field.span() => self.#name = other.#name)
                        }
                        _ => {
                            quote_spanned!(field.span() => self.#name.clone_from(&other.#name))
                        }
                    }
                });
                let clone_from = quote! {
                    #(#clone_from);*
                };

                Ok((clone, clone_from))
            }
            Fields::Unnamed(fields) => {
                // Tuple structs with numbered fields.
                let clone = fields.unnamed.iter().enumerate().map(|(i, field)| {
                    let index = Index::from(i);
                    match field.ty {
                        Type::Reference(_) => {
                            quote_spanned!(field.span() => #index: self.#index)
                        }
                        _ => {
                            quote_spanned!(field.span() => #index: self.#index.clone())
                        }
                    }
                });
                let clone = quote! {
                    Self {
                    #(#clone),*
                    }
                };

                let clone_from = fields.unnamed.iter().enumerate().map(|(i, field)| {
                    let index = Index::from(i);
                    match field.ty {
                        Type::Reference(_) => {
                            quote_spanned!(field.span() => self.#index = other.#index)
                        }
                        _ => {
                            quote_spanned!(field.span() => self.#index.clone_from(&other.#index))
                        }
                    }
                });
                let clone_from = quote! {
                    #(#clone_from);*
                };

                Ok((clone, clone_from))
            }
            Fields::Unit => Err(syn::Error::new(
                data.struct_token.span,
                "Cannot derive CloneFrom for unit.",
            )),
        },
        Data::Enum(data) => Err(syn::Error::new(
            data.enum_token.span,
            "Cannot derive CloneFrom for enum.",
        )),
        Data::Union(data) => Err(syn::Error::new(
            data.union_token.span,
            "Cannot CloneFrom for union.",
        )),
    }
}
