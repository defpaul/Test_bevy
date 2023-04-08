use bevy::{prelude::*, transform::commands};

pub const CLEAR:Color = Color::rgb(0.1, 0.1, 0.1);

mod player;
use player::PlayerPlugin;

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
    .add_plugin(PlayerPlugin)
    .insert_resource(ClearColor(CLEAR))
    .add_startup_system(spawn_cam)
//    .add_system(hello_world)
    .run()
}


fn hello_world() {
    println!("hallo world");
}

fn spawn_cam(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}