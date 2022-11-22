#[macro_use]
extern crate synstructure;
#[macro_use]
extern crate quote;
extern crate proc_macro2;

/*
fn state_table_derive(s: synstructure::Structure) -> proc_macro2::TokenStream {
    let to_lua_body = s.each(|bi| {
        quote! {
            tbl.set(format!("{}", #bi), self.#bi)?;
        }
    });

    print!(">> {:?}", to_lua_body);

    let _from_lua_body = s.each(|_bi| {
        quote! {
            //tbl.get(format!("{}", #bi))?;
        }
    });

    s.gen_impl(quote! {
        gen impl<'lua> ToLua<'lua> for @Self {
            fn to_lua(self, ctx: rlua::Context<'lua>) -> rlua::Result<rlua::Value<'lua>> {
                let tbl = ctx.create_table()?;
                //#to_lua_body
                Ok(rlua::Value::Table(tbl))
            }
        }

        gen impl<'lua> FromLua<'lua> for @Self {
            fn from_lua(value: rlua::Value<'lua>, _ctx: rlua::Context<'lua>) -> rlua::Result<Self> {
                match value {
                    rlua::Value::Table(tbl) => Ok(@Self {
                        //#from_lua_body
                    }),
                    _ => Err(rlua::Error::FromLuaConversionError {
                        from: value.type_name(),
                        to: "@Self",
                        message: None,
                    })
                }
            }
        }
    })
}
synstructure::decl_derive!([StateTable] => state_table_derive);
*/

#[proc_macro_derive(StateTable)]
pub fn state_table_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let syn::DeriveInput { ident, data, .. } = syn::parse_macro_input!(input);
    if let syn::Data::Struct(ds) = data {
        /*
        if let syn::Fields::Named(named) = ds.fields {
            named.named.iter().for_each(|f| {
                let fnms = f.ident.as_ref().unwrap();
                /*
                let span = fnm.span();
                let fnms = &syn::Ident::new(
                    syn::LitStr::new(&fnm.to_string(), span).value().as_str(),
                    span,
                );
                dbg!(fnms);
                */
                let output = quote! {
                    #fnms = 1
                };
                dbg!(output);
                /*
                match &f.ident {
                    Some(id) => dbg!(id),
                    None => panic!("FUCKED"),
                };
                */
            });
        }
        */

        let fields: Vec<String> = match ds.fields {
            syn::Fields::Named(named) => named
                .named
                .iter()
                .map(|fnm| fnm.ident.as_ref().unwrap().to_string())
                .collect(),
            _ => vec![],
        };

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

        /*
        let mut fields_vec_innards = quote!();
        let fields_vec = match ds.fields {
            syn::Fields::Named(named) => {
                &named.named.iter().for_each(|f| {
                    dbg!(f.ident);
                })
            }
            _ => unimplemented!(),
        };
        */

        let output = quote! {
            impl<'lua> rlua::ToLua<'lua> for #ident {
                fn to_lua(self, ctx: rlua::Context<'lua>) -> rlua::Result<rlua::Value<'lua>> {
                    let tbl = ctx.create_table()?;
                    // TODO: Iterate over fields in the derive input
                    #(#to_lua_body)*
                    Ok(rlua::Value::Table(tbl))
                }
            }

            impl<'lua> rlua::FromLua<'lua> for #ident {
                fn from_lua(value: rlua::Value<'lua>, _lua: rlua::Context<'lua>) -> rlua::Result<Self> {
                    match value {
                       rlua::Value::Table(tbl) => Ok(#ident {
                        // TODO: Iterate over fields in the derive input
                        #(#from_lua_body)*
                       }),
                       _ => Err(rlua::Error::FromLuaConversionError {
                            from: value.type_name(),
                            to: "#ident",
                            message: None,
                       }),
                    }
                }
            }
        };
        output.into()
    } else {
        let output = quote! { impl Example for #ident {}  };
        output.into()
        //compile_error!("StateTable can only be run on structs")
    }

    //return output.into();
}
