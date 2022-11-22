use std::{error::Error, str::FromStr};

const CIV_UNIT_BLOCK: &str = "civ.unit.block";
const CIV_UNIT_UNLOCK: &str = "civ.unit.unlock";
const CIV_BUILDING_BLOCK: &str = "civ.building.block";
const CIV_BUILDING_UNLOCK: &str = "civ.building.unlock";
const CITY_UNIT_BLOCK: &str = "city.unit.block";
const CITY_UNIT_UNLOCK: &str = "city.unit.unlock";
const CITY_BUILDING_BLOCK: &str = "city.building.block";
const CITY_BUILDING_UNLOCK: &str = "city.building.unlock";

/// Meta actions modify details about Civs or Cities and their buildings, units, or other details that can
/// be described atomically. Typically this takes the form of unlocking content via tree nodes but can be run at any time.
/// Most importantly, these are non-*functional* in the sense that they are purely descriptive actions with no
/// other arbitrary logic attached to them.
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

impl MetaActionType {
    fn as_str(&self) -> &'static str {
        match self {
            Self::CivUnitBlock => CIV_UNIT_BLOCK,
            Self::CivUnitUnlock => CIV_UNIT_UNLOCK,
            Self::CivBuildingBlock => CIV_BUILDING_BLOCK,
            Self::CivBuildingUnlock => CIV_BUILDING_UNLOCK,
            Self::CityUnitBlock => CITY_UNIT_BLOCK,
            Self::CityUnitUnlock => CITY_UNIT_UNLOCK,
            Self::CityBuildingBlock => CITY_BUILDING_BLOCK,
            Self::CityBuildingUnlock => CITY_BUILDING_UNLOCK,
        }
    }
}

impl ToString for MetaActionType {
    fn to_string(&self) -> String {
        self.as_str().into()
    }
}

impl FromStr for MetaActionType {
    type Err = std::fmt::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            CIV_UNIT_BLOCK => Ok(Self::CivUnitBlock),
            CIV_UNIT_UNLOCK => Ok(Self::CivUnitUnlock),
            CIV_BUILDING_BLOCK => Ok(Self::CivBuildingBlock),
            CIV_BUILDING_UNLOCK => Ok(Self::CivBuildingUnlock),
            CITY_UNIT_BLOCK => Ok(Self::CityUnitBlock),
            CITY_UNIT_UNLOCK => Ok(Self::CityUnitUnlock),
            CITY_BUILDING_BLOCK => Ok(Self::CityBuildingBlock),
            CITY_BUILDING_UNLOCK => Ok(Self::CityBuildingUnlock),
            _ => Err(std::fmt::Error),
        }
    }
}

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
