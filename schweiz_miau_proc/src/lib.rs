use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn мяу(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn grueti_mitenand(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro]
pub fn fondue(input: TokenStream) -> TokenStream {
    input
}

#[proc_macro]
pub fn schoggi(input: TokenStream) -> TokenStream {
    input
}

#[proc_macro]
pub fn kaese(input: TokenStream) -> TokenStream {
    input
}
