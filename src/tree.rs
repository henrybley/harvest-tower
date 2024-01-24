use bevy::prelude::*;
use rand::Rng;

use crate::PlayerResources;

pub struct TreePlugin;

impl Plugin for TreePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(TreeSpawnConfig {timer : Timer::from_seconds(4.0, TimerMode::Repeating) })
            .add_systems(Startup, spawn_forest)
            .add_systems(Update, (spawn_trees, tree_lifetime))
            .register_type::<Tree>();
    }
}

#[derive(Component)]
pub struct Forest;

fn spawn_forest(mut commands: Commands) {
    commands.spawn((SpatialBundle::default(), Forest, Name::new("Forest")));
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Tree {
    pub lifetime: Timer,
}

#[derive(Resource)]
struct TreeSpawnConfig {
    timer: Timer,
}

fn spawn_trees(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut config: ResMut<TreeSpawnConfig>,
    forest: Query<Entity, With<Forest>>,
) {
    config.timer.tick(time.delta());

    if config.timer.finished() {

        let texture = asset_server.load("Moss_tree2.png");
        let forest = forest.single();

        let mut rng = rand::thread_rng();
        let x: f32 = rng.gen_range(1.0..=500.0);
        let y: f32 = rng.gen_range(1.0..=500.0);


        info!("spawning tree at {:?}, {:?}", x, y);

        commands.entity(forest).with_children(|commands| {
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(x, y, 0.0),
                    texture,
                    ..default()
                },
                Tree {
                    lifetime: Timer::from_seconds(2.0, TimerMode::Once),
                },
                Name::new("Tree"),
            ));
        });
    }
}

fn tree_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut trees: Query<(Entity, &mut Tree)>,
    forest: Query<Entity, With<Forest>>,
    mut player_resources: ResMut<PlayerResources>,
) {
    for(tree_entity, mut tree) in &mut trees {
        tree.lifetime.tick(time.delta());
        let forest = forest.single();

        if tree.lifetime.finished() {
            player_resources.wood += 15;
            commands.entity(forest).remove_children(&[tree_entity]);
            commands.entity(tree_entity).despawn();

            info!("Tree Chopped down for 15 wood! current Wood: {:?}", player_resources.wood);
        }
    }
}
