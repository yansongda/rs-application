use quote::quote;
use syn::Signature;

pub(crate) fn generate_param_logs(sig: &Signature) -> Vec<proc_macro2::TokenStream> {
    sig.inputs
        .iter()
        .enumerate()
        .map(|(i, arg)| {
            match arg {
                syn::FnArg::Receiver(_) => {
                    quote! {
                        params.push("self: <self>".to_string());
                    }
                }
                syn::FnArg::Typed(pat_type) => {
                    if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                        let ident = &pat_ident.ident;
                        let ident_str = ident.to_string();

                        quote! {
                            params.push(application_kernel::logger::truncate_for_log(&format!("{}: {:?}", #ident_str, #ident)));
                        }
                    } else {
                        // 跳过匿名和复杂模式参数
                        quote! {
                            params.push(format!("arg_{}: <complex pattern>", #i));
                        }
                    }
                }
            }
        })
        .collect()
}
