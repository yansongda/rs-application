use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn logger_sql(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let sig = &input.sig;
    let block = &input.block;
    let vis = &input.vis;
    let attrs = &input.attrs;

    let name = &sig.ident;

    let result = quote! {
        #(#attrs)*
        #vis #sig {
            let sql = sql;
            let started_at = std::time::Instant::now();
            let result = (|| async #block)().await;
            let elapsed = started_at.elapsed().as_secs_f32();

            tracing::info!(elapsed, sql, function = stringify!(#name));

            result
        }
    };

    result.into()
}
