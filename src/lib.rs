use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::DeriveInput;

#[proc_macro_derive(Getters)]
pub fn getters(tokens: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(tokens).unwrap();
    let getters = gen_getters(&ast);

    generate_methods(&ast, getters).into()
}

fn generate_methods(ast: &DeriveInput, methods: Vec<TokenStream2>) -> TokenStream2 {
    let (impl_generics, type_generics, where_clause) = &ast.generics.split_for_impl();
    let struct_name = &ast.ident;

    quote! {
        impl #impl_generics #struct_name #type_generics #where_clause {
            #(#methods)*
        }
    }
}

fn gen_getters(ast: &DeriveInput) -> Vec<TokenStream2> {
    let r#struct = match &ast.data {
        syn::Data::Struct(r#struct) => r#struct,
        syn::Data::Enum(_) => panic!("getters can only be generated for structs"),
        syn::Data::Union(_) => panic!("getters can only be generated for structs"),
    };

    r#struct.fields.iter().fold(Vec::new(), |mut acc, field| {
        let field_name = field.ident.clone().unwrap();
        let method_name = format_ident!("get_{field_name}");
        let r#type = &field.ty;

        acc.push(quote! {
            pub fn #method_name(&self) -> &#r#type {
                &self.#field_name
            }
        });

        acc
    })
}
