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
    MetaNameValue,
    PatType,
    ExprPath,
    ItemFn,
    Expr,
    Meta,
    Block
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
                println!("[Thread: {:?}] virtual: {}::{}", std::process::id(), #trait_name, #method_name);
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
                            println!("    {} = {:?}", #name_str, #name_ident);
                        });
                    }
                }
            }

            method.block = syn::parse_quote!({
                println!("[Thread: {:?}] real: {}::{}", std::process::id(), #struct_name, #method_name);
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
            println!("Calling function: {}", #fn_name);
            #block
        }
    };

    result.into()
}

#[proc_macro_attribute]
pub fn py_call(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let attr_meta = syn::parse_macro_input!(attr as Meta);

    // Parse attribute like #[py_call(py_impl = api)]
    let py_impl_expr = match attr_meta {
        Meta::NameValue(MetaNameValue { path, value, .. }) if path.is_ident("py_impl") => {
            match value {
                Expr::Path(ExprPath { path, .. }) => {
                    quote! { #path }
                }
                _ => {
                    return syn::Error::new_spanned(value, "Expected field name")
                        .to_compile_error()
                        .into();
                }
            }
        }
        _ => {
            return syn::Error::new_spanned(attr_meta, "Expected #[py_call(py_impl = field name)]")
                .to_compile_error()
                .into();
        }
    };

    let fn_name = &input_fn.sig.ident;
    let fn_name_str = fn_name.to_string();
    let inputs = &input_fn.sig.inputs;
    let output = &input_fn.sig.output;
    let vis = &input_fn.vis;
    let fallback_block: &Block = &input_fn.block;

    // Parse arguments: split receiver and others
    let mut self_arg = None;
    let mut arg_defs = vec![];
    let mut arg_names = vec![];

    for input in inputs {
        match input {
            FnArg::Receiver(_) => { // self
                self_arg = Some(quote! { #input });
            }
            FnArg::Typed(PatType { pat, .. }) => {
                match **pat {
                    Pat::Ident(ref ident) => {
                        arg_names.push(quote! { *#ident });
                        arg_defs.push(quote! { #input });
                    }
                    _ => panic!("Only identifier patterns supported"),
                }
            }
        }
    }

    let self_arg_tokens = self_arg.unwrap_or_else(|| quote!());
    let py_tuple = if arg_names.is_empty() {
        quote! { pyo3::types::PyTuple::empty(py) }
    } else {
        quote! { pyo3::types::PyTuple::new(py, [#(#arg_names),*])? }
    };

    let gen_code = quote! {
        #[log_fn_call]
        #vis fn #fn_name(#self_arg_tokens #(, #arg_defs)*) #output {
            use pyo3::prelude::*;
            Python::with_gil(|py| -> PyResult<_> {
                match self.#py_impl_expr.call_method(
                    py,
                    #fn_name_str,
                    #py_tuple,
                    None
                ) {
                    Ok(val) => Ok(val.extract(py).unwrap_or_else(|_| #fallback_block)),
                    Err(_) => Ok(#fallback_block),
                }
            }).unwrap_or_else(|_| #fallback_block)
        }
    };

    gen_code.into()
}
