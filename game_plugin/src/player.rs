use crate::actions::{Actions, PlayerMovement};
use crate::loading::TextureAssets;
use crate::tiles::{Tile, TilePosition};
use crate::GameState;
use bevy::prelude::*;

pub struct PlayerPlugin;

pub struct Player;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(spawn_player.system())
                .with_system(spawn_camera.system()),
        )
        .add_system_set(SystemSet::on_update(GameState::Playing).with_system(move_player.system()));
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_player(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(textures.robot_cheer.clone().into()),
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            sprite: Sprite {
                size: Vec2::new(48., 64.),
                resize_mode: SpriteResizeMode::Manual,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player)
        .insert(TilePosition { x: 0, y: 0 });
}

fn move_player(
    actions: Res<Actions>,
    mut query: (
        Query<&mut TilePosition, With<Player>>,
        Query<(&Tile, &TilePosition), Without<Player>>,
    ),
) {
    if actions.player_movement.is_none() {
        return;
    }

    let direction = actions.player_movement.unwrap();

    let movement = match direction {
        PlayerMovement::Up => TilePosition { x: 0, y: 1 },
        PlayerMovement::Down => TilePosition { x: 0, y: -1 },
        PlayerMovement::Left => TilePosition { y: 0, x: -1 },
        PlayerMovement::Right => TilePosition { y: 0, x: 1 },
    };
    let mut player_query = query.0;
    let tile_query = query.1;
    for mut player_transform in player_query.iter_mut() {
        let newPos = TilePosition {
            x: player_transform.x + movement.x,
            y: player_transform.y + movement.y,
        };

        bevy::log::info!(
            "Player moving from {} {} to {} {}",
            player_transform.x,
            player_transform.y,
            newPos.x,
            newPos.y
        );

        let mut collision = false;

        for (tile, position) in tile_query.iter() {
            if position.x == newPos.x && position.y == newPos.y {
                if tile.check_collision(direction) {
                    collision = true;
                }
            } else if position.x == player_transform.x && position.y == player_transform.y {
                if tile.cant_leave(direction) {
                    collision = true;
                }
            }
        }

        if !collision {
            player_transform.x = newPos.x;
            player_transform.y = newPos.y;
        }
    }
}
