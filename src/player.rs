use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn bulid(&self, _app: &mut App) {
        add_startup_system(spawn_player);}
}




#[derive(Component)]
struct Player;

fn spawn_player(
    mut commands: Commands,
    mut textture_atlas: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>
) {
    let atlas = TextureAtlas::from_grid(
        asset_server.load("Characters/character_0001.png"),
        Vec2::splat(24.),
        11, 1, None, None);
    commands.spawn((SpriteSheetBundle {
        texture_atlas: textture_atlas.add(atlas),
        sprite: TextureAtlasSprite { index: 0, ..Default::default() },
        ..Default::default()
    }, Player)); 
}