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
                        params.push(format!("self: <self>"));
                    }
                }
                syn::FnArg::Typed(pat_type) => {
                    let ident = match &*pat_type.pat {
                        syn::Pat::Ident(pat_ident) => &pat_ident.ident,
                        _ => {
                            let default_name = format!("arg_{}", i);
                            return quote! {
                                params.push(format!("{}: {:?}", #default_name, arg_{}));
                            };
                        }
                    };

                    let ident_str = ident.to_string();
                    quote! {
                        params.push(format!("{}: {:?}", #ident_str, #ident));
                    }
                }
            }
        })
        .collect()
}
