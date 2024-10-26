use bevy::prelude::*;

#[derive(Event, Debug)]
pub struct WeaponFiredEvent
{

}

#[derive(Event, Debug)]
pub struct WeapoonReloadedEvent
{

}

#[derive(Event, Debug)]
pub struct WeaponSwitchedEvent
{
    prev_weapon: Entity,
    next_weapon: Entity,
}

/// States used to control weapon handling functionality
#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum WeaponHandlingState
{
    /// Weapon is raised and can be operated.
    Raised,
    /// Player is aiming down sights, and can be operated.
    Aiming,
    /// Weapon is lowered (e.g. for cutscene, safe area, or after idling for some period of time).
    /// The weapon cannot be operated in this state.
    Lowered,
    /// The player is in the process of holstering their weapon.
    /// The weapon cannot be operated in this state.
    Holstering,
    /// The weapon is holstered.
    /// The weapon cannot be operated in this state.
    Holstered,
    /// The player is in the proces of drawing their weapon.
    /// The weapon cannot be operated in this state.
    Unholstering,
    /// The player is cycling weapons.
    /// The weapon cannot be operated in this state.
    Switching,
    /// The player is sprinting.
    /// The weapon cannot be operated in this state.
    Sprinting,
}

/// States used to control the actual operation of weapons
#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum WeaponOperationState
{
    /// The player is not currently operating their weapon.
    Idle,
    /// The player is currently firing the weapon.
    /// (this may be unnecessary, as "WeaponFired" event triggers may handle this more effectively)
    Firing,
    /// The player is reloading their weapon.
    Reloading,
    /// The player is venting heat from their weapon.
    Venting,
    /// The player is charging up their weapon.
    Charging,
    /// The player is performing a press-check on their weapon (after idling for long enough).
    PressCheck,
    /// The player is manually operating the weapon's action (as with a pump or bolt action weapon).
    OperatingAction,
    /// The player is performing a melee attack.
    Melee,
    /// The player is switching between fire modes (e.g. semi->auto).
    FireSelection,
}