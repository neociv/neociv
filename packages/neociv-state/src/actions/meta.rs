use crate::{state_enum, state_table};

state_enum! {
    /// Meta actions modify details about Civs or Cities and their buildings, units, or other details
    /// that can be described atomically. Typically this takes the form of unlocking content via tree
    /// nodes but can be run at any time. Most importantly, these are non-*functional* in the sense that
    /// they are purely descriptive actions with no other arbitrary logic attached to them.
    pub enum MetaActionType {
        /// Prevents and/or removes a unit from being available to a Civ
        CivUnitBlock,

        /// Allows a unit to be available to a Civ
        CivUnitUnlock,

        /// Prevents and/or removes a building from being available to a Civ
        CivBuildingBlock,

        /// Allows a building to be available to a Civ
        CivBuildingUnlock,

        /// Prevents and/or removes a unit from being available to a City
        CityUnitBlock,

        /// Allows a unit to be available to a City
        CityUnitUnlock,

        /// Prevents and/or removes a building from being available to a City
        CityBuildingBlock,

        /// Allows a building to be available to a City
        CityBuildingUnlock,
    }
}

state_table! {
    pub struct MetaAction {
        /// The action to be performed on the state
        action: MetaActionType,

        /// Civ or City that this action applies to
        target: String,

        /// The content descriptor string - this String will have a predicate attached to it most likely, for
        /// example a unit descriptor that may also match with a key in other tables to be able to look up
        /// icons, titles, or other associated data.
        content: String,
    }
}
