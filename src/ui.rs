use bevy::prelude::*;

use crate::PlayerResources;

pub struct GameUI;

#[derive(Component)]
pub struct WoodText;

#[derive(Component)]
pub struct StoneText;

impl Plugin for GameUI {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_game_ui)
            .add_systems(Update, (update_wood_ui, update_stone_ui));
    }
}

fn spawn_game_ui(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(10.0),
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: Color::INDIGO.into(),
                ..default()
            },
            Name::new("UI Root"),
        ))
        .with_children(|commands| {
            commands.spawn((
                TextBundle {
                    text: Text::from_section(
                        "Wood!",
                        TextStyle {
                            font_size: 32.0,
                            ..default()
                        },
                    ),
                    ..default()
                },
                WoodText,
            ));
        }).with_children(|commands| {
            commands.spawn((
                TextBundle {
                    text: Text::from_section(
                        "Stone!",
                        TextStyle {
                            font_size: 32.0,
                            ..default()
                        },
                    ),
                    ..default()
                },
                StoneText,
            ));
        });
}


fn update_wood_ui(
    mut wood_text: Query<&mut Text, With<WoodText>>,
    player_resouces: Res<PlayerResources>
) {
    for mut text in &mut wood_text {
        text.sections[0].value = format!("Wood: {:?}", player_resouces.wood);
    }
}
fn update_stone_ui(
    mut stone_text: Query<&mut Text, With<StoneText>>,
    player_resouces: Res<PlayerResources>
) {
    for mut text in &mut stone_text {
        text.sections[0].value = format!("Stone: {:?}", player_resouces.stone);
    }
}
