use std::time;

use bevy::{prelude::*, transform::commands, window::PrimaryWindow, input::keyboard::KeyboardInput};

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
    .add_startup_system(spawnPlayer)
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
pub struct Player;

pub fn spawnPlayer(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    AssetServer: Res<AssetServer>
) {
   let Window: &Window = window_query.get_single().unwrap();

   commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            texture: AssetServer.load("2d/Characters/character_0001.png"),
            ..default()
        },
        Player{},
   ));
}

pub fn player_mufment(
    KeyboardInput: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>
    ) {
        if let Ok(mut transform) = player_query.get_single_mut() {
            let mut direction = Vec::ZERO;

            if KeyboardInput.pressed(KeyCode::Left) || KeyboardInput.pressed(KeyCode::A) {
                direction += Vec3::new(-1.0, 0.0, 0.0);    
            }
            if KeyboardInput.pressed(KeyCode::Right) || KeyboardInput.pressed(KeyCode::D) {
                direction += Vec3::new(1.0, 0.0, 0.0);    
            }
            if KeyboardInput.pressed(KeyCode::Up) || KeyboardInput.pressed(KeyCode::W) {
                direction += Vec3::new(0.0, 1.0, 0.0);    
            }
            if KeyboardInput.pressed(KeyCode::Down) || KeyboardInput.pressed(KeyCode::S) {
                direction += Vec3::new(0.0, -1.0, 0.0);    
            }
        }
    }