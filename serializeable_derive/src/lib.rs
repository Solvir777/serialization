use proc_macro::TokenStream;
use std::env::var;
use proc_macro2::{Ident};
use quote::{format_ident, quote, ToTokens};
use syn;
use syn::{Data, DataEnum, DataStruct, Field, Fields, Index, Type, Variant};

#[proc_macro_derive(Serializeable)]
pub fn derive_serializeable(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    let ge = match ast.data {
        Data::Struct(struct_data) => match struct_data.fields {
            Fields::Named(_) => impl_for_struct(name, struct_data),
            Fields::Unnamed(_) => {impl_for_tuple_struct(name, struct_data)},
            Fields::Unit => {impl_for_unit_struct(name)}
        },
        Data::Enum(enum_data) => impl_for_enum(name, enum_data),
        Data::Union(_) => {unimplemented!()}
    };
    ge.into()
}

fn impl_for_struct(name: &Ident, struct_data: DataStruct) -> TokenStream {
    let idents = struct_data.fields.iter().map(|field| &field.ident).collect::<Vec<_>>();
    quote! {
        impl Serializeable for #name {
            fn serialize_into<E: Extend<u8>>(&self, data: &mut E) {
                #(
                    self.#idents.serialize_into(data);
                ) *
            }
            fn deserialize<R: Read>(reader: &mut R) -> Self {
                Self{
                    #(
                        #idents: Serializeable::deserialize(reader),
                    ) *
                }
            }
        }
    }.into()
}

fn impl_for_tuple_struct(name: &Ident, struct_data: DataStruct) -> TokenStream {
    let len = struct_data.fields.len();
    let idents = (0..len).map(Index::from).map(|x| quote!{#x}).collect::<Vec<_>>();
    let tuple_contents = (0..len).map(|_|quote!(Serializeable::deserialize(reader),)).collect::<Vec<_>>();
    quote! {
        impl Serializeable for #name {
            fn serialize_into<E: Extend<u8>>(&self, data: &mut E) {
                #(
                    self.#idents.serialize_into(data);
                ) *
            }
            fn deserialize<R: Read>(reader: &mut R) -> Self {
                Self(
                    #(
                        #tuple_contents
                    ) *
                )
            }
        }
    }.into()
}

fn impl_for_unit_struct(name: &Ident) -> TokenStream {
    quote!{
        impl Serializeable for #name {
            fn serialize_into<E: Extend<u8>>(&self, data: &mut E) {}
            fn deserialize<R: Read>(reader: &mut R) -> Self {}
        }
    }.into()
}


fn impl_for_enum(name: &Ident, enum_data: DataEnum) -> TokenStream {
    let per_variant_serialize = enum_data.variants.iter().enumerate().map(|(variant_index, variant)| {
        serialize_enum_variant(variant, variant_index as u8)
    }).collect::<Vec<_>>();

    let per_variant_deserialize = enum_data.variants.iter().enumerate().map(|(variant_index, variant)| {
        deserialize_enum_variant(variant, variant_index as u8)
    }).collect::<Vec<_>>();


    quote!{
        impl Serializeable for #name {
            fn serialize_into<E: Extend<u8>>(&self, data: &mut E) {
                match self {
                    #(#per_variant_serialize)*
                }
            }
            fn deserialize<R: Read>(reader: &mut R) -> Self {
                let discr = u8::deserialize(reader);
                match discr {
                    #(#per_variant_deserialize)*
                    _ => panic!("Deserialization of enum failed: Invalid Discriminant")
                }
            }
        }
    }.into()
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
                |name| quote!{#name: Serializeable::deserialize(reader),}
            );
            quote!{
                #id => Self::#variant_name{#deserialize_fields},
            }
        }
        Fields::Unnamed(_) => {
            let deserialize_fields = wrap_concat_fields(
                fields,
                |_| quote!{Serializeable::deserialize(reader), }
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