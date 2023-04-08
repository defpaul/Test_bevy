use bevy::{prelude::*, transform::commands};

pub const CLEAR:Color = Color::rgb(0.1, 0.1, 0.1);

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "I am a window!".into(),
            resolution: (1600., 900.).into(),
            fit_canvas_to_parent: true,
            prevent_default_event_handling: false,
            ..default()
        }),
        ..default()
    }))
    .insert_resource(ClearColor(CLEAR))
    .add_startup_system(spawn_cam)
    .add_startup_system(spawn_player)
//    .add_system(hello_world)
    .run()
}


fn hello_world() {
    println!("hallo world");
}

fn spawn_cam(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
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
