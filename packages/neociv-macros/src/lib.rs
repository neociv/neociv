extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(StateTable)]
pub fn state_table_derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl<'lua> ToLua<'lua> for #ident {
            fn to_lua(self, ctx: rlua::Context<'lua>) -> rlua::Result<rlua::Value<'lua>> {

            }
        }

        impl<'lua> FromLua<'lua> for #ident {
            fn from_lua(value: rlua::Value<'lua>, _lua: rlua::Context<'lua>) -> rlua::Result<Self> {
                
            }
        }
    };
    output.into()
}
