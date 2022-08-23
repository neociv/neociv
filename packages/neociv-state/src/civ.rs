use regex::*;
use rlua::{ToLua, Value as LuaValue};
use serde::{Deserialize, Serialize};

use crate::alignments::Alignments;

/// CivIds are namespaced strings delimited by "." and act as an identifier whenever a Civ needs to
/// be referenced from another context. This should be the only means of identifying a Civ.
///
/// # Examples
/// org.neociv.civs.aus
/// expanse.belters.ilus
/// mod.civs.example
pub type CivId = String;

/// CivKeys are identical to CivIds except that they have an index attached to them. In practice
/// this will be indexed to the sub-index of pre-existing Civs with this CivId. A game with all
/// unique Civs will have keys that all end in "[0]" as they are all the first entry for the given
/// CivId.
///
/// # Examples
/// org.neociv.civs[0]
/// expanse.belters[0]
/// mod.civs.example[0]
pub type CivKey = String;

lazy_static! {
    /// Regex to validate CivIds
    pub static ref VALID_CIV_ID: Regex = Regex::new(r"^[a-zA-Z0-9]+\.[a-zA-Z0-9]+(?:\.[a-zA-Z0-9])*$").unwrap();
}

lazy_static! {
    /// Regex to validate CivKeys
    pub static ref VALID_CIV_KEY: Regex = Regex::new(r"^[a-zA-Z0-9]+\.[a-zA-Z0-9]+(?:\.[a-zA-Z0-9])*\[\d+\]$").unwrap();
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Civ {
    pub id: CivId,
    pub title: String,
    pub alignments: Alignments,
}

impl<'lua> ToLua<'lua> for Civ {
    fn to_lua(self, ctx: rlua::Context<'lua>) -> rlua::Result<rlua::Value<'lua>> {
        let civ_tbl = ctx.create_table()?;
        civ_tbl.set("id", self.id)?;
        civ_tbl.set("title", self.title)?;
        let aligns_tbl = ctx.create_table_from(self.alignments)?;
        civ_tbl.set("alignments", aligns_tbl)?;
        Ok(LuaValue::Table(civ_tbl))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_valid_civ_id() {
        // Invalid CivIds
        assert!(!crate::civ::VALID_CIV_ID.is_match("example"));
        // Valid CivIds
        assert!(crate::civ::VALID_CIV_ID.is_match("example.com"));
        assert!(crate::civ::VALID_CIV_ID.is_match("1234.5678"));
    }

    #[test]
    fn test_valid_civ_key() {
        // Invalid CivIds (which are also not valid CivKeys)
        assert!(!crate::civ::VALID_CIV_KEY.is_match("example"));
        // Valid CivIds but invalid CivKeys
        assert!(!crate::civ::VALID_CIV_KEY.is_match("example.com"));
        // Invalid CivKeys
        assert!(!crate::civ::VALID_CIV_KEY.is_match("example[0]"));
        assert!(!crate::civ::VALID_CIV_KEY.is_match("example[]"));
        assert!(!crate::civ::VALID_CIV_KEY.is_match("example.com[]"));
    }
}
