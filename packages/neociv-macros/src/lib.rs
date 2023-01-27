#[macro_use]
extern crate quote;
extern crate proc_macro2;

mod utils;

use self::utils::*;

#[proc_macro_derive(StateTable)]
pub fn state_table_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let syn::DeriveInput { ident, data, .. } = syn::parse_macro_input!(input);

    if let syn::Data::Struct(ds) = data {
        let fields = get_fields(ds.fields);

        let to_lua_body: Vec<proc_macro2::TokenStream> = fields
            .iter()
            .map(|fld| {
                let flid = syn::Ident::new(fld, proc_macro2::Span::call_site());
                return quote! {
                    tbl.set(#fld, self.#flid)?;
                };
            })
            .collect();

        let from_lua_body: Vec<proc_macro2::TokenStream> = fields
            .iter()
            .map(|fld| {
                let flid = syn::Ident::new(fld, proc_macro2::Span::call_site());
                return quote! {
                    #flid: tbl.get(#fld)?,
                };
            })
            .collect();

        // Generate the implementations
        quote! {
            impl<'lua> rlua::ToLua<'lua> for #ident {
                fn to_lua(self, ctx: rlua::Context<'lua>) -> rlua::Result<rlua::Value<'lua>> {
                    let tbl = ctx.create_table()?;
                    #(#to_lua_body)*
                    Ok(rlua::Value::Table(tbl))
                }
            }

            impl<'lua> rlua::FromLua<'lua> for #ident {
                fn from_lua(value: rlua::Value<'lua>, _lua: rlua::Context<'lua>) -> rlua::Result<Self> {
                    match value {
                        rlua::Value::Table(tbl) => Ok(#ident {
                            #(#from_lua_body)*
                        }),
                        _ => Err(rlua::Error::FromLuaConversionError {
                                from: value.type_name(),
                                to: stringify!(#ident),
                                message: None,
                        }),
                    }
                }
            }
        }.into()
    } else {
        unreachable!()
        //compile_error!("StateTable can only be run on structs")
    }

    //return output.into();
}

/// Turn an enum into a namespaced string with an optional prefix
#[proc_macro_derive(StateEnum)]
pub fn state_enum_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let syn::DeriveInput { ident, data, .. } = syn::parse_macro_input!(input);
    if let syn::Data::Enum(enm) = data {
        let fields = get_enum_fields(enm.variants);

        let as_str_body: Vec<proc_macro2::TokenStream> = fields
            .iter()
            .map(|f| {
                let flid = syn::Ident::new(f, proc_macro2::Span::call_site());
                let nsid = to_namespace(f.to_string(), None);
                quote! {
                    Self::#flid => #nsid,
                }
            })
            .collect();

        let from_str_body: Vec<proc_macro2::TokenStream> = fields
            .iter()
            .map(|f| {
                let flid = syn::Ident::new(f, proc_macro2::Span::call_site());
                let nsid = to_namespace(f.to_string(), None);
                quote! {
                    #nsid => Ok(Self::#flid),
                }
            })
            .collect();

        let from_lua_body: Vec<proc_macro2::TokenStream> = fields
            .iter()
            .map(|f| {
                let flid = syn::Ident::new(f, proc_macro2::Span::call_site());
                let nsid = to_namespace(f.to_string(), None);
                quote! {
                    #nsid => Ok(Self::#flid),
                }
            })
            .collect();

        quote! {
            impl #ident {
                fn as_str(&self) -> &'static str {
                    match self {
                        #(#as_str_body)*
                    }
                }
            }

            impl ToString for #ident {
                fn to_string(&self) -> String {
                    self.as_str().into()
                }
            }

            impl std::str::FromStr for #ident {
                type Err = std::fmt::Error;
                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    match s {
                        #(#from_str_body)*
                        _ => Err(std::fmt::Error),
                    }
                }
            }

            impl<'lua> rlua::ToLua<'lua> for #ident {
                fn to_lua(self, ctx: rlua::Context<'lua>) -> rlua::Result<rlua::Value<'lua>> {
                    Ok(rlua::Value::String(ctx.create_string(self.as_str())?))
                }
            }

            impl<'lua> rlua::FromLua<'lua> for #ident {
                fn from_lua(value: rlua::Value<'lua>, _lua: rlua::Context<'lua>) -> rlua::Result<Self> {
                    let tn = value.type_name();
                    match value {
                        rlua::Value::String(s) => match s.to_str().unwrap() {
                            #(#from_lua_body)*
                            _ => Err(rlua::Error::FromLuaConversionError {
                                from: tn,
                                to: stringify!(#ident),
                                message: None,
                            })
                        },
                        _ => Err(rlua::Error::FromLuaConversionError {
                            from: tn,
                            to: stringify!(#ident),
                            message: None,
                        }),
                    }
                }
            }
        }.into()
    } else {
        unreachable!()
    }
}
