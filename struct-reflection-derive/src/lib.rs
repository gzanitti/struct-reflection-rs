extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(StructReflection)]
pub fn struct_reflection(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = input.ident;
    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let generic_types: Vec<_> = generics.type_params().collect();

    let field_list_code = match input.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields) => fields
                .named
                .iter()
                .map(|field| generate_field_code(field, &generic_types))
                .collect(),
            Fields::Unnamed(fields) => fields
                .unnamed
                .iter()
                .enumerate()
                .map(|(i, field)| generate_unnamed_field_code(i, field, &generic_types))
                .collect(),
            Fields::Unit => vec![],
        },
        _ => panic!("StructReflection can only be used on structs."),
    };

    let expanded = quote! {
        impl #impl_generics StructReflectionHelper for #struct_name #ty_generics #where_clause {
            fn struct_reflection() -> Option<Vec<String>> {
                let mut fields = Vec::new();
                #(#field_list_code)*
                Some(fields)
            }
        }
    };

    TokenStream::from(expanded)
}

fn generate_field_code(
    field: &syn::Field,
    generic_types: &[&syn::TypeParam],
) -> proc_macro2::TokenStream {
    let field_name = field.ident.as_ref().unwrap().to_string();
    let field_type = &field.ty;

    if is_primitive_type(field_type) {
        return quote! {
            fields.push(#field_name.to_string());
        };
    }

    for generic_type in generic_types {
        if is_generic_parameter(field_type, generic_type) {
            return quote! {
                fields.push(#field_name.to_string());
            };
        }
    }

    if let Some(token_stream) = handle_tuple_type(field_type, &field_name, generic_types) {
        return token_stream;
    }

    if let Some(token_stream) = handle_array_type(field_type, &field_name, generic_types) {
        return token_stream;
    }

    quote! {
        if let Some(inner_fields) = <#field_type as StructReflectionHelper>::struct_reflection() {
            for inner_field in inner_fields {
                fields.push(format!("{}__{}",  #field_name, inner_field));
            }
        } else {
            fields.push(#field_name.to_string());
        }
    }
}

fn generate_unnamed_field_code(
    index: usize,
    field: &syn::Field,
    generic_types: &[&syn::TypeParam],
) -> proc_macro2::TokenStream {
    let field_type = &field.ty;
    let index_str = index.to_string();

    if is_primitive_type(field_type) {
        return quote! {
            fields.push(#index_str.to_string());
        };
    }

    for generic_type in generic_types {
        if is_generic_parameter(field_type, generic_type) {
            return quote! {
                fields.push(#index_str.to_string());
            };
        }
    }

    if let Some(token_stream) = handle_tuple_type(field_type, &index_str, generic_types) {
        return token_stream;
    }

    if let Some(token_stream) = handle_array_type(field_type, &index_str, generic_types) {
        return token_stream;
    }

    quote! {
        if let Some(inner_fields) = <#field_type as StructReflectionHelper>::struct_reflection() {
            for inner_field in inner_fields {
                fields.push(format!("{}__{}", #index, inner_field));
            }
        } else {
            fields.push(#index.to_string());
        }
    }
}

// Helper functions for type checking
fn is_primitive_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            let type_name = &segment.ident.to_string();
            return matches!(
                type_name.as_str(),
                "bool"
                    | "char"
                    | "u8"
                    | "u16"
                    | "u32"
                    | "u64"
                    | "u128"
                    | "usize"
                    | "i8"
                    | "i16"
                    | "i32"
                    | "i64"
                    | "i128"
                    | "isize"
                    | "f32"
                    | "f64"
                    | "str"
                    | "String"
            );
        }
    }
    false
}

fn is_generic_parameter(ty: &syn::Type, generic_type: &syn::TypeParam) -> bool {
    if let syn::Type::Path(type_path) = ty {
        return type_path.path.is_ident(&generic_type.ident);
    }
    false
}

fn is_tuple_type(ty: &syn::Type) -> bool {
    if let syn::Type::Tuple(_) = ty {
        return true;
    }
    false
}

fn handle_tuple_type(
    ty: &syn::Type,
    field_name: &str,
    generic_types: &[&syn::TypeParam],
) -> Option<proc_macro2::TokenStream> {
    if let syn::Type::Tuple(tuple_type) = ty {
        let mut element_handling = Vec::new();

        // Generate code for each element of the tuple
        for (i, elem) in tuple_type.elems.iter().enumerate() {
            let index = proc_macro2::Literal::usize_unsuffixed(i);

            // Check if element is a generic parameter
            let mut is_generic = false;
            for generic_type in generic_types {
                if is_generic_parameter(elem, generic_type) {
                    is_generic = true;
                    element_handling.push(quote! {
                        fields.push(format!("{}__{}",  #field_name, #index));
                    });
                    break;
                }
            }

            // If not a generic parameter, try to handle recursively
            if !is_generic {
                element_handling.push(quote! {
                    if let Some(inner_fields) = <#elem as StructReflectionHelper>::struct_reflection() {
                        for inner_field in inner_fields {
                            fields.push(format!("{}__{}__{}",  #field_name, #index, inner_field));
                        }
                    } else {
                        fields.push(format!("{}__{}",  #field_name, #index));
                    }
                });
            }
        }

        return Some(quote! {
            #(#element_handling)*
        });
    }
    None
}

// CORRECTED FUNCTION: Handle array types properly
fn handle_array_type(
    ty: &syn::Type,
    field_name: &str,
    generic_types: &[&syn::TypeParam],
) -> Option<proc_macro2::TokenStream> {
    if let syn::Type::Array(array_type) = ty {
        let elem_type = &*array_type.elem;
        let array_len = &array_type.len;

        // Case 1: Array of tuples like [(T, U); N]
        if is_tuple_type(elem_type) {
            if let syn::Type::Tuple(tuple_type) = elem_type {
                let tuple_size = tuple_type.elems.len();

                return Some(quote! {
                    for i in 0..#array_len {
                        for j in 0..#tuple_size {
                            fields.push(format!("{}__{}__{}",  #field_name, i, j));
                        }
                    }
                });
            }
        }

        // Case 2: Nested arrays with tuples like [[(T, U); M]; N]
        if let syn::Type::Array(inner_array_type) = elem_type {
            let inner_elem_type = &*inner_array_type.elem;
            let inner_array_len = &inner_array_type.len;

            if is_tuple_type(inner_elem_type) {
                if let syn::Type::Tuple(tuple_type) = inner_elem_type {
                    let tuple_size = tuple_type.elems.len();

                    return Some(quote! {
                        for i in 0..#array_len {
                            for j in 0..#inner_array_len {
                                for k in 0..#tuple_size {
                                    fields.push(format!("{}__{}__{}__{}", #field_name, i, j, k));
                                }
                            }
                        }
                    });
                }
            }
        }

        // Case 3: Check if array element is primitive type
        if is_primitive_type(elem_type) {
            return Some(quote! {
                for i in 0..#array_len {
                    fields.push(format!("{}__{}", #field_name, i));
                }
            });
        }

        // Case 4: Array of generic type [T; N]
        for generic_type in generic_types {
            if is_generic_parameter(elem_type, generic_type) {
                return Some(quote! {
                    for i in 0..#array_len {
                        fields.push(format!("{}__{}", #field_name, i));
                    }
                });
            }
        }

        // Case 5: Nested array [[ElementType; M]; N]
        if let syn::Type::Array(inner_array_type) = elem_type {
            let inner_elem_type = &*inner_array_type.elem;
            let inner_array_len = &inner_array_type.len;

            // Case 5.1: If inner element is primitive
            if is_primitive_type(inner_elem_type) {
                return Some(quote! {
                    for i in 0..#array_len {
                        for j in 0..#inner_array_len {
                            fields.push(format!("{}__{}__{}", #field_name, i, j));
                        }
                    }
                });
            }

            // Case 5.2: If inner element is generic
            for generic_type in generic_types {
                if is_generic_parameter(inner_elem_type, generic_type) {
                    return Some(quote! {
                        for i in 0..#array_len {
                            for j in 0..#inner_array_len {
                                fields.push(format!("{}__{}__{}", #field_name, i, j));
                            }
                        }
                    });
                }
            }
        }

        // Default case for arrays: Try to get nested fields or use basic indexing
        return Some(quote! {
            if let Some(sub_fields) = <#elem_type as StructReflectionHelper>::struct_reflection() {
                for i in 0..#array_len {
                    for sub_field in &sub_fields {
                        fields.push(format!("{}__{}__{}", #field_name, i, sub_field));
                    }
                }
            } else {
                for i in 0..#array_len {
                    fields.push(format!("{}__{}", #field_name, i));
                }
            }
        });
    }
    None
}
