use bevy::prelude::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_systems(Update, render_gizmos);
    }
}

fn render_gizmos(mut commands: Commands)
{
    return;
}