use bevy::prelude::*;
pub use recoil::*;
pub use controls::*;
pub use common::*;
#[cfg(feature = "editor")]
use space_editor::prelude::EditorRegistryExt;

pub mod recoil;
pub mod common;
pub mod controls;

pub struct WeaponHandlingPlugin;
//Component + Default + Send + 'static + GetTypeRegistration + Reflect + FromReflect
impl Plugin for WeaponHandlingPlugin
{
    #[cfg(feature = "editor")]
    fn build(&self, app: &mut App)
    {
        app.editor_registry::<Recoil>();
    }

    #[cfg(not(feature = "editor"))]
    fn build(&self, app: &mut App) {

        app.add_systems(Update, handle_weapon_recoil);
        app.add_systems(Update, read_controls);

        app.add_event::<WeaponFiredEvent>();
        app.add_event::<WeaponSwitchedEvent>();
        app.add_event::<WeapoonReloadedEvent>();
    }
}