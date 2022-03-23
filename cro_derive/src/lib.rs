use proc_macro2::TokenStream;
use quote::quote;
//use syn::parse::{Parse, ParseStream};
//use syn::punctuated::Punctuated;

//use syn::parse_quote;
//use syn::Type;
// use syn::braced;
use syn::spanned::Spanned;

use syn::{parse_macro_input, parse_quote, Ident, Type, TypePath};
//token, Field, Ident, Result, Token};
use syn::{ItemImpl, ItemStruct};

#[proc_macro_attribute]
pub fn cro_state(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut cro_state = parse_macro_input!(item as ItemStruct);

    let id = cro_state.ident.clone();

    let cro_state_id = Ident::new(
        &format!("CroState{}", cro_state.ident),
        cro_state.ident.span(),
    );

    cro_state.ident = cro_state_id.clone();

    let attr: TokenStream = attr.into();

    let ts = quote! {
        #attr
        #cro_state

        pub struct #id {
            pub state : Resource<#cro_state_id>
        }
    };

    proc_macro::TokenStream::from(ts)
}

#[proc_macro_attribute]
pub fn cro_impl(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut cro_impl = parse_macro_input!(item as ItemImpl);

    let type_id = &*cro_impl.self_ty;

    let new_type_id = Ident::new(
        &format!("CroState{}", quote!(#type_id)),
        cro_impl.self_ty.span(),
    );

    let ts: TypePath = parse_quote!(#new_type_id);

    cro_impl.self_ty = Box::new(Type::Path(ts));

    let attr: TokenStream = attr.into();

    let ts = quote! {
        #attr
        #cro_impl
    };

    proc_macro::TokenStream::from(ts)
}
