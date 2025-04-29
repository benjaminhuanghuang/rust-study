use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn mytest_proc_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
  let attr = attr.to_string();
  let item = item.to_string();
  let result = format!("{} {}", attr, item);
  result.parse().unwrap()
}
