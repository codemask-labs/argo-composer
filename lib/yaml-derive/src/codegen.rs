use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields};

use crate::utils::{
    extract_doc_comments, extract_option_inner_type, extract_rename_attribute, is_complex_type,
    is_option_type, is_vec_type,
};

/// Generate the implementation code for the Deserialize trait
pub fn generate_deserialize_impl(input: &DeriveInput) -> TokenStream {
    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let fields_code = generate_fields_deserialization(&input.data);

    quote! {
        impl #impl_generics YamlDeserializer for #name #ty_generics #where_clause {
            fn from_yaml_nodes(nodes: &[yaml_ast::YamlNode]) -> Result<Self, String> {
                #fields_code
            }
        }
    }
}

/// Generate deserialization code for struct fields
fn generate_fields_deserialization(data: &Data) -> TokenStream {
    let Data::Struct(data_struct) = data else {
        return quote! {
            Err("Only structs are supported for deserialization".to_string())
        };
    };

    let Fields::Named(fields) = &data_struct.fields else {
        return quote! {
            Err("Only named fields are supported for deserialization".to_string())
        };
    };

    let field_extractions = fields
        .named
        .iter()
        .map(|field| generate_field_deserialization(field));

    quote! {
        use ::yaml::deserializer::{extract_object_pairs, find_field_value, FromYamlValue};

        let pairs = extract_object_pairs(nodes)?;

        Ok(Self {
            #(#field_extractions)*
        })
    }
}

/// Generate deserialization code for a single field
fn generate_field_deserialization(field: &syn::Field) -> TokenStream {
    let field_name = &field.ident;
    let default_field_name_str = field_name.as_ref().unwrap().to_string();
    let field_type = &field.ty;

    // Check for rename attribute
    let field_name_str = extract_rename_attribute(&field.attrs).unwrap_or(default_field_name_str);

    let is_option = is_option_type(field_type);
    let is_vec = is_vec_type(field_type);

    if is_option {
        // For Option<T>, extract T and check if it's complex
        let inner_type = extract_option_inner_type(field_type)
            .expect("Failed to extract inner type from Option");
        let is_complex_inner = is_complex_type(inner_type);

        if is_complex_inner {
            // Option<ComplexType> - use YamlDeserializer
            quote! {
                #field_name: find_field_value(&pairs, #field_name_str)
                    .and_then(|v| {
                        let node = yaml_ast::YamlNode {
                            value: v.clone(),
                            inline_comment: None,
                            leading_comment: None,
                        };
                        <#inner_type as ::yaml::deserializer::YamlDeserializer>::from_yaml_nodes(&[node]).ok()
                    }),
            }
        } else {
            // Option<PrimitiveType> - use FromYamlValue
            quote! {
                #field_name: find_field_value(&pairs, #field_name_str)
                    .and_then(|v| <#field_type as FromYamlValue>::from_yaml_value(v).ok())
                    .flatten(),
            }
        }
    } else if is_vec {
        // Vec field - default to empty if missing (since we skip empty Vecs during serialization)
        quote! {
            #field_name: find_field_value(&pairs, #field_name_str)
                .map(|v| {
                    let node = yaml_ast::YamlNode {
                        value: v.clone(),
                        inline_comment: None,
                        leading_comment: None,
                    };
                    <#field_type as ::yaml::deserializer::YamlDeserializer>::from_yaml_nodes(&[node])
                        .unwrap_or_default()
                })
                .unwrap_or_default(),
        }
    } else {
        // Required field - check if it's a YamlDeserializer type or FromYamlValue type
        let is_complex = is_complex_type(field_type);

        if is_complex {
            // Complex type that implements YamlDeserializer (nested objects, Vecs, etc.)
            quote! {
                #field_name: find_field_value(&pairs, #field_name_str)
                    .ok_or_else(|| format!("Missing required field: {}", #field_name_str))
                    .and_then(|v| {
                        // Create a YamlNode for complex types
                        let node = yaml_ast::YamlNode {
                            value: v.clone(),
                            inline_comment: None,
                            leading_comment: None,
                        };
                        <#field_type as ::yaml::deserializer::YamlDeserializer>::from_yaml_nodes(&[node])
                    })?,
            }
        } else {
            // Simple type that implements FromYamlValue (String, bool, i64, etc.)
            quote! {
                #field_name: find_field_value(&pairs, #field_name_str)
                    .ok_or_else(|| format!("Missing required field: {}", #field_name_str))
                    .and_then(|v| <#field_type as FromYamlValue>::from_yaml_value(v))?,
            }
        }
    }
}

/// Generate the implementation code for the Serialize trait
pub fn generate_serialize_impl(input: &DeriveInput) -> TokenStream {
    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let fields_code = generate_fields_serialization(&input.data);

    quote! {
        impl #impl_generics YamlSerializer for #name #ty_generics #where_clause {
            fn to_yaml_nodes(&self) -> Vec<yaml_ast::YamlNode> {
                #fields_code
            }
        }
    }
}

/// Generate serialization code for struct fields
fn generate_fields_serialization(data: &Data) -> TokenStream {
    let Data::Struct(data_struct) = data else {
        return quote! { vec![] };
    };

    let Fields::Named(fields) = &data_struct.fields else {
        return quote! { vec![] };
    };

    let field_conversions = fields
        .named
        .iter()
        .map(|field| generate_field_serialization(field));

    quote! {
        // Build a vector that will hold comment nodes and field pairs in order
        let mut result_nodes = Vec::new();
        let mut pairs = Vec::new();

        #(#field_conversions)*

        // Wrap all pairs in a single Object node
        result_nodes.push(yaml_ast::YamlNode {
            value: yaml_ast::YamlValue::Object(pairs),
            inline_comment: None,
            leading_comment: None,
        });

        result_nodes
    }
}

/// Generate serialization code for a single field
fn generate_field_serialization(field: &syn::Field) -> TokenStream {
    let field_name = &field.ident;
    let default_field_name_str = field_name.as_ref().unwrap().to_string();
    let field_type = &field.ty;

    // Check for rename attribute
    let field_name_str = extract_rename_attribute(&field.attrs).unwrap_or(default_field_name_str);

    let doc_comment = extract_doc_comments(&field.attrs);
    let is_option = is_option_type(field_type);
    let is_vec = is_vec_type(field_type);
    let is_string = is_string_like_type(field_type);

    // Handle Vec types specially - skip if empty
    if is_vec && !is_option {
        return if let Some(comment) = doc_comment {
            generate_vec_with_comment(field_name, &field_name_str, &comment)
        } else {
            generate_vec_without_comment(field_name, &field_name_str)
        };
    }

    match (is_option, is_string, doc_comment) {
        (true, true, Some(comment)) => {
            generate_optional_string_with_comment(field_name, &field_name_str, &comment)
        }
        (true, true, None) => generate_optional_string_without_comment(field_name, &field_name_str),
        (true, false, Some(comment)) => {
            generate_optional_nested_with_comment(field_name, &field_name_str, &comment)
        }
        (true, false, None) => {
            generate_optional_nested_without_comment(field_name, &field_name_str)
        }
        (false, true, Some(comment)) => {
            generate_required_string_with_comment(field_name, &field_name_str, &comment)
        }
        (false, true, None) => {
            generate_required_string_without_comment(field_name, &field_name_str)
        }
        (false, false, Some(comment)) => {
            generate_required_nested_with_comment(field_name, &field_name_str, &comment)
        }
        (false, false, None) => {
            generate_required_nested_without_comment(field_name, &field_name_str)
        }
    }
}

/// Check if type is String-like (inside Option or not)
fn is_string_like_type(ty: &syn::Type) -> bool {
    use crate::utils::is_string_type;

    // Check if it's directly a String
    if is_string_type(ty) {
        return true;
    }

    // Check if it's Option<String>
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Option" {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first() {
                        return is_string_type(inner_ty);
                    }
                }
            }
        }
    }

    false
}

