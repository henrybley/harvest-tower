mod stone;
mod tree;
mod ui;

use bevy::window::PrimaryWindow;
use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_inspector_egui::InspectorOptions;
use stone::StonePlugin;
use tree::TreePlugin;
use ui::GameUI;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Reaper Game".into(),
                        resolution: (640.0, 480.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_plugins(WorldInspectorPlugin::new())
        .insert_resource(PlayerResources { wood: 0, stone: 0 })
        .register_type::<Player>()
        .add_plugins((TreePlugin, StonePlugin, GameUI))
        .add_systems(Startup, setup)
        .add_systems(Update, (character_movement, mouse_button_input))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 1024.0,
        min_height: 576.0,
    };

    commands.spawn(camera);

    let texture = asset_server.load("0_Reaper_Man_Idle_000.png");

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(100.0, 100.0)),
                ..default()
            },
            texture,
            ..default()
        },
        Player { speed: 200.0 },
        Name::new("Player"),
    ));
}

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct Player {
    #[inspector(min = 0.0)]
    pub speed: f32,
}

fn character_movement(
    mut characters: Query<(&mut Transform, &Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, player) in &mut characters {
        let movement_amount = player.speed * time.delta_seconds();
        if input.pressed(KeyCode::W) {
            transform.translation.y += movement_amount;
        }
        if input.pressed(KeyCode::S) {
            transform.translation.y -= movement_amount;
        }
        if input.pressed(KeyCode::D) {
            transform.translation.x += movement_amount;
        }
        if input.pressed(KeyCode::A) {
            transform.translation.x -= movement_amount;
        }
    }
}

fn mouse_button_input(
    buttons: Res<Input<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        // Left button was pressed
        info!("Left Mouse button Was clicked");
        if let Some(position) = q_windows.single().cursor_position() {
            println!("Cursor is inside the primary window, at {:?}", position);
        } 
    }
    if buttons.pressed(MouseButton::Right) {
        // Right Button is being held down
        info!("Right Mouse button Was clicked");
        if let Some(position) = q_windows.single().cursor_position() {
            println!("Cursor is inside the primary window, at {:?}", position);
        }
    }
}

#[derive(Resource)]
pub struct PlayerResources {
    wood: u32,
    stone: u32,
}
