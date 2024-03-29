pub mod building;
pub mod unit;

#[macro_export]
macro_rules! neociv_desc {
    ($kind: ident { $($props:tt)* }) => {
        paste::paste! {
            #[derive(
                Clone,
                Debug,
                neociv_macros::StateTable,
                serde::Serialize,
                serde::Deserialize,
                bevy_ecs::component::Component,
                bevy_ecs::system::Resource
            )]
            pub struct [<Neociv $kind DescProps>] {
                $($props)*
            }

            #[derive(
                Clone,
                Debug,
                neociv_macros::StateTable,
                serde::Serialize,
                serde::Deserialize,
                bevy_ecs::component::Component,
                bevy_ecs::system::Resource
            )]
            pub struct [<Neociv $kind Desc>] {
                id: String,
                props: [<Neociv $kind DescProps>],
                resources: crate::desc::NeocivDescResources,
            }

            impl [<Neociv $kind Desc>] {
                pub fn new(id: String, props: [<Neociv $kind DescProps>], resources: crate::desc::NeocivDescResources) -> Self {
                    Self {
                        id,
                        props,
                        resources,
                    }
                }
            }

            impl crate::desc::NeocivDesc<[<Neociv $kind DescProps>]> for [<Neociv $kind Desc>] {
                fn id(&self) -> String {
                    self.id.to_owned()
                }

                fn kind(&self) -> crate::desc::NeocivDescKind {
                    crate::desc::NeocivDescKind::$kind
                }

                fn props(&self) -> [<Neociv $kind DescProps>] {
                    self.props.to_owned()
                }

                fn resources(&self) -> crate::desc::NeocivDescResources {
                    self.resources.to_owned()
                }
            }
        }
    };
}

pub(crate) use neociv_desc;

use self::{building::NeocivBuildingDesc, unit::NeocivUnitDesc};

pub enum NeocivDescKind {
    Building,
    Unit,
}

/**
 * Define the actual media / resources content of an entity.
 */
#[derive(
    Clone, Debug, Default, serde::Serialize, serde::Deserialize, neociv_macros::StateTable,
)]
pub struct NeocivDescResources {
    mesh: String,
    materials: Vec<String>,
}

pub trait NeocivDesc<T: ?Sized> {
    fn id(&self) -> String;
    fn kind(&self) -> NeocivDescKind;
    fn props(&self) -> T;
    fn resources(&self) -> NeocivDescResources;
}

pub enum NeocivEntityDesc {
    Building(NeocivBuildingDesc),
    Unit(NeocivUnitDesc),
}