// String field generators
fn generate_optional_string_with_comment(
    field_name: &Option<syn::Ident>,
    field_name_str: &str,
    comment_text: &str,
) -> TokenStream {
    quote! {
        if let Some(ref value) = self.#field_name {
            let value_node = yaml_ast::YamlNode {
                value: yaml_ast::YamlValue::String(value.to_string()),
                inline_comment: None,
                leading_comment: Some(#comment_text.to_string()),
            };
            pairs.push((#field_name_str.to_string(), value_node));
        }
    }
}

fn generate_optional_string_without_comment(
    field_name: &Option<syn::Ident>,
    field_name_str: &str,
) -> TokenStream {
    quote! {
        if let Some(ref value) = self.#field_name {
            let value_node = yaml_ast::YamlNode {
                value: yaml_ast::YamlValue::String(value.to_string()),
                inline_comment: None,
                leading_comment: None,
            };
            pairs.push((#field_name_str.to_string(), value_node));
        }
    }
}

fn generate_required_string_with_comment(
    field_name: &Option<syn::Ident>,
    field_name_str: &str,
    comment_text: &str,
) -> TokenStream {
    quote! {
        {
            let value_node = yaml_ast::YamlNode {
                value: yaml_ast::YamlValue::String(self.#field_name.to_string()),
                inline_comment: None,
                leading_comment: Some(#comment_text.to_string()),
            };
            pairs.push((#field_name_str.to_string(), value_node));
        }
    }
}

fn generate_required_string_without_comment(
    field_name: &Option<syn::Ident>,
    field_name_str: &str,
) -> TokenStream {
    quote! {
        {
            let value_node = yaml_ast::YamlNode {
                value: yaml_ast::YamlValue::String(self.#field_name.to_string()),
                inline_comment: None,
                leading_comment: None,
            };
            pairs.push((#field_name_str.to_string(), value_node));
        }
    }
}

// Nested object/collection field generators
fn generate_optional_nested_with_comment(
    field_name: &Option<syn::Ident>,
    field_name_str: &str,
    comment_text: &str,
) -> TokenStream {
    quote! {
        if let Some(ref value) = self.#field_name {
            let nested_nodes = value.to_yaml_nodes();
            // Use the first non-comment node (could be Object or Collection)
            let mut value_node = nested_nodes.iter()
                .find(|node| !matches!(node.value, yaml_ast::YamlValue::Comment(_)))
                .cloned()
                .unwrap_or_else(|| yaml_ast::YamlNode {
                    value: yaml_ast::YamlValue::Object(vec![]),
                    inline_comment: None,
                    leading_comment: None,
                });

            value_node.leading_comment = Some(#comment_text.to_string());
            pairs.push((#field_name_str.to_string(), value_node));
        }
    }
}

fn generate_optional_nested_without_comment(
    field_name: &Option<syn::Ident>,
    field_name_str: &str,
) -> TokenStream {
    quote! {
        if let Some(ref value) = self.#field_name {
            let nested_nodes = value.to_yaml_nodes();
            // Use the first non-comment node (could be Object or Collection)
            let value_node = nested_nodes.iter()
                .find(|node| !matches!(node.value, yaml_ast::YamlValue::Comment(_)))
                .cloned()
                .unwrap_or_else(|| yaml_ast::YamlNode {
                    value: yaml_ast::YamlValue::Object(vec![]),
                    inline_comment: None,
                    leading_comment: None,
                });

            pairs.push((#field_name_str.to_string(), value_node));
        }
    }
}

fn generate_required_nested_with_comment(
    field_name: &Option<syn::Ident>,
    field_name_str: &str,
    comment_text: &str,
) -> TokenStream {
    quote! {
        {
            let nested_nodes = self.#field_name.to_yaml_nodes();
            // Use the first non-comment node (could be Object or Collection)
            let mut value_node = nested_nodes.iter()
                .find(|node| !matches!(node.value, yaml_ast::YamlValue::Comment(_)))
                .cloned()
                .unwrap_or_else(|| yaml_ast::YamlNode {
                    value: yaml_ast::YamlValue::Object(vec![]),
                    inline_comment: None,
                    leading_comment: None,
                });

            value_node.leading_comment = Some(#comment_text.to_string());
            pairs.push((#field_name_str.to_string(), value_node));
        }
    }
}

fn generate_required_nested_without_comment(
    field_name: &Option<syn::Ident>,
    field_name_str: &str,
) -> TokenStream {
    quote! {
        {
            let nested_nodes = self.#field_name.to_yaml_nodes();
            // Use the first non-comment node (could be Object or Collection)
            let value_node = nested_nodes.iter()
                .find(|node| !matches!(node.value, yaml_ast::YamlValue::Comment(_)))
                .cloned()
                .unwrap_or_else(|| yaml_ast::YamlNode {
                    value: yaml_ast::YamlValue::Object(vec![]),
                    inline_comment: None,
                    leading_comment: None,
                });

            pairs.push((#field_name_str.to_string(), value_node));
        }
    }
}

// Vec field generators - skip if empty
fn generate_vec_with_comment(
    field_name: &Option<syn::Ident>,
    field_name_str: &str,
    comment_text: &str,
) -> TokenStream {
    quote! {
        if !self.#field_name.is_empty() {
            let nested_nodes = self.#field_name.to_yaml_nodes();
            let mut value_node = nested_nodes.iter()
                .find(|node| !matches!(node.value, yaml_ast::YamlValue::Comment(_)))
                .cloned()
                .unwrap_or_else(|| yaml_ast::YamlNode {
                    value: yaml_ast::YamlValue::Collection(vec![]),
                    inline_comment: None,
                    leading_comment: None,
                });

            value_node.leading_comment = Some(#comment_text.to_string());
            pairs.push((#field_name_str.to_string(), value_node));
        }
    }
}

fn generate_vec_without_comment(
    field_name: &Option<syn::Ident>,
    field_name_str: &str,
) -> TokenStream {
    quote! {
        if !self.#field_name.is_empty() {
            let nested_nodes = self.#field_name.to_yaml_nodes();
            let value_node = nested_nodes.iter()
                .find(|node| !matches!(node.value, yaml_ast::YamlValue::Comment(_)))
                .cloned()
                .unwrap_or_else(|| yaml_ast::YamlNode {
                    value: yaml_ast::YamlValue::Collection(vec![]),
                    inline_comment: None,
                    leading_comment: None,
                });

            pairs.push((#field_name_str.to_string(), value_node));
        }
    }
}
