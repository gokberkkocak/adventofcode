use proc_macro::TokenStream;

use quote::quote;

use syn::{parse_macro_input, DeriveInput};

use proc_macro2::{Ident, Span};

#[proc_macro_derive(AOCYear)]
pub fn aoc_year_maker(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let struct_name = &ast.ident;

    let full = (1..=25u8).collect::<Vec<_>>();
    let available = return_available_days(&struct_name.to_string()).unwrap();
    let not_available = full
        .iter()
        .filter(|item| !available.contains(item))
        .collect::<Vec<_>>();
    let available_module_names = available
        .iter()
        .map(|item| format!("day{}", item))
        .map(|s| Ident::new(&s, Span::call_site()))
        .collect::<Vec<_>>();
    let not_available_module_names = not_available
        .iter()
        .map(|item| format!("day{}", item))
        .map(|s| Ident::new(&s, Span::call_site()))
        .collect::<Vec<_>>();

    let expanded = quote! {
      #(
        mod #available_module_names;
      )*
      impl AOCYear for #struct_name {
        fn new() -> Box<dyn AOCYear> {
          Box::new(Self {})
        }
        #(
          fn #available_module_names(&self) {
            #available_module_names::run();
          }
        )*
        #(fn #not_available_module_names(&self) {
            unimplemented!("day {}", #not_available);
        })*
      }
    };
    expanded.into()
}

fn return_available_days(struct_name: &str) -> anyhow::Result<Vec<u8>> {
    let module_path = std::env::current_dir()?
        .join("src")
        .join(struct_name.to_lowercase());

    let mut years = Vec::new();
    for f in std::fs::read_dir(module_path)? {
        let f = f?;
        let filename = f.file_name();
        if let Some(filename) = filename.to_str() {
            if filename.starts_with("day") {
                let day = filename
                    .trim_start_matches("day")
                    .trim_end_matches(".rs")
                    .parse::<u8>()?;
                  years.push(day);
            }
        }
    }
    Ok(years)
}
