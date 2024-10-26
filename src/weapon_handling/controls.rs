use bevy::prelude::*;

use super::common::*;

pub fn read_controls(
    mut commands: Commands,
    mut weapon_fired_event_writer: EventWriter<WeaponFiredEvent>,
    mut weapon_switched_event_writer: EventWriter<WeaponSwitchedEvent>,
    mut weapon_reloaded_event_writer: EventWriter<WeapoonReloadedEvent>,
)
{
    // TODO: implement kbm & controller input (possibly through input manager like leafwing or another)
}