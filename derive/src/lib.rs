use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(ConstIdentify)]
pub fn derive_const_identify(input: TokenStream) -> TokenStream {
    // parse the input
    let DeriveInput {
        ident, generics, ..
    } = parse_macro_input!(input);

    // gather generics for quote
    let (implgen, typegen, wheregen) = generics.split_for_impl();

    // create output
    let output = quote! {
        unsafe impl #implgen ::const_identify::ConstIdentify for #ident #typegen #wheregen {
            const CONST_ID: ::const_identify::ConstId = ::const_identify::ConstId::generate(
                concat!(module_path!(), "::", stringify!(#ident))
            );
        }
    };

    // convert output and return
    output.into()
}
