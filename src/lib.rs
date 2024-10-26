use avian3d::PhysicsPlugins;
use bevy::{pbr::experimental::meshlet::MeshletPlugin, prelude::*};
use bevy_tnua::prelude::TnuaControllerPlugin;
use bevy_tnua_avian3d::TnuaAvian3dPlugin;
use player::PlayerPlugin;
use weapon_handling::WeaponHandlingPlugin;

mod weapon_handling;
mod player;

pub struct GamePlugin;

impl Plugin for GamePlugin
{
    #[allow(unused_variables)]
    fn build(&self, app: &mut App)
    {
        let plugins = (
            MeshletPlugin,
            PhysicsPlugins::default(),
            TnuaControllerPlugin::default(),
            TnuaAvian3dPlugin::default(),
            PlayerPlugin,
            WeaponHandlingPlugin,
        );
        app.add_plugins(plugins);
    }
}
