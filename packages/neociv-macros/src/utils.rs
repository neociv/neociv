pub fn get_fields(fields: syn::Fields) -> Vec<String> {
    match fields {
        syn::Fields::Named(named) => named
            .named
            .iter()
            .map(|fnm| fnm.ident.as_ref().unwrap().to_string())
            .collect(),
        _ => vec![],
    }
}

pub fn get_enum_fields(
    variants: syn::punctuated::Punctuated<syn::Variant, syn::token::Comma>,
) -> Vec<String> {
    variants.iter().map(|v| v.ident.to_string()).collect()
}

pub fn to_namespace(s: String, prefix: Option<String>) -> String {
    let re = regex::Regex::new(r"([A-Z][a-z]*)").expect("Unable to create regex pattern");
    let segs: Vec<String> = re
        .find_iter(s.as_str())
        .map(|m| m.as_str().to_ascii_lowercase())
        .collect();

    return segs.join(".");
}
