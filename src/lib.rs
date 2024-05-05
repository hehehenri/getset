use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{DeriveInput, Fields};

#[proc_macro_derive(Getters)]
pub fn getters(tokens: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(tokens).unwrap();
    generate_methods(&ast, gen_getters).into()
}

#[proc_macro_derive(Setters)]
pub fn setters(tokens: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(tokens).unwrap();
    generate_methods(&ast, gen_setters).into()
}

fn generate_methods<F>(ast: &DeriveInput, generator: F) -> TokenStream2
where
    F: Fn(&Fields) -> Vec<TokenStream2>,
{
    let (impl_generics, type_generics, where_clause) = &ast.generics.split_for_impl();
    let struct_name = &ast.ident;

    let fields = match &ast.data {
        syn::Data::Struct(r#struct) => &r#struct.fields,
        syn::Data::Enum(_) => panic!("setters can only be generated for structs"),
        syn::Data::Union(_) => panic!("setters can only be generated for structs"),
    };

    let generated_methods = generator(fields);

    quote! {
        impl #impl_generics #struct_name #type_generics #where_clause {
            #(#generated_methods)*
        }
    }
}

fn gen_getters(fields: &Fields) -> Vec<TokenStream2> {
    fields.iter().fold(Vec::new(), |mut acc, field| {
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

fn gen_setters(fields: &Fields) -> Vec<TokenStream2> {
    fields.iter().fold(Vec::new(), |mut acc, field| {
        let field_name = field.ident.clone().unwrap();
        let method_name = format_ident!("set_{field_name}");
        let r#type = &field.ty;

        acc.push(quote! {
            pub fn #method_name(&mut self, value: #r#type) {
                self.#field_name = value;
            }
        });

        acc
    })
}
