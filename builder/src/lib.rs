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
        impl #bident{
            pub fn executable(&mut self, executable: String) -> &mut Self {
                        self.executable = Some(executable);
                        self
            }
            pub fn args(&mut self, args: Vec<String>) -> &mut Self {
                        self.args = Some(args);
                        self
            }
            pub fn env(&mut self, env: Vec<String>) -> &mut Self {
                        self.env = Some(env);
                        self
            }
            pub fn current_dir(&mut self, current_dir: String) -> &mut Self {
                        self.current_dir = Some(current_dir);
                        self
            }

            pub fn build(&mut self) -> Result<#name, Box<dyn std::error::Error>> {
                Ok(
                    #name
                    {
                    executable: self.executable.clone().ok_or("should set executable")?,
                    env: self.env.clone().ok_or("should set env")?,
                    args: self.args.clone().ok_or("should set args")?,
                    current_dir: self.current_dir.clone().ok_or("should set current dir")?,
                }
                )
            }
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
