use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{ToTokens, quote, format_ident};
use syn;
use syn::{Data, DataEnum, Fields, Index, Variant};

#[proc_macro_derive(Serializeable)]
pub fn derive_serializeable(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    match ast.data {
        Data::Struct(struct_data) => match struct_data.fields {
            fields => impl_for_struct(name, fields),
        },
        Data::Enum(enum_data) => impl_for_enum(name, enum_data),
        Data::Union(_) => {
            unimplemented!()
        }
    }.into()
}

fn impl_for_struct(name: &Ident, fields: Fields) -> proc_macro2::TokenStream {
    let identifier = fields
        .iter()
        .enumerate()
        .map(|(index, field)| match &field.ident {
            None => Index::from(index).to_token_stream(),
            Some(name) => {
                quote! {#name}
            }
        })
        .collect::<Vec<_>>();

    let mut async_impl: Option<proc_macro2::TokenStream> = None;
    #[cfg(feature = "async")]
    {
        let async_deserialize = async_deserialize_struct_per_field(fields, &identifier);
        async_impl = Some(quote! {
            fn async_deserialize<R: ::tokio::io::AsyncRead + Unpin>(reader: &mut R) -> impl Future<Output=Self> {
                async {
                    Self {
                        #(#async_deserialize)*
                    }
                }
            }
        });
    }
    quote! {
        impl Serializeable for #name {
            fn serialize_into<E: Extend<u8>>(&self, data: &mut E) {
                #(self.#identifier.serialize_into(data); )*
            }
            fn deserialize<R: Read>(reader: &mut R) -> Result<Self, Error> {
            Ok(
                Self{
                        #(#identifier: Serializeable::deserialize(reader)?,)*
                    }
                )
            }
            #async_impl
        }
    }
}

#[cfg(feature = "async")]
fn async_deserialize_struct_per_field(fields: Fields, identifiers: &Vec<proc_macro2::TokenStream>) -> Vec<proc_macro2::TokenStream> {
    fields.iter().zip(identifiers.iter()).map(|(field, identifier)| {
        let ty = &field.ty.to_token_stream();
        quote!{
            #identifier: <#ty>::async_deserialize(reader).await,
        }
    }).collect::<Vec<_>>()
}


fn wrap_concat_fields(fields: &Fields, wrap: fn(&proc_macro2::TokenStream) -> proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let names = if let Fields::Named(named_fields) = fields {
        named_fields.named.iter().map(|field| {
            let ident = &field.ident;
            quote!{#ident}
        }).collect::<Vec<_>>()
    }
    else{
        (0..fields.len())
            .map(
                |index| format_ident!("field{}", index).into_token_stream()
            ).collect::<Vec<_>>()
    };
    names.iter().map(
        |ident| wrap(ident)
    ).fold(quote!{}, |a, b| quote! {#a #b})
}
fn impl_for_enum(name: &Ident, enum_data: DataEnum) -> proc_macro2::TokenStream {
    let per_variant_serialize = enum_data.variants.iter().enumerate().map(|(variant_index, variant)| {
        serialize_enum_variant(variant, variant_index as u8)
    }).collect::<Vec<_>>();

    let per_variant_deserialize = enum_data.variants.iter().enumerate().map(|(variant_index, variant)| {
        deserialize_enum_variant(variant, variant_index as u8)
    }).collect::<Vec<_>>();


    let mut async_impl: Option<proc_macro2::TokenStream> = None;

    #[cfg(feature = "async")]
    {
        let per_variant_deserialize_async = enum_data.variants.iter().enumerate().map(|(variant_index, variant)| {
            deserialize_async_enum_variant(variant, variant_index as u8)
        }).collect::<Vec<_>>();
        async_impl = Some(quote! {
            fn async_deserialize<R: ::tokio::io::AsyncRead + Unpin>(reader: &mut R) -> impl Future<Output=Self> {
                async {
                    match u8::async_deserialize(reader).await {
                        #(#per_variant_deserialize_async)*
                        _ => panic!("Deserialization of enum failed: Invalid Discriminant")
                    }
                }
            }
        })
    }

    quote!{
        impl Serializeable for #name {
            fn serialize_into<E: Extend<u8>>(&self, data: &mut E) {
                match self {
                    #(#per_variant_serialize)*
                }
            }
            fn deserialize<R: ::std::io::Read>(reader: &mut R) -> Result<Self, ::std::io::Error> {
                Ok(
                    match u8::deserialize(reader)? {
                        #(#per_variant_deserialize)*
                        _ => panic!("Deserialization of enum failed: Invalid Discriminant")
                    }
                )
            }

            #async_impl

        }
    }
}

fn serialize_enum_variant(variant: &Variant, id: u8) -> proc_macro2::TokenStream {
    let variant_name = &variant.ident;
    let fields_comma_sep = wrap_concat_fields(
        &variant.fields,
        |ident| quote!{#ident, }
    );
    let serialize_fields = wrap_concat_fields(
        &variant.fields,
        |ident| quote!{#ident.serialize_into(data);}
    );
    match &variant.fields {
        Fields::Named(_) => {
            quote!{
                Self::#variant_name{#fields_comma_sep} => {
                    #id.serialize_into(data);
                    #serialize_fields
                },
            }
        }
        Fields::Unnamed(_) => {
            quote!{
                Self::#variant_name(#fields_comma_sep) => {
                    #id.serialize_into(data);
                    #serialize_fields
                },
            }
        }
        Fields::Unit => {
            quote!{
                Self::#variant_name => {
                    #id.serialize_into(data);
                },
            }
        }
    }.into()
}

fn deserialize_enum_variant(variant: &Variant, id: u8) -> proc_macro2::TokenStream {
    let variant_name = &variant.ident;
    let fields = &variant.fields;
    match fields {
        Fields::Named(_) => {
            let deserialize_fields = wrap_concat_fields(
                fields,
                |name| quote!{#name: Serializeable::deserialize(reader)?,}
            );
            quote!{
                #id => Self::#variant_name{#deserialize_fields},
            }
        }
        Fields::Unnamed(_) => {
            let deserialize_fields = wrap_concat_fields(
                fields,
                |_| quote!{Serializeable::deserialize(reader)?, }
            );
            quote!{
                #id => Self::#variant_name(#deserialize_fields),
            }
        }
        Fields::Unit => {
            quote!{
                #id => Self::#variant_name,
            }
        }
    }.into()
}

#[cfg(feature = "async")]
fn deserialize_async_enum_variant(variant: &Variant, id: u8) -> proc_macro2::TokenStream {
    let identifiers = variant
        .fields
        .iter()
        .enumerate()
        .map(|(index, field)| match &field.ident {
            None => quote!{},
            Some(name) => {
                quote! {#name: }
            }
        })
        .collect::<Vec<_>>();


    let per_field_deserialize = variant.fields.iter().zip(identifiers.iter()).map(|(field, identifier)| {
        let ty = &field.ty.to_token_stream();
        quote!{
            #identifier <#ty>::async_deserialize(reader).await,
        }
    }).collect::<Vec<_>>();

    let variant_name = &variant.ident;
    let fields = &variant.fields;
    match fields {
        Fields::Named(_) => {
            quote!{
                #id => Self::#variant_name
                {
                    #(#per_field_deserialize)*
                },
            }
        }
        Fields::Unnamed(_) => {
            quote!{
                #id => Self::#variant_name
                (
                    #(#per_field_deserialize)*
                ),
            }
        }
        Fields::Unit => {
            quote!{
                #id => Self::#variant_name,
            }
        }
    }.into()
}