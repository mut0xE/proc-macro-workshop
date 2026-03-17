use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, Data::Struct, DeriveInput, Fields::Named, GenericArgument, Ident,
    PathArguments, Type,
};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input_ast = parse_macro_input!(input as DeriveInput);
    eprintln!("{:#?}", input_ast);
    // TokenStream::new()

    let name = &input_ast.ident; // Builder
    let bname = format!("{}Builder", name);
    let bident = Ident::new(&bname, name.span()); // CommandBuilder

    let fields = match &input_ast.data {
        Struct(fields_named) => match &fields_named.fields {
            Named(fields_named) => {
                // eprintln!("{:#?}", &fields_named.named);
                &fields_named.named
            }
            _ => {
                return syn::Error::new_spanned(&input_ast, "Only named fields supported")
                    .to_compile_error()
                    .into();
            }
        },
        _ => {
            return syn::Error::new_spanned(&input_ast, "Only structs supported")
                .to_compile_error()
                .into();
        }
    };

    let build_fields = fields.iter().map(|f| {
        let fname = &f.ident;
        let ftype = &f.ty;

        if option_inner_type(ftype).is_some() {
            quote! {
                #fname: self.#fname.clone()
            }
        } else {
            let err = format!("should set {}", fname.as_ref().unwrap());
            quote! {
                #fname: self.#fname.clone().ok_or(#err)?
            }
        }
    });
    let builder_init_fields = fields.iter().map(|f| {
        let fname = &f.ident;
        quote! {
            #fname: None
        }
    });

    let builder_fields = fields.iter().map(|f| {
        let fname = &f.ident;
        let ftype = &f.ty;
        if let Some(input_type) = option_inner_type(ftype) {
            quote! {
                #fname : Option<#input_type>
            }
        } else {
            quote! {
                #fname : Option<#ftype>
            }
        }
    });

    let setters = fields.iter().map(|f| {
        let fname = &f.ident;
        let ftype = &f.ty;
        let setter_type = if let Some(inner_type) = option_inner_type(ftype) {
            inner_type
        } else {
            ftype
        };
        quote! {
            pub fn #fname(&mut self, #fname : #setter_type) -> &mut Self{
                self.#fname = Some(#fname);
                self
            }
        }
    });
    let expand = quote! {
        pub struct #bident {
           #(#builder_fields,)*
        }

        impl #bident{
            #(#setters)*

            pub fn build(&mut self) -> Result<#name, Box<dyn std::error::Error>> {
                Ok(#name{
                   #(#build_fields,)*
                }
                )
            }
        }

        impl #name{
            pub fn builder() -> #bident {
                #bident {
                    #(#builder_init_fields,)*
                }
            }
        }
    };
    expand.into()
}

fn option_inner_type(ty: &Type) -> Option<&Type> {
    if let Type::Path(type_path) = ty {
        if type_path.qself.is_none()
            && type_path.path.segments.len() == 1
            && type_path.path.segments[0].ident == "Option"
        {
            if let PathArguments::AngleBracketed(args) = &type_path.path.segments[0].arguments {
                if let Some(GenericArgument::Type(inner_type)) = args.args.first() {
                    return Some(inner_type);
                }
            }
        }
    }
    None
}
