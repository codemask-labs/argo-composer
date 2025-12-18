use syn::{Attribute, Type};

/// Extract the rename attribute value from field attributes
/// Looks for #[yaml(rename = "fieldName")]
pub fn extract_rename_attribute(attrs: &[Attribute]) -> Option<String> {
    for attr in attrs {
        // Check if this is a #[yaml(...)] attribute
        if !attr.path().is_ident("yaml") {
            continue;
        }

        // Parse the nested meta items inside yaml(...)
        let nested = attr
            .parse_args_with(
                syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
            )
            .ok()?;

        for meta in nested {
            // Look for rename = "value"
            if let syn::Meta::NameValue(name_value) = meta {
                if name_value.path.is_ident("rename") {
                    // Extract the string literal value
                    if let syn::Expr::Lit(expr_lit) = &name_value.value {
                        if let syn::Lit::Str(lit_str) = &expr_lit.lit {
                            return Some(lit_str.value());
                        }
                    }
                }
            }
        }
    }

    None
}

/// Check if a type is Option<T>
pub fn is_option_type(ty: &Type) -> bool {
    let Type::Path(type_path) = ty else {
        return false;
    };

    let Some(segment) = type_path.path.segments.last() else {
        return false;
    };

    segment.ident == "Option"
}

/// Extract the inner type from Option<T>
pub fn extract_option_inner_type(ty: &Type) -> Option<&Type> {
    let Type::Path(type_path) = ty else {
        return None;
    };

    let segment = type_path.path.segments.last()?;

    if segment.ident != "Option" {
        return None;
    }

    let syn::PathArguments::AngleBracketed(args) = &segment.arguments else {
        return None;
    };

    let syn::GenericArgument::Type(inner_type) = args.args.first()? else {
        return None;
    };

    Some(inner_type)
}

/// Check if a type is String or a primitive type with Display
pub fn is_string_type(ty: &Type) -> bool {
    let Type::Path(type_path) = ty else {
        return false;
    };

    let Some(segment) = type_path.path.segments.last() else {
        return false;
    };

    matches!(
        segment.ident.to_string().as_str(),
        "String"
            | "str"
            | "i32"
            | "i64"
            | "u32"
            | "u64"
            | "f32"
            | "f64"
            | "bool"
            | "usize"
            | "isize"
    )
}

/// Check if a type is a complex type that needs YamlDeserializer (Vec, HashMap, custom structs)
/// Returns true for Vec, BTreeMap, HashMap, and any type not in is_string_type
pub fn is_complex_type(ty: &Type) -> bool {
    let Type::Path(type_path) = ty else {
        return true; // Unknown types are assumed complex
    };

    let Some(segment) = type_path.path.segments.last() else {
        return true;
    };

    let type_name = segment.ident.to_string();

    // These types use YamlDeserializer
    if matches!(type_name.as_str(), "Vec" | "BTreeMap" | "HashMap") {
        return true;
    }

    // Primitive types use FromYamlValue, complex types use YamlDeserializer
    !is_string_type(ty)
}

/// Check if a type is Vec<T>
pub fn is_vec_type(ty: &Type) -> bool {
    let Type::Path(type_path) = ty else {
        return false;
    };

    let Some(segment) = type_path.path.segments.last() else {
        return false;
    };

    segment.ident == "Vec"
}

/// Check if a type is BTreeMap<K, V> or HashMap<K, V>
pub fn is_map_type(ty: &Type) -> bool {
    let Type::Path(type_path) = ty else {
        return false;
    };

    let Some(segment) = type_path.path.segments.last() else {
        return false;
    };

    matches!(segment.ident.to_string().as_str(), "BTreeMap" | "HashMap")
}

/// Extract doc comments from attributes
pub fn extract_doc_comments(attrs: &[Attribute]) -> Option<String> {
    let mut comments = Vec::new();

    for attr in attrs {
        if !attr.path().is_ident("doc") {
            continue;
        }

        let Ok(meta) = attr.meta.require_name_value() else {
            continue;
        };

        let syn::Expr::Lit(expr_lit) = &meta.value else {
            continue;
        };

        let syn::Lit::Str(lit_str) = &expr_lit.lit else {
            continue;
        };

        let comment = lit_str.value().trim().to_string();

        if comment.is_empty() {
            continue;
        }

        comments.push(comment);
    }

    if comments.is_empty() {
        return None;
    }

    Some(comments.join("\n"))
}
