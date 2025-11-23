extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input,
    ItemImpl,
    ItemTrait,
    TraitItem,
    TraitItemFn,
    FnArg,
    Pat,
    ItemFn,
};

#[proc_macro_attribute]
pub fn log_trait_calls(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemTrait);

    // Get the name of the struct/type this impl is for
    let trait_name = input.ident.to_string();

    for item in &mut input.items {
        if let TraitItem::Fn(TraitItemFn { default: Some(block), sig, .. }) = item {
            let method_name = sig.ident.to_string();
            let original_block = block.clone();

            *block = syn::parse_quote!({
                #[cfg(feature = "log_calls")]
                println!("[{:?}] virtual: {}::{}", std::thread::current().id(), #trait_name, #method_name);
                #original_block
            });
        }
    }

    quote!(#input).into()
}

#[proc_macro_attribute]
pub fn log_impl_calls(_args: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemImpl);

    // Get the name of the struct/type this impl is for
    let struct_name = if let syn::Type::Path(type_path) = input.self_ty.as_ref() {
        type_path.path.segments.last().unwrap().ident.to_string()
    } else {
        String::from("UnknownType")
    };

    for item in &mut input.items {
        if let syn::ImplItem::Fn(method) = item {
            let method_name = method.sig.ident.to_string();
            let original_block = &method.block;

            // Collect parameter name/value pairs
            let mut param_logs = Vec::new();
            for input in &method.sig.inputs {
                if let FnArg::Typed(pat_type) = input {
                    if let Pat::Ident(ident) = &*pat_type.pat {
                        let name_str = ident.ident.to_string();
                        let name_ident = &ident.ident;
                        param_logs.push(quote! {
                            #[cfg(feature = "log_calls")]
                            println!("    {} = {:?}", #name_str, #name_ident);
                        });
                    }
                }
            }

            method.block = syn::parse_quote!({
                #[cfg(feature = "log_calls")]
                println!("[{:?}] real: {}::{}", std::thread::current().id(), #struct_name, #method_name);
                #(#param_logs)*
                #original_block
            });
        }
    }

    quote!(#input).into()
}

#[proc_macro_attribute]
pub fn log_fn_call(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let vis = &input.vis;
    let sig = &input.sig;
    let block = &input.block;
    let fn_name = sig.ident.to_string();

    let result = quote! {
        #vis #sig {
            #[cfg(feature = "log_calls")]
            println!("Calling function: {}", #fn_name);
            #block
        }
    };

    result.into()
}
