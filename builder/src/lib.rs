use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input_ast = parse_macro_input!(input as DeriveInput);
    // eprintln!("{:#?}", input_ast);
    // TokenStream::new()

    let name = input_ast.ident; // Builder
    let bname = format!("{}Builder", name);
    let bident = Ident::new(&bname, name.span()); // CommandBuilder
    let expand = quote! {
        pub struct #bident {
              executable: Option<String>,
              args: Option<Vec<String>>,
              env: Option<Vec<String>>,
              current_dir: Option<String>,
        }

        impl #name{
            pub fn builder() -> #bident {
                #bident {
                    executable: None,
                    args: None,
                    env: None,
                    current_dir: None,
                }
            }
        }
    };
    expand.into()
}
