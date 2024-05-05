use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::DeriveInput;

#[proc_macro_derive(Getters, attributes(get))]
pub fn getters(tokens: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(tokens).unwrap();
    getters::r#impl(&ast)
}

mod getters {
    use super::*;
    use syn::FieldsNamed;

    pub fn r#impl(ast: &DeriveInput) -> TokenStream {
        let r#struct = match &ast.data {
            syn::Data::Struct(r#struct) => r#struct,
            syn::Data::Enum(_) => panic!("Getters can only be generated for structs"),
            syn::Data::Union(_) => panic!("Getters can only be generated for structs"),
        };

        match &r#struct.fields {
            syn::Fields::Named(fields) => named_fields(ast, fields),
            syn::Fields::Unnamed(_fields) => todo!(),
            syn::Fields::Unit => todo!(),
        }
    }

    fn named_fields(ast: &DeriveInput, fields: &FieldsNamed) -> TokenStream {
        let struct_name = &ast.ident;

        let methods = fields.named.iter().fold(Vec::new(), |mut acc, field| {
            let field_name = field.ident.clone().unwrap();
            let method_name = format_ident!("get_{field_name}");
            let r#type = &field.ty;

            acc.push(quote! {
                pub fn #method_name(&self) -> &#r#type {
                    &self.#field_name
                }
            });

            acc
        });

        quote! {
            impl #struct_name {
                #(#methods)*
            }
        }
        .into()
    }
}
