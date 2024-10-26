use bevy::{core_pipeline::Skybox, prelude::*};
use std::f32::consts::PI;

pub struct StagePlugin;

impl Plugin for StagePlugin
{
    fn build(&self, app: &mut App)
    {   
        app.insert_resource(AmbientLight
        {
            color: Color::default(),
            brightness: 0.75,
        });
        app.add_systems(Startup, init_stage);
    }
}

fn init_stage(mut commands: Commands, asset_server: Res<AssetServer>)
{
    // Eventually load stages from files, probably in response to some kind of "stage load" event (this would happen in the update schedule)
    commands.spawn(
    DirectionalLightBundle
    {
        directional_light: DirectionalLight
        {
            illuminance: 32000.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 2.0, 0.0)
            .with_rotation(Quat::from_rotation_x(-PI / 4.)),
            ..default()
    });

    //let handle = asset_server.load("textures/skybox/skybox_10_stacked.png");
}