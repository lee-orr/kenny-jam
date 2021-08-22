use crate::GameState;
use bevy::prelude::*;

pub struct ActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<Actions>().add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(set_movement_actions.system()),
        );
    }
}

#[derive(Default)]
pub struct Actions {
    pub player_movement: Option<PlayerMovement>,
    pub tile_rotation: Option<TileRotation>,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum PlayerMovement {
    Up,
    Down,
    Left,
    Right,
}

pub enum TileRotation {
    Left,
    Right,
}

fn set_movement_actions(mut actions: ResMut<Actions>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::W) || keyboard_input.just_pressed(KeyCode::Up) {
        actions.player_movement = Some(PlayerMovement::Up);
    } else if keyboard_input.just_pressed(KeyCode::S) || keyboard_input.just_pressed(KeyCode::Down)
    {
        actions.player_movement = Some(PlayerMovement::Down);
    } else if keyboard_input.just_pressed(KeyCode::A) || keyboard_input.just_pressed(KeyCode::Left)
    {
        actions.player_movement = Some(PlayerMovement::Left);
    } else if keyboard_input.just_pressed(KeyCode::D) || keyboard_input.just_pressed(KeyCode::Right)
    {
        actions.player_movement = Some(PlayerMovement::Right);
    } else {
        actions.player_movement = None;
    }

    if keyboard_input.just_pressed(KeyCode::Q) || keyboard_input.just_pressed(KeyCode::Numpad4) {
        actions.tile_rotation = Some(TileRotation::Left);
    } else if keyboard_input.just_pressed(KeyCode::E)
        || keyboard_input.just_pressed(KeyCode::Numpad6)
    {
        actions.tile_rotation = Some(TileRotation::Right);
    } else {
        actions.tile_rotation = None;
    }
}
