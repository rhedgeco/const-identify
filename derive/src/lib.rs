use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, spanned::Spanned, DeriveInput, GenericParam, TypeParamBound};

#[proc_macro_derive(ConstIdentify)]
pub fn derive_const_id(input: TokenStream) -> TokenStream {
    // parse the input
    let DeriveInput {
        ident,
        mut generics,
        ..
    } = parse_macro_input!(input);

    // add type bounds to all the type parameters
    let mut type_generics = Vec::new();
    let mut const_generics = Vec::new();
    let const_bound: TypeParamBound = syn::parse_quote!(::const_identify::ConstIdentify);
    for param in generics.params.iter_mut() {
        match param {
            GenericParam::Lifetime(_) => {} // do nothing with lifetimes
            GenericParam::Type(type_param) => {
                let param_ident = &type_param.ident;
                type_generics.push(quote! { #param_ident::TYPE_INFO });
                type_param.bounds.push(const_bound.clone());
            }
            GenericParam::Const(const_param) => {
                let param_ident = &const_param.ident;
                let type_string = const_param.ty.to_token_stream().to_string();
                match type_string.as_str() {
                    "char" => const_generics
                        .push(quote! { ::const_identify::ConstGeneric::Char(#param_ident) }),
                    "bool" => const_generics
                        .push(quote! { ::const_identify::ConstGeneric::Bool(#param_ident) }),
                    "u8" => const_generics
                        .push(quote! { ::const_identify::ConstGeneric::U8(#param_ident) }),
                    "u16" => const_generics
                        .push(quote! { ::const_identify::ConstGeneric::U16(#param_ident) }),
                    "u32" => const_generics
                        .push(quote! { ::const_identify::ConstGeneric::U32(#param_ident) }),
                    "u64" => const_generics
                        .push(quote! { ::const_identify::ConstGeneric::U64(#param_ident) }),
                    "u128" => const_generics
                        .push(quote! { ::const_identify::ConstGeneric::U128(#param_ident) }),
                    "usize" => const_generics
                        .push(quote! { ::const_identify::ConstGeneric::USize(#param_ident) }),
                    "i8" => const_generics
                        .push(quote! { ::const_identify::ConstGeneric::I8(#param_ident) }),
                    "i16" => const_generics
                        .push(quote! { ::const_identify::ConstGeneric::I16(#param_ident) }),
                    "i32" => const_generics
                        .push(quote! { ::const_identify::ConstGeneric::I32(#param_ident) }),
                    "i64" => const_generics
                        .push(quote! { ::const_identify::ConstGeneric::I64(#param_ident) }),
                    "i128" => const_generics
                        .push(quote! { ::const_identify::ConstGeneric::I128(#param_ident) }),
                    "isize" => const_generics
                        .push(quote! { ::const_identify::ConstGeneric::ISize(#param_ident) }),
                    _ => {
                        return syn::Error::new(const_param.ty.span(), "unknown const param")
                            .into_compile_error()
                            .into()
                    }
                }
            }
        }
    }

    // gather generics for quote
    let (implgen, typegen, wheregen) = generics.split_for_impl();

    // create output
    let output = quote! {
        unsafe impl #implgen ::const_identify::ConstIdentify for #ident #typegen #wheregen
        {
            const TYPE_INFO: &::const_identify::TypeInfo<'static> = &::const_identify::TypeInfo::new(
                file!(),
                line!(),
                column!(),
                stringify!(#ident),
                &[#(#type_generics,)*],
                &[#(#const_generics,)*],
            );
        }
    };

    // convert output and return
    output.into()
}
