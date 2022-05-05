use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, spanned::Spanned, ReturnType};
use quote::{quote, quote_spanned};


/// Gets an async function and wraps it in a `PinBox`
///
/// ```no_run
///
/// /// turns
/// #[derive_pinbox]
/// async fn hello_world() -> String {
///    "Hello, world!".to_string()
/// }
///
/// /// into
/// fn hello_world() -> Pin<Box<dyn Future<Output = String> + Send + 'static>>> {
///   Box::pin(async move {
///    "Hello, world!".to_string()
///  })
/// }
///
/// ```
#[proc_macro_attribute]
pub fn derive_pinbox(_attr: TokenStream, input: TokenStream) -> TokenStream {
    // parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as syn::Item);

    // get the function and confirm if it is async
    let mut func = match input {
        syn::Item::Fn(func) if func.sig.asyncness.is_some() => func,
        _ => panic!("derive_pinbox only works on async functions"),
    };

    func.sig.asyncness = None;

    // get the function body
    let func_block = func.block.clone();

    // get the function return type without the ->
    let return_type = match &func.sig.output {
        syn::ReturnType::Type(_, return_type) => quote!{ #return_type },
        syn::ReturnType::Default => quote!{ () },
    };

    let func_block = TokenStream::from(quote! {{
        Box::pin(async move {
            // do this for clearer errors
            let ret: #return_type = #func_block;
            ret
        })
    }});

    let return_type = TokenStream::from(quote! { -> std::pin::Pin<std::boxed::Box<dyn std::future::Future<Output = #return_type> + std::marker::Send + '_>>});

    func.sig.output = parse_macro_input!(return_type as ReturnType);

    func.block = Box::new(parse_macro_input!(func_block as syn::Block));

    // get the block of code to be wrapped
    let expanded = quote! {
        #func
    };

    TokenStream::from(expanded)
}