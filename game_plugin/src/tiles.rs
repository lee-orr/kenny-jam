use std::ops::{Add, AddAssign};

use crate::actions::{self, Actions, PlayerMovement, TileRotation};
use crate::loading::TextureAssets;
use crate::player::Player;
use crate::GameState;
use bevy::prelude::*;
use rand::prelude::*;

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(spawn_backdrop.system())
                .with_system(spawn_tiles.system()),
        );
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(set_tile_rotation.system())
                .with_system(position_tile.system())
                .with_system(display_tile_rotation.system()),
        );
    }
}

pub struct TilePosition {
    pub x: i32,
    pub y: i32,
}

impl Add for TilePosition {
    type Output = TilePosition;

    fn add(self, rhs: Self) -> Self::Output {
        TilePosition {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

pub struct Tile {
    pub tile_type: TileType,
    pub tile_rotation: Rotation,
}

#[derive(Clone, Debug)]
pub enum Rotation {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Clone, Debug)]
pub enum TileType {
    Obstacle,
    DeadEnd,
    Line,
    Corner,
    TIntersection,
    Intersection,
}

impl Tile {
    pub fn calculate_rotation(&self) -> f32 {
        match self.tile_rotation {
            Rotation::Up => 0.,
            Rotation::Down => 180.,
            Rotation::Left => 90.,
            Rotation::Right => 270.,
        }
    }

    pub fn rotate_left(&mut self) {
        self.tile_rotation = match self.tile_rotation {
            Rotation::Up => Rotation::Left,
            Rotation::Down => Rotation::Right,
            Rotation::Left => Rotation::Down,
            Rotation::Right => Rotation::Up,
        }
    }

    pub fn rotate_right(&mut self) {
        self.tile_rotation = match self.tile_rotation {
            Rotation::Up => Rotation::Right,
            Rotation::Down => Rotation::Left,
            Rotation::Left => Rotation::Up,
            Rotation::Right => Rotation::Down,
        }
    }

    pub fn check_collision(&self, from: PlayerMovement) -> bool {
        bevy::log::info!(
            "Checking collision from {:?} with rotation {:?} and type {:?}",
            from,
            self.tile_rotation,
            self.tile_type
        );
        match self.tile_type {
            TileType::Obstacle => true,
            TileType::DeadEnd => {
                !(match self.tile_rotation {
                    Rotation::Up => from == PlayerMovement::Down,
                    Rotation::Left => from == PlayerMovement::Right,
                    Rotation::Down => from == PlayerMovement::Up,
                    Rotation::Right => from == PlayerMovement::Left,
                })
            }
            TileType::Line => {
                !(match self.tile_rotation {
                    Rotation::Up => from == PlayerMovement::Up || from == PlayerMovement::Down,
                    Rotation::Down => from == PlayerMovement::Up || from == PlayerMovement::Down,
                    Rotation::Left => from == PlayerMovement::Left || from == PlayerMovement::Right,
                    Rotation::Right => {
                        from == PlayerMovement::Left || from == PlayerMovement::Right
                    }
                })
            }
            TileType::Corner => {
                !(match self.tile_rotation {
                    Rotation::Up => from == PlayerMovement::Down || from == PlayerMovement::Left,
                    Rotation::Left => from == PlayerMovement::Down || from == PlayerMovement::Right,
                    Rotation::Down => from == PlayerMovement::Up || from == PlayerMovement::Right,
                    Rotation::Right => from == PlayerMovement::Up || from == PlayerMovement::Left,
                })
            }
            TileType::TIntersection => {
                !(match self.tile_rotation {
                    Rotation::Up => {
                        from == PlayerMovement::Down
                            || from == PlayerMovement::Left
                            || from == PlayerMovement::Right
                    }
                    Rotation::Left => {
                        from == PlayerMovement::Down
                            || from == PlayerMovement::Up
                            || from == PlayerMovement::Right
                    }
                    Rotation::Down => {
                        from == PlayerMovement::Up
                            || from == PlayerMovement::Left
                            || from == PlayerMovement::Right
                    }
                    Rotation::Right => {
                        from == PlayerMovement::Down
                            || from == PlayerMovement::Left
                            || from == PlayerMovement::Up
                    }
                })
            }
            TileType::Intersection => false,
        }
    }

    pub fn cant_leave(&self, from: PlayerMovement) -> bool {
        bevy::log::info!(
            "Checking ability to leave towards {:?} with rotation {:?} and type {:?}",
            from,
            self.tile_rotation,
            self.tile_type
        );
        self.check_collision(match from {
            PlayerMovement::Up => PlayerMovement::Down,
            PlayerMovement::Down => PlayerMovement::Up,
            PlayerMovement::Left => PlayerMovement::Right,
            PlayerMovement::Right => PlayerMovement::Left,
        })
    }
}

fn spawn_backdrop(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let backdrops = [
        materials.add(textures.grass_1.clone().into()),
        materials.add(textures.grass_2.clone().into()),
        materials.add(textures.mud_1.clone().into()),
        materials.add(textures.mud_2.clone().into()),
        materials.add(textures.stone_1.clone().into()),
        materials.add(textures.stone_2.clone().into()),
    ];
    let mut rng = rand::thread_rng();
    (-20..20).for_each(|x| {
        (-20..20).for_each(|y| {
            commands.spawn_bundle(SpriteBundle {
                material: if let Some(t) = backdrops.choose(&mut rng) {
                    t.clone().into()
                } else {
                    backdrops[0].clone().into()
                },
                transform: Transform::from_translation(Vec3::new(
                    (x as f32) * 64.0,
                    (y as f32) * 64.0,
                    0.,
                )),
                ..Default::default()
            });
        });
    });
}

fn spawn_tiles(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let dead_end = materials.add(textures.dead_end.clone().into());
    let line_out = materials.add(textures.line_out.clone().into());
    let corner = materials.add(textures.corner.clone().into());
    let t_intersection = materials.add(textures.t_intersection.clone().into());
    let intersection = materials.add(textures.intersection.clone().into());
    let obstacles = [
        materials.add(textures.obstacle_stone_1.clone().into()),
        materials.add(textures.obstacle_stone_2.clone().into()),
        materials.add(textures.obstacle_stone_3.clone().into()),
        materials.add(textures.obstacle_stone_4.clone().into()),
        materials.add(textures.obstacle_stone_5.clone().into()),
        materials.add(textures.obstacle_stone_6.clone().into()),
    ];
    let types = [
        TileType::DeadEnd,
        TileType::Corner,
        TileType::Intersection,
        TileType::Line,
        TileType::Obstacle,
        TileType::TIntersection,
    ];
    let rotations = [
        Rotation::Up,
        Rotation::Down,
        Rotation::Left,
        Rotation::Right,
    ];
    let mut rng = rand::thread_rng();
    (-20..20).for_each(|x| {
        (-20..20).for_each(|y| {
            let tile_type = types.choose(&mut rng).unwrap();
            let tile_rotation = rotations.choose(&mut rng).unwrap();
            commands
                .spawn_bundle(SpriteBundle {
                    material: match tile_type {
                        TileType::DeadEnd => dead_end.clone().into(),
                        TileType::Corner => corner.clone().into(),
                        TileType::Intersection => intersection.clone().into(),
                        TileType::Line => line_out.clone().into(),
                        TileType::TIntersection => t_intersection.clone().into(),
                        TileType::Obstacle => {
                            if let Some(t) = obstacles.choose(&mut rng) {
                                t.clone().into()
                            } else {
                                obstacles[0].clone().into()
                            }
                        }
                    },
                    transform: Transform::from_translation(Vec3::new(
                        (x as f32) * 64.0,
                        (y as f32) * 64.0,
                        0.5,
                    )),
                    ..Default::default()
                })
                .insert(Tile {
                    tile_type: tile_type.clone(),
                    tile_rotation: tile_rotation.clone(),
                })
                .insert(TilePosition {
                    x: x as i32,
                    y: y as i32,
                });
        });
    });
}

fn position_tile(mut query: Query<(&mut Transform, &TilePosition), (Changed<TilePosition>)>) {
    for (mut transform, tile) in query.iter_mut() {
        transform.translation = Vec3::new(
            (tile.x as f32) * 64.,
            (tile.y as f32) * 64.,
            transform.translation.z,
        );
    }
}

fn display_tile_rotation(mut query: Query<(&mut Transform, &mut Tile), (Changed<Tile>)>) {
    for (mut transform, tile) in query.iter_mut() {
        let rotation = tile.calculate_rotation();
        transform.rotation = Quat::from_rotation_z(rotation.to_radians());
    }
}

fn set_tile_rotation(
    mut query: (
        Query<(&mut Tile, &TilePosition)>,
        Query<&TilePosition, With<Player>>,
    ),
    actions: Res<Actions>,
) {
    if actions.tile_rotation.is_none() {
        return;
    }
    let rotation = actions.tile_rotation.as_ref().unwrap();
    let mut tiles = query.0;
    let players = query.1;
    for player in players.iter() {
        for (mut tile, transform) in tiles.iter_mut() {
            if player.x == transform.x && player.y == transform.y {
                match rotation {
                    TileRotation::Left => tile.rotate_left(),
                    TileRotation::Right => tile.rotate_right(),
                };
            }
        }
    }
}
