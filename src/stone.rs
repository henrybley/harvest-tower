use bevy::prelude::*;
use rand::Rng;

use crate::PlayerResources;
pub struct StonePlugin;

impl Plugin for StonePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(RockSpawnConfig {timer: Timer::from_seconds(5.0, TimerMode::Repeating)})
            .add_systems(Startup, spawn_rock_mine)
            .add_systems(Update, (spawn_rocks, rock_lifetime))
            .register_type::<Rock>();
    }
}

#[derive(Component)]
struct RockMine;

fn spawn_rock_mine(mut commands: Commands) {
    commands.spawn((SpatialBundle::default(), RockMine, Name::new("RockMine")));
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
struct Rock {
    pub lifetime: Timer,
}

#[derive(Resource)]
struct RockSpawnConfig {
    timer: Timer,
}

fn spawn_rocks(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut config: ResMut<RockSpawnConfig>,
    rock_mine: Query<Entity, With<RockMine>>,
) {
    config.timer.tick(time.delta());

    if config.timer.finished() {
        let texture = asset_server.load("Rock1_1.png");
        let rock_mine = rock_mine.single();

        let mut rng = rand::thread_rng();
        let x: f32 = rng.gen_range(1.0..=500.0);
        let y: f32 = rng.gen_range(1.0..=500.0);

        info!("spawning tree at {:?}, {:?}", x, y);

        commands.entity(rock_mine).with_children(|commands| {
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(x, y, 0.0),
                    texture,
                    ..default()
                },
                Rock {
                    lifetime: Timer::from_seconds(2.0, TimerMode::Once),
                },
                Name::new("Rock"),
            ));
        });

    }
}


fn rock_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut rocks: Query<(Entity, &mut Rock)>,
    rock_mine: Query<Entity, With<RockMine>>,
    mut player_resources: ResMut<PlayerResources>,
) {
    for(rock_entity, mut rock) in &mut rocks {
        rock.lifetime.tick(time.delta());
        let rock_mine = rock_mine.single();

        if rock.lifetime.finished() {
            player_resources.stone += 10;
            commands.entity(rock_mine).remove_children(&[rock_entity]);
            commands.entity(rock_entity).despawn();

            info!("Rock Mined for 10 stone! current Stone: {:?}", player_resources.stone);
        }
    }
}
