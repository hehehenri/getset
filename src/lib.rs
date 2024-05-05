use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{DeriveInput, Meta};

#[proc_macro_derive(Getters, attributes(getset))]
pub fn getters(tokens: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(tokens).unwrap();
    generate_methods(&ast, gen_getters).into()
}

#[proc_macro_derive(Setters, attributes(getset))]
pub fn setters(tokens: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(tokens).unwrap();
    generate_methods(&ast, gen_setters).into()
}

fn generate_methods<F>(ast: &DeriveInput, generator: F) -> TokenStream2
where
    F: Fn(&Vec<Field>) -> Vec<TokenStream2>,
{
    let (impl_generics, type_generics, where_clause) = &ast.generics.split_for_impl();
    let struct_name = &ast.ident;

    let fields = match &ast.data {
        syn::Data::Struct(r#struct) => &r#struct.fields,
        syn::Data::Enum(_) => panic!("setters can only be generated for structs"),
        syn::Data::Union(_) => panic!("setters can only be generated for structs"),
    };

    let fields = parse_fields(fields);

    let generated_methods = generator(&fields);

    quote! {
        impl #impl_generics #struct_name #type_generics #where_clause {
            #(#generated_methods)*
        }
    }
}

struct Attributes {
    pub skip: bool,
    pub skip_getter: bool,
    pub skip_setter: bool,
}

impl Default for Attributes {
    fn default() -> Self {
        Self {
            skip: false,
            skip_getter: false,
            skip_setter: false,
        }
    }
}

fn parse_attributes(attrs: &Vec<syn::Attribute>) -> Attributes {
    use syn::{punctuated::Punctuated, Token};

    attrs
        .iter()
        .filter(|attr| attr.path.is_ident("getset"))
        .fold(Attributes::default(), |mut acc, attr| {
            attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                .expect("failed to parse getset attribute")
                .into_iter()
                .for_each(|meta| {
                    if meta.path().is_ident("skip") {
                        acc.skip = true;
                    }

                    if meta.path().is_ident("skip_getter") {
                        acc.skip_getter = true;
                    }

                    if meta.path().is_ident("skip_setter") {
                        acc.skip_setter = true;
                    }
                });

            acc
        })
}

struct Field {
    pub raw_field: syn::Field,
    pub attributes: Attributes,
}

fn parse_fields(fields: &syn::Fields) -> Vec<Field> {
    fields
        .iter()
        .map(|field| Field {
            attributes: parse_attributes(&field.attrs),
            raw_field: field.clone(),
        })
        .collect()
}

fn gen_getters(fields: &Vec<Field>) -> Vec<TokenStream2> {
    fields
        .iter()
        .filter_map(|field| {
            if field.attributes.skip || field.attributes.skip_getter {
                None
            } else {
                Some(&field.raw_field)
            }
        })
        .fold(Vec::new(), |mut acc, field| {
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

fn gen_setters(fields: &Vec<Field>) -> Vec<TokenStream2> {
    fields
        .iter()
        .filter_map(|field| {
            if field.attributes.skip || field.attributes.skip_setter {
                None
            } else {
                Some(&field.raw_field)
            }
        })
        .fold(Vec::new(), |mut acc, field| {
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
