use bevy::prelude::*;

use super::WeaponFiredEvent;

/// Defines recoil characteristics. All rotational parameters are specified in degrees. All positional parameters are specified in game units (treated as meters).
#[derive(Component, Default, Reflect, Debug, Clone)]
#[reflect(Component, Default)]
pub struct Recoil
{
    /// Whether this recoil component should be applied when not aiming down sights (generally true for weapons and false for camera armatures)
    apply_recoil_when_not_aiming: bool,

    // Direct backwards translation, primarily for weapons
    kickback: f32,
    // The total kickback will not exceed this amount
    kickback_limit: f32,

    // The amount of pitch per shot will not exceed this number
    pitch: f32,
    // The total pitch will not exceed this amount
    pitch_limit: f32,

    // The amount of roll per shot will be between these numbers (negative roll is leftward, positive is right)
    // The first number MUST be less than or equal the the second
    roll: (f32, f32),
    // The max amount of roll will not exceed the corresponding limit
    // The first number MUST be less than or equal the the second
    roll_limit: (f32, f32),

    // The amount of yaw per shot will be between these numbers (negative roll is leftward, positive is right)
    // The first number MUST be less than or equal the the second
    yaw: (f32, f32),
    // The max amount of yaw will not exceed the corresponding limit
    // The first number MUST be less than or equal the the second
    yaw_limit: (f32, f32),

    return_speed: f32,
    snappiness: f32,

    // rotations are stored as euler coordinates in ZXY order
    target_rotation: Vec3,
    current_rotation: Vec3,
}

pub fn handle_weapon_recoil(
    time: Res<Time>,
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(&mut Recoil, &mut Transform)>,
    mut weapon_fired_event_reader: EventReader<WeaponFiredEvent>,
)
{
    for (mut r, mut t) in &mut query
    {
        let event_iterator = weapon_fired_event_reader.read();
        if event_iterator.peekable().peek().is_some()
        {
            // Apply recoil. Do we need to do this per shot? Not if fire rate is below 1800 (at 30 fps) rpm, let's set that as the hard cap for fire rate
            // for shot in weapon_fired_event_reader.read()
            // {
                
            // }
            //r.target_rotation += Vec3()
        }
        else // Return to center
        {
            r.target_rotation = r.target_rotation.lerp( Vec3::ZERO, r.return_speed * time.delta_seconds());
            
            // The vendored Vec3 library lacks slerp, so we must use the bevy-provided Dir3 type instead
            r.current_rotation = Dir3::slerp( 
                r.current_rotation.try_into().unwrap(),
                r.target_rotation.try_into().unwrap(),
                r.snappiness * fixed_time.delta_seconds())
                .try_into().unwrap();

            t.rotate_local(Quat::from_euler(
                EulerRot::ZXY,
                f32::to_radians(r.current_rotation.x),
                f32::to_radians(r.current_rotation.y),
                f32::to_radians(r.current_rotation.z)));
        }
    }
}

// #endregion