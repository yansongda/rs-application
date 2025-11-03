pub(crate) mod logger;

use crate::logger::generate_param_logs;
use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

#[proc_macro_attribute]
pub fn logger_function(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let sig = &input.sig;
    let block = &input.block;
    let vis = &input.vis;
    let attrs = &input.attrs;
    let name = &sig.ident;

    let param_logs = generate_param_logs(sig);

    let result = quote! {
        #(#attrs)*
        #vis #sig {
            let started_at = std::time::Instant::now();

            let params = {
                let mut params = Vec::new();
                #(#param_logs)*
                params.join(", ")
            };

            let result = async #block.await;
            let elapsed = started_at.elapsed().as_secs_f32();

            tracing::info!(elapsed, ?params, ?result, "函数 {} 已执行", stringify!(#name));

            result
        }
    };

    result.into()
}
