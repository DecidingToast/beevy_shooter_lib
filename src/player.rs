use avian3d::prelude::*;
use bevy::{color::palettes::css, prelude::*};
use bevy_tnua::prelude::*;
use bevy_tnua_avian3d::*;

pub struct PlayerPlugin;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Player
{
    mov_spd: f32,
}

impl Plugin for PlayerPlugin
{
    fn build(&self, app: &mut App)
    {
        app.register_type::<Player>()
           .add_systems(Startup, init_player)
           .add_systems(Update, apply_controls.in_set(TnuaUserControlsSystemSet));
    }
}

fn init_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
)
{

    let player = commands.spawn((
        Name::new("PlayerEntity"),
        PbrBundle {
            mesh: meshes.add(Cylinder {
                radius: 0.5,
                half_height: 1.0,
                //half_length: 0.5,
            }),
            material: materials.add(Color::from(css::DARK_CYAN)),
            transform: Transform::from_xyz(0.0, 2.0, 0.0),
            ..Default::default()
        },
        // The player character needs to be configured as a dynamic rigid body of the physics
        // engine.
        RigidBody::Dynamic,
        Collider::cylinder(0.5, 1.0),
        // This bundle holds the main components.
        TnuaControllerBundle::default(),
        // A sensor shape is not strictly necessary, but without it we'll get weird results.
        TnuaAvian3dSensorShape(Collider::cylinder(0.49, 0.0)),
        // Tnua can fix the rotation, but the character will still get rotated before it can do so.
        // By locking the rotation we can prevent this.
        LockedAxes::ROTATION_LOCKED,
    ))
    .id();
    
    let camera = commands.spawn(Camera3dBundle
    {
        transform: Transform::from_xyz(0.0, 4.0, 6.0)
        .looking_at(Vec3::new(0.0, 2.0, 0.0), Vec3::Y),
        ..Default::default()
    })
    .id();

    commands.entity(player).add_child(camera);
    // NOTE: if this was Rapier, we'd also need `TnuaRapier3dIOBundle`. Avian does not need it.

}

fn apply_controls(keyboard: Res<ButtonInput<KeyCode>>, mut query: Query<&mut TnuaController>)
{
    let Ok(mut controller) = query.get_single_mut() else {
        return;
    };

    let mut direction = Vec3::ZERO;

    if keyboard.pressed(KeyCode::KeyW) {
        direction -= Vec3::Z;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        direction += Vec3::Z;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        direction -= Vec3::X;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        direction += Vec3::X;
    }

    // Feed the basis every frame. Even if the player doesn't move - just use `desired_velocity:
    // Vec3::ZERO`. `TnuaController` starts without a basis, which will make the character collider
    // just fall.
    controller.basis(TnuaBuiltinWalk {
        // The `desired_velocity` determines how the character will move.
        desired_velocity: direction.normalize_or_zero() * 10.0,
        // The `float_height` must be greater (even if by little) from the distance between the
        // character's center and the lowest point of its collider.
        float_height: 1.5,
        // `TnuaBuiltinWalk` has many other fields for customizing the movement - but they have
        // sensible defaults. Refer to the `TnuaBuiltinWalk`'s documentation to learn what they do.
        ..Default::default()
    });

    // Feed the jump action every frame as long as the player holds the jump button. If the player
    // stops holding the jump button, simply stop feeding the action.
    if keyboard.pressed(KeyCode::Space) {
        controller.action(TnuaBuiltinJump {
            // The height is the only mandatory field of the jump button.
            height: 4.0,
            // `TnuaBuiltinJump` also has customization fields with sensible defaults.
            ..Default::default()
        });
    }
}