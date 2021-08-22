use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};
use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

/// This plugin loads all assets using [AssetLoader] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at https://bevy-cheatbook.github.io/features/assets.html
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        AssetLoader::new(GameState::Loading, GameState::Menu)
            .with_collection::<FontAssets>()
            .with_collection::<AudioAssets>()
            .with_collection::<TextureAssets>()
            .build(app);
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see https://github.com/NiklasEi/bevy_asset_loader)

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct AudioAssets {
    #[asset(path = "audio/flying.ogg")]
    pub flying: Handle<AudioSource>,
}

#[derive(AssetCollection)]
pub struct TextureAssets {
    #[asset(path = "textures/Robot/character_robot_cheer0.png")]
    pub robot_cheer: Handle<Texture>,
    #[asset(path = "textures/MainTiles/Tile/medievalTile_15.png")]
    pub stone_1: Handle<Texture>,
    #[asset(path = "textures/MainTiles/Tile/medievalTile_16.png")]
    pub stone_2: Handle<Texture>,
    #[asset(path = "textures/MainTiles/Tile/medievalTile_13.png")]
    pub mud_1: Handle<Texture>,
    #[asset(path = "textures/MainTiles/Tile/medievalTile_14.png")]
    pub mud_2: Handle<Texture>,
    #[asset(path = "textures/MainTiles/Tile/medievalTile_57.png")]
    pub grass_1: Handle<Texture>,
    #[asset(path = "textures/MainTiles/Tile/medievalTile_58.png")]
    pub grass_2: Handle<Texture>,
    #[asset(path = "textures/MainTiles/Tile/medievalTile_39.png")]
    pub dead_end: Handle<Texture>,
    #[asset(path = "textures/MainTiles/Tile/medievalTile_08.png")]
    pub line_out: Handle<Texture>,
    #[asset(path = "textures/MainTiles/Tile/medievalTile_36.png")]
    pub corner: Handle<Texture>,
    #[asset(path = "textures/MainTiles/Tile/medievalTile_12.png")]
    pub t_intersection: Handle<Texture>,
    #[asset(path = "textures/MainTiles/Tile/medievalTile_10.png")]
    pub intersection: Handle<Texture>,
    #[asset(path = "textures/MainTiles/Environment/medievalEnvironment_07.png")]
    pub obstacle_stone_1: Handle<Texture>,
    #[asset(path = "textures/MainTiles/Environment/medievalEnvironment_09.png")]
    pub obstacle_stone_2: Handle<Texture>,
    #[asset(path = "textures/MainTiles/Environment/medievalEnvironment_10.png")]
    pub obstacle_stone_3: Handle<Texture>,
    #[asset(path = "textures/MainTiles/Environment/medievalEnvironment_16.png")]
    pub obstacle_stone_4: Handle<Texture>,
    #[asset(path = "textures/MainTiles/Environment/medievalEnvironment_17.png")]
    pub obstacle_stone_5: Handle<Texture>,
    #[asset(path = "textures/MainTiles/Environment/medievalEnvironment_19.png")]
    pub obstacle_stone_6: Handle<Texture>,
    #[asset(path = "textures/MainTiles/Environment/medievalEnvironment_02.png")]
    pub obstacle_tree_1: Handle<Texture>,
    #[asset(path = "textures/MainTiles/Environment/medievalEnvironment_04.png")]
    pub obstacle_tree_2: Handle<Texture>,
}
