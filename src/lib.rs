use proc_macro::TokenStream;
use quote::quote;
use syn::DataStruct;
use syn::DeriveInput;
use syn::Fields;
use syn::parse_macro_input;

#[proc_macro_derive(StringHolda)]
pub fn string_holda_derive(input: TokenStream) -> TokenStream {
    string_holder_derive_impl(input, true)
}

#[proc_macro_derive(Holda, attributes(holda))]
pub fn holda_derive(input: TokenStream) -> TokenStream {
    string_holder_derive_impl(input, false)
}

fn string_holder_derive_impl(input: TokenStream, is_string: bool) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    // Get the struct name
    let struct_name = &ast.ident;

    // Parse the attributes to determine which traits to skip
    let mut skip_display = false;
    let mut skip_eq = false;
    let mut skip_ord = false;
    let mut skip_hash = false;
    let mut skip_clone = false;
    let mut skip_serde = false;

    for attr in &ast.attrs {
        if attr.path().is_ident("holda") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("NoDisplay") {
                    skip_display = true;
                } else if meta.path.is_ident("NoEq") {
                    skip_eq = true;
                } else if meta.path.is_ident("NoOrd") {
                    skip_ord = true;
                } else if meta.path.is_ident("NoHash") {
                    skip_hash = true;
                } else if meta.path.is_ident("NoClone") {
                    skip_clone = true;
                } else if meta.path.is_ident("NoSerde") {
                    skip_serde = true;
                }
                Ok(())
            })
            .unwrap();
        }
    }

    // Get the field name (assuming it's a single field named "inner")
    let fields = if let syn::Data::Struct(DataStruct {
        fields: Fields::Named(named),
        ..
    }) = ast.data
    {
        named
    } else {
        panic!("Only structs with named fields are supported");
    };

    // Get the type of the "inner" field
    let inner_field = fields
        .named
        .first()
        .expect("Struct must have at least one field")
        .clone();
    let inner_ident = inner_field.ident.expect("Field must be named");
    let inner_type = inner_field.ty;

    let from_str_impl = if is_string {
        quote! {
            impl std::str::FromStr for #struct_name {
                type Err = eyre::Error;

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    let value = s.to_string().into();
                    Ok(Self { #inner_ident: value })
                }
            }
        }
    } else {
        quote! {}
    };

    let from_str_and_str_impl = if is_string {
        quote! {
            //Implement From<&str>
            impl From<&str> for #struct_name {
                fn from(value: &str) -> Self {
                    Self { #inner_ident: value.into() }
                }
            }
        }
    } else {
        quote! {}
    };

    let display_impl = if !skip_display {
        quote! {
            impl std::fmt::Display for #struct_name
            where #inner_type: std::fmt::Display
            {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}", self.#inner_ident)
                }
            }
        }
    } else {
        quote! {}
    };

    let debug_impl = quote! {
        impl std::fmt::Debug for #struct_name
        where #inner_type: std::fmt::Debug
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", self.#inner_ident)
            }
        }
    };

    let partial_eq_impl = if !skip_eq {
        quote! {
            impl PartialEq for #struct_name
            where #inner_type: PartialEq
            {
                fn eq(&self, other: &Self) -> bool {
                    self.#inner_ident == other.#inner_ident
                }
            }
        }
    } else {
        quote! {}
    };

    let eq_impl = if !skip_eq {
        quote! {
            impl Eq for #struct_name where #inner_type: Eq {}
        }
    } else {
        quote! {}
    };

    let partial_ord_impl = if !skip_ord {
        quote! {
            impl PartialOrd for #struct_name
            where #inner_type: PartialOrd
            {
                fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                    self.#inner_ident.partial_cmp(&other.#inner_ident)
                }
            }
        }
    } else {
        quote! {}
    };

    let ord_impl = if !skip_ord {
        quote! {
            impl Ord for #struct_name
            where #inner_type: Ord
            {
                fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                    self.#inner_ident.cmp(&other.#inner_ident)
                }
            }
        }
    } else {
        quote! {}
    };

    let hash_impl = if !skip_hash {
        quote! {
            impl std::hash::Hash for #struct_name
            where #inner_type: std::hash::Hash
            {
                fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                    self.#inner_ident.hash(state);
                }
            }
        }
    } else {
        quote! {}
    };

    let clone_impl = if !skip_clone {
        quote! {
            impl Clone for #struct_name
            where #inner_type: Clone
            {
                fn clone(&self) -> Self {
                    Self {
                        #inner_ident: self.#inner_ident.clone(),
                    }
                }
            }
        }
    } else {
        quote! {}
    };

    let serde_impl = if !skip_serde {
        quote! {
            #[cfg(feature = "serde")]
            impl serde::Serialize for #struct_name
            where #inner_type: serde::Serialize
            {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    self.#inner_ident.serialize(serializer)
                }
            }

            #[cfg(feature = "serde")]
            impl<'de> serde::Deserialize<'de> for #struct_name
            where #inner_type: serde::Deserialize<'de>
            {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    let value = <#inner_type as serde::Deserialize>::deserialize(deserializer)?;
                    Ok(Self { #inner_ident: value })
                }
            }
        }
    } else {
        quote! {}
    };

    // Generate all the trait implementations
    let expanded = quote! {
        impl #struct_name {
            pub fn new(value: impl Into<#inner_type>) -> Self {
                Self { #inner_ident: value.into() }
            }
        }

        impl AsRef<#inner_type> for #struct_name {
            fn as_ref(&self) -> &#inner_type {
                &self.#inner_ident
            }
        }

        // Implement From trait for other types
        impl From<#inner_type> for #struct_name {
            fn from(value: #inner_type) -> Self {
                Self { #inner_ident: value }
            }
        }

        #from_str_and_str_impl

        // Implement Into trait
        impl Into<#inner_type> for #struct_name {
            fn into(self) -> #inner_type {
                self.#inner_ident
            }
        }

        // Deref implementation
        impl std::ops::Deref for #struct_name {
            type Target = #inner_type;

            fn deref(&self) -> &Self::Target {
                &self.#inner_ident
            }
        }

        // DerefMut implementation
        impl std::ops::DerefMut for #struct_name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.#inner_ident
            }
        }

        #display_impl

        #debug_impl

        #from_str_impl

        #partial_eq_impl
        #eq_impl

        #partial_ord_impl
        #ord_impl

        #hash_impl

        #clone_impl

        #serde_impl
    };

    expanded.into()
}
